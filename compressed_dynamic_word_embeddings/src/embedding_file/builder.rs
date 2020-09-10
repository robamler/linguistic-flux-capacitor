use super::{FileHeader, HEADER_SIZE};
use crate::{
    ans::EntropyModel12_16,
    tensors::{RankThreeTensor, RankThreeTensorView},
    u12::pack_u12s,
};

use std::{collections::HashMap, convert::TryInto};

pub fn build_file(
    uncompressed: RankThreeTensorView<i16>,
    chunk_size: u32,
    scale_factor: f32,
) -> Vec<u32> {
    let (num_timesteps, vocab_size, embedding_dim) = uncompressed.shape();
    let num_timesteps: u32 = num_timesteps.try_into().unwrap();
    let vocab_size: u32 = vocab_size.try_into().unwrap();
    let embedding_dim: u32 = embedding_dim.try_into().unwrap();

    assert!(vocab_size > 0);
    assert!(embedding_dim > 0);
    assert!(chunk_size > 0);
    assert_eq!(vocab_size % chunk_size, 0);

    let chunks_per_timestep = vocab_size / chunk_size;

    let entropy_model_description_length = |num_symbols: u32| {
        let symbol_count_u16s = 1;
        let frequency_u16s = 3 * num_symbols / 4;
        let symbols_u16s = num_symbols;
        let total_u16s = symbol_count_u16s + frequency_u16s + symbols_u16s;

        (total_u16s + 1) / 2
    };

    assert!(num_timesteps >= 2);
    let (diffs, counts) = get_diffs(uncompressed);

    // TODO: estimate file size based on entropies and reserve it.
    let mut compressed = Vec::new();
    // Fill in header later, when we know the file size.
    compressed.resize(HEADER_SIZE as usize, 0);

    // Calculate and push time step addresses.
    let mut address = HEADER_SIZE + num_timesteps;
    let symbols_and_frequencies = counts
        .into_iter()
        .map(|counts| {
            let symbols_and_frequencies = optimal_frequencies_12bit(&counts);
            compressed.push(address);
            address += entropy_model_description_length(symbols_and_frequencies.len() as u32)
                + chunks_per_timestep;
            symbols_and_frequencies
        })
        .collect::<Vec<_>>();

    let meta_size = address;

    // Skip over time step meta data since we don't know the chunk addresses yet.
    compressed.resize(meta_size as usize, 0);

    // Write out compressed chunks in tree traversal order (might improve memory locality).
    let mut chunk_addresses =
        vec![Vec::with_capacity(chunks_per_timestep as usize); num_timesteps as usize];

    let mut write_timestep = |t: usize| {
        let symbols_and_frequencies = &symbols_and_frequencies[t];
        let encoder_model = EntropyModel12_16::new(
            symbols_and_frequencies.iter().map(|&(s, _)| s),
            symbols_and_frequencies.iter().map(|&(_, f)| f),
        )
        .encoder_model();

        let chunk_addresses = &mut chunk_addresses[t];
        for chunk in diffs
            .as_view()
            .subview(t)
            .slice()
            .chunks((chunk_size * embedding_dim) as usize)
        {
            let mut compressed_chunk = encoder_model.encode(chunk).unwrap();
            if compressed_chunk.len() % 2 == 1 {
                compressed_chunk.push(0);
            }

            chunk_addresses.push(compressed.len() as u32);
            compressed.reserve(compressed_chunk.len() / 2);
            let mut iter = compressed_chunk.iter();
            while let Some(first) = iter.next() {
                let second = iter.next().unwrap();
                // Little endian byte order: first bytes are least significant.
                compressed.push((*second as u32) << 16 | (*first as u32));
            }
        }
    };

    write_timestep(0);
    write_timestep((num_timesteps - 1) as usize);

    traverse_subtree(
        2,
        0,
        0,
        num_timesteps as usize - 1,
        1,
        &mut |t, _, _, _, _, _| write_timestep(t),
    );

    // Write out time step meta data.
    let mut address = HEADER_SIZE + num_timesteps;
    for (symbols_and_frequencies, chunk_addresses) in
        symbols_and_frequencies.into_iter().zip(chunk_addresses)
    {
        let frequencies = symbols_and_frequencies
            .iter()
            .map(|&(_, f)| f)
            .collect::<Vec<_>>();
        let mut u16_iter = std::iter::once(symbols_and_frequencies.len().try_into().unwrap())
            .chain(symbols_and_frequencies.iter().map(|&(s, _)| s as u16))
            .chain(pack_u12s(&frequencies[..frequencies.len() - 1]))
            .chain(std::iter::once(0));

        let description_length =
            entropy_model_description_length(symbols_and_frequencies.len() as u32);
        for dest in &mut compressed[address as usize..(address + description_length) as usize] {
            let first = u16_iter.next().unwrap();
            let second = u16_iter.next().unwrap();
            // Little endian byte order: first bytes are least significant.
            *dest = (second as u32) << 16 | first as u32;
        }

        address += description_length;
        compressed[address as usize..(address + chunks_per_timestep) as usize]
            .copy_from_slice(&chunk_addresses);
        address += chunks_per_timestep;
    }

    // Write file header.
    let file_header = FileHeader {
        magic: 0x6577_6400,
        major_version: 1,
        minor_version: 0,
        file_size: compressed.len() as u32,
        meta_size,
        num_timesteps,
        vocab_size,
        embedding_dim,
        chunk_size,
        scale_factor,
    };

    let header_array = unsafe {
        const HEADER_SIZE: usize = std::mem::size_of::<FileHeader>() / 4;
        // This is safe because `FileHeader` is `repr(C)` and has the same alignment as
        // `[u32; HEADER_SIZE]`.
        &*(&file_header as *const FileHeader as *const [u32; HEADER_SIZE])
    };

    compressed[..header_array.len()].copy_from_slice(header_array);

    compressed
}

fn optimal_frequencies_12bit(counts: &HashMap<i16, u32>) -> Vec<(i16, u16)> {
    assert!(!counts.is_empty());

    let max_weight = (1 << 12) - 1;

    if counts.len() == 1 {
        // The file format does not support degenerate models with all probability mass
        // on a single symbol. We therefore add a token additional symbol with minimal
        // frequency.
        let only_symbol = *counts.iter().next().unwrap().0;
        return vec![(only_symbol, max_weight), (only_symbol.wrapping_add(1), 1)];
    }

    // Start by assigning each symbol weight 1 and then distributing no more than
    // the remaining weight approximately evenly across all symbols.
    let free_weight = (1 << 12) - counts.len() as u32;
    let total_count = counts.iter().map(|(_, &count)| count).sum::<u32>();
    let mut remaining_weight = 1 << 12;

    let mut symbols_counts_weights_wins_losses = counts
        .iter()
        .map(|(&symbol, &count)| {
            let weight = (1 + count as u64 * free_weight as u64 / total_count as u64) as u16;
            remaining_weight -= weight;

            // How much the cross entropy would decrease when increasing the weight by one.
            let win = if weight == max_weight {
                std::f64::NEG_INFINITY
            } else {
                count as f64 * ((weight + 1) as f64 / weight as f64).log2()
            };

            // How much the cross entropy would increase when decreasing the weight by one.
            let loss = if weight == 1 {
                std::f64::INFINITY
            } else {
                count as f64 * (weight as f64 / (weight - 1) as f64).log2()
            };

            (symbol, count, weight, win, loss)
        })
        .collect::<Vec<_>>();

    // Distribute remaining weight evenly among symbols with highest wins.
    // Break ties by symbol value to make output deterministic.
    symbols_counts_weights_wins_losses.sort_by(|&(s1, _, _, win1, _), &(s2, _, _, win2, _)| {
        (win2, s1).partial_cmp(&(win1, s2)).unwrap()
    });
    for (_, count, weight, win, loss) in
        &mut symbols_counts_weights_wins_losses[..remaining_weight as usize]
    {
        *weight += 1; // Cannot end up in `max_weight` because win would otherwise be zero.
        *win = if *weight == max_weight {
            std::f64::NEG_INFINITY
        } else {
            *count as f64 * ((*weight + 1) as f64 / *weight as f64).log2()
        };
        *loss = *count as f64 * (*weight as f64 / (*weight - 1) as f64).log2();
    }

    loop {
        // Find element where increasing weight would incur the biggest win.
        let (buyer_index, &(_, _, _, buyer_win, _)) = symbols_counts_weights_wins_losses
            .iter()
            .enumerate()
            .max_by(|(_, (_, _, _, win1, _)), (_, (_, _, _, win2, _))| {
                win1.partial_cmp(win2).unwrap()
            })
            .unwrap();
        let (seller_index, (_, seller_count, seller_weight, seller_win, seller_loss)) =
            symbols_counts_weights_wins_losses
                .iter_mut()
                .enumerate()
                .min_by(|(_, (_, _, _, _, loss1)), (_, (_, _, _, _, loss2))| {
                    loss1.partial_cmp(loss2).unwrap()
                })
                .unwrap();

        if buyer_index == seller_index {
            // This can only happen due to rounding errors. In this case, we can't expect
            // to be able to improve further.
            break;
        }

        if buyer_win <= *seller_loss {
            // We've found the optimal solution.
            break;
        }

        *seller_weight -= 1;
        *seller_win =
            *seller_count as f64 * ((*seller_weight + 1) as f64 / *seller_weight as f64).log2();
        *seller_loss = if *seller_weight == 1 {
            std::f64::INFINITY
        } else {
            *seller_count as f64 * (*seller_weight as f64 / (*seller_weight - 1) as f64).log2()
        };

        let (_, buyer_count, buyer_weight, buyer_win, buyer_loss) =
            &mut symbols_counts_weights_wins_losses[buyer_index];
        *buyer_weight += 1;
        *buyer_win = if *buyer_weight == max_weight {
            std::f64::NEG_INFINITY
        } else {
            *buyer_count as f64 * ((*buyer_weight + 1) as f64 / *buyer_weight as f64).log2()
        };
        *buyer_loss =
            *buyer_count as f64 * (*buyer_weight as f64 / (*buyer_weight - 1) as f64).log2();
    }

    let mut ret = symbols_counts_weights_wins_losses
        .into_iter()
        .map(|(symbol, _, weight, _, _)| (symbol, weight))
        .collect::<Vec<_>>();
    ret.sort_by_key(|&(s, w)| (u16::max_value() - w, s)); // Sort to make output deterministic.
    ret
}

/// Calculates checked differences and their statistics.
///
/// Returns a tuple `(diffs, counts)`, where `diffs` has the same shape as
/// `input` and contains the differences from the left and right parent, and
/// `counts` contains a `Vec` of `HashMap`s that map from symbols in the respective
/// slice of `diff` to their counts.
fn get_diffs(input: RankThreeTensorView<i16>) -> (RankThreeTensor<i16>, Vec<HashMap<i16, u32>>) {
    let (num_timesteps, vocab_size, embedding_dim) = input.shape();
    let mut diffs = RankThreeTensor::new(
        num_timesteps as usize,
        vocab_size as usize,
        embedding_dim as usize,
    );
    let mut diffs_view = diffs.as_view_mut();
    let mut counts = vec![HashMap::new(); num_timesteps];

    // Copy over first and last time step and create their `counts`.
    for &t in [0, num_timesteps - 1].iter() {
        let source_view = input.subview(t as usize);
        let mut target_view = diffs_view.subview_mut(t as usize);
        let current_counts = &mut counts[t];

        for (target, source) in target_view
            .as_mut_slice()
            .iter_mut()
            .zip(source_view.slice())
        {
            *target = *source;
            current_counts
                .entry(*target)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
    }

    // Calculate diffs of inner time steps and create their `counts`.
    traverse_subtree(
        2,
        0,
        0,
        num_timesteps - 1,
        1,
        &mut |t, _level, left_t, _left_level, right_t, _right_level| {
            let left_view = input.subview(left_t as usize);
            let right_view = input.subview(right_t as usize);
            let center_view = input.subview(t as usize);
            let mut target_view = diffs_view.subview_mut(t as usize);
            let current_counts = &mut counts[t];

            for (((target, left), right), center) in target_view
                .as_mut_slice()
                .iter_mut()
                .zip(left_view.slice())
                .zip(right_view.slice())
                .zip(center_view.slice())
            {
                *target = (*center as i32 - ((*left as i32 + *right as i32) / 2))
                    .try_into()
                    .unwrap();
                current_counts
                    .entry(*target)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        },
    );

    (diffs, counts)
}

fn traverse_subtree<F: FnMut(usize, usize, usize, usize, usize, usize)>(
    level: usize,
    left_t: usize,
    left_level: usize,
    right_t: usize,
    right_level: usize,
    callback: &mut F,
) {
    let t = (left_t + right_t) / 2;
    if t != left_t {
        callback(t, level, left_t, left_level, right_t, right_level);
        traverse_subtree(level + 1, left_t, left_level, t, level, callback);
        traverse_subtree(level + 1, t, level, right_t, right_level, callback);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use super::super::{EmbeddingFile, TimestepReader};
    use crate::tensors::RankTwoTensorView;

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_optimal_frequencies_12bit() {
        fn test(counts_and_expected_frequencies: &[(u32, u16)]) {
            let (counts, expected) = counts_and_expected_frequencies
                .iter()
                .enumerate()
                .map(|(s, &(c, f))| ((s as i16, c), (s as i16, f)))
                .unzip();

            let symbols_and_frequencies = optimal_frequencies_12bit(&counts);
            let calculated = symbols_and_frequencies
                .into_iter()
                .collect::<HashMap<_, _>>();

            assert_eq!(calculated, expected);
        }

        test(&[(2, 0x0200), (5, 0x0500), (9, 0x0900)]);
        test(&[(3, 723), (5, 1205), (9, 2168)]);
        test(&[(3000, 723), (5000, 1204), (9008, 2169)]);
        test(&[(3000, 722), (5000, 1204), (9009, 2170)]);
    }

    #[test]
    fn create_file() {
        const NUM_TIMESTEPS: u32 = 6;
        const VOCAB_SIZE: u32 = 100;
        const EMBEDDING_DIM: u32 = 16;
        const CHUNK_SIZE: u32 = 20;

        let file_name = format!(
            "{}/tests/fake_data_generation/random_{}_{}_{}",
            env!("CARGO_MANIFEST_DIR"),
            NUM_TIMESTEPS,
            VOCAB_SIZE,
            EMBEDDING_DIM
        );
        let mut input_file = File::open(file_name).unwrap();

        let mut input_buf = Vec::new();
        input_file.read_to_end(&mut input_buf).unwrap();
        assert_eq!(
            input_buf.len(),
            (NUM_TIMESTEPS * VOCAB_SIZE * EMBEDDING_DIM) as usize
        );

        // Convert to i16.
        let input_buf = input_buf
            .iter()
            .map(|&x| x as i8 as i16)
            .collect::<Vec<_>>();

        // Check that negative values are treated correctly.
        assert_eq!(
            input_buf[(3 * VOCAB_SIZE * EMBEDDING_DIM + 5 * EMBEDDING_DIM + 10) as usize],
            -39
        );

        let uncompressed = RankThreeTensor::from_flattened(
            input_buf,
            NUM_TIMESTEPS as usize,
            VOCAB_SIZE as usize,
            EMBEDDING_DIM as usize,
        );
        let uncompressed = uncompressed.as_view();

        const SCALE_FACTOR: f32 = 1.5;
        let compressed = build_file(uncompressed, CHUNK_SIZE, SCALE_FACTOR);
        let compressed_len = compressed.len();

        let file = EmbeddingFile::new(compressed.into_boxed_slice()).unwrap();

        let header = file.header();
        assert_eq!(
            header,
            &FileHeader {
                magic: ('\0' as u32) | ('d' as u32) << 8 | ('w' as u32) << 16 | ('e' as u32) << 24,
                major_version: 1,
                minor_version: 0,
                file_size: compressed_len as u32,
                meta_size: header.meta_size, // Gets checked below.
                num_timesteps: NUM_TIMESTEPS,
                vocab_size: VOCAB_SIZE,
                embedding_dim: EMBEDDING_DIM,
                chunk_size: CHUNK_SIZE,
                scale_factor: SCALE_FACTOR,
            }
        );

        // Check `header.meta_size` *after* we've verified the other header fields
        // because a malformed header would likely cause the below calculation of
        // `min_chunk_address` fail in a way that would be hard to debug.
        let min_chunk_address = *(0..NUM_TIMESTEPS)
            .flat_map(|t| file.timestep(t).unwrap().chunk_addresses)
            .min()
            .unwrap();
        assert_eq!(header.meta_size, min_chunk_address);

        let test_timestep = |t, expected: RankTwoTensorView<i16>| {
            let timestep = file.timestep(t).unwrap();
            assert_eq!(
                timestep.chunk_addresses.len(),
                (VOCAB_SIZE / CHUNK_SIZE) as usize
            );

            let mut buf = [0i16; EMBEDDING_DIM as usize];
            let mut reader = timestep.reader();
            for i in &[0, 1, 8, 19, 20, 25, 45, 59, 67, 68, 83, 99] {
                reader
                    .next_diff_vector_in_ascending_order(*i, buf.iter_mut(), |source, dest| {
                        *dest = source;
                    })
                    .unwrap();
                assert_eq!(&buf[..], expected.subview(*i as usize));
            }
        };

        let (diffs, _) = get_diffs(uncompressed);
        for t in 0..NUM_TIMESTEPS {
            test_timestep(t, diffs.as_view().subview(t as usize));
        }

        // Just in case `get_diffs` is wrong, let's also explicitly test the
        // first, last, and center time steps.
        test_timestep(0, uncompressed.subview(0));
        test_timestep(
            NUM_TIMESTEPS - 1,
            uncompressed.subview((NUM_TIMESTEPS - 1) as usize),
        );

        let center_timestep = ((NUM_TIMESTEPS - 1) / 2);
        let center_diff = uncompressed
            .subview(0)
            .slice()
            .iter()
            .zip(uncompressed.subview((NUM_TIMESTEPS - 1) as usize).slice())
            .zip(uncompressed.subview(center_timestep as usize).slice())
            .map(|((&first, &last), &center)| {
                (center as i32 - ((first as i32 + last as i32) / 2)) as i16
            })
            .collect::<Vec<_>>();
        let center_diff =
            RankTwoTensorView::from_flattened(VOCAB_SIZE, EMBEDDING_DIM, &center_diff);
        test_timestep(center_timestep, center_diff);
    }
}
