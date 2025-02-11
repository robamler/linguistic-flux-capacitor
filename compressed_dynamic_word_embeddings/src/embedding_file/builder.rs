use super::{FileHeader, JumpPointer, HEADER_SIZE};
use crate::{
    tensors::{RankThreeTensor, RankThreeTensorView},
    u12::pack_u12s,
};

use byteorder::{LittleEndian, WriteBytesExt};
use constriction::{stream::stack::SmallAnsCoder, Pos, UnwrapInfallible};

use std::{collections::HashMap, convert::TryInto, io::Write};

type EncoderModel = constriction::stream::model::SmallNonContiguousCategoricalEncoderModel<i16>;

fn create_and_serialize_encoder_models(
    counts: &[HashMap<i16, u32>],
) -> Result<(Vec<EncoderModel>, Vec<u16>), ()> {
    let mut serialized = Vec::new();
    let mut models = Vec::with_capacity(counts.len());

    for counts in counts {
        let symbols_and_frequencies = optimal_frequencies_12bit(counts);

        let frequencies = symbols_and_frequencies
            .iter()
            .map(|&(_, f)| f)
            .collect::<Vec<_>>();

        let num_symbols: u16 = symbols_and_frequencies.len().try_into().map_err(|_| ())?;
        serialized.push(num_symbols);
        for &(symbol, _) in &symbols_and_frequencies {
            serialized.push(symbol as u16);
        }
        for frequency_encoding in pack_u12s(&frequencies[..frequencies.len() - 1]) {
            serialized.push(frequency_encoding);
        }

        models.push(
            EncoderModel::from_symbols_and_nonzero_fixed_point_probabilities(
                symbols_and_frequencies.iter().map(|&(s, _)| s),
                symbols_and_frequencies.iter().map(|&(_, f)| f),
                false,
            )?,
        );
    }

    // Add padding if necessary.
    if serialized.len() % 2 == 1 {
        serialized.push(0);
    }

    Ok((models, serialized))
}

fn compress_data(
    diffs: RankThreeTensorView<i16>,
    models: &[EncoderModel],
    jump_interval: u32,
) -> Result<(Vec<JumpPointer>, Vec<u16>), ()> {
    let (num_timesteps, vocab_size, embedding_dim) = diffs.shape();
    let num_timesteps: u32 = num_timesteps.try_into().unwrap();
    let vocab_size: u32 = vocab_size.try_into().unwrap();
    let embedding_dim: u32 = embedding_dim.try_into().unwrap();

    let mut tree_order = Vec::with_capacity(num_timesteps as usize);
    tree_order.push(0);
    tree_order.push(num_timesteps - 1);
    traverse_subtree(
        2,
        0,
        0,
        (num_timesteps - 1) as usize,
        1,
        &mut |t, _, _, _, _, _| {
            tree_order.push(t as u32);
        },
    );

    let jump_points_per_timestep = vocab_size.div_ceil(jump_interval);
    let jump_table_len = num_timesteps * jump_points_per_timestep;
    let mut jump_table_section = vec![JumpPointer::default(); jump_table_len as usize];

    let mut encoder = SmallAnsCoder::from_binary(vec![0]).unwrap_infallible();

    for &t in tree_order.iter().rev() {
        let model = &models[t as usize];
        let data = diffs.subview(t as usize).slice();
        let chunks = data.chunks(jump_interval as usize * embedding_dim as usize);

        for (i, chunk) in chunks.enumerate().rev() {
            encoder
                .encode_iid_symbols_reverse(chunk, model)
                .map_err(|_| ())?;
            let (pos, state) = encoder.pos();
            jump_table_section[(t * jump_points_per_timestep + i as u32) as usize] = JumpPointer {
                offset: pos as u32,
                state,
            };
        }
    }

    let (mut compressed_data_section, _) = encoder.into_raw_parts();
    compressed_data_section.reverse();

    let final_compressed_size: u32 = compressed_data_section.len().try_into().map_err(|_| ())?;
    for JumpPointer { offset, .. } in jump_table_section.iter_mut() {
        *offset = final_compressed_size - *offset;
    }

    // Apply padding if necessary (*after* getting `final_compressed_size`).
    if compressed_data_section.len() % 2 == 1 {
        compressed_data_section.push(0);
    }

    Ok((jump_table_section, compressed_data_section))
}

/// Returns the number of written *bytes* (not u32's) upon success.
pub fn write_compressed_dwe_file(
    uncompressed: RankThreeTensorView<i16>,
    jump_interval: u32,
    scale_factor: f32,
    mut output: impl Write,
) -> Result<usize, ()> {
    let (num_timesteps, vocab_size, embedding_dim) = uncompressed.shape();
    let num_timesteps: u32 = num_timesteps.try_into().unwrap();
    let vocab_size: u32 = vocab_size.try_into().unwrap();
    let embedding_dim: u32 = embedding_dim.try_into().unwrap();

    assert!(vocab_size > 0);
    assert!(embedding_dim > 0);
    assert!(num_timesteps >= 2);
    assert!(jump_interval > 0);
    assert!(jump_interval <= vocab_size);

    let (diffs, counts) = get_diffs(uncompressed);
    let (encoder_models, entropy_models_section) = create_and_serialize_encoder_models(&counts)?;
    let (jump_table_section, compressed_data_section) =
        compress_data(diffs.as_view(), &encoder_models, jump_interval)?;

    let entropy_model_section_size: u32 = (entropy_models_section.len() / 2)
        .try_into()
        .map_err(|_| ())?;
    let compressed_data_section_size: u32 = (compressed_data_section.len() / 2)
        .try_into()
        .map_err(|_| ())?;
    let jump_table_address = HEADER_SIZE + entropy_model_section_size;
    let file_size =
        jump_table_address + 2 * jump_table_section.len() as u32 + compressed_data_section_size;

    let file_header = FileHeader {
        magic: 0x6577_6400,
        major_version: 1,
        minor_version: 0,
        file_size,
        jump_table_address,
        num_timesteps,
        vocab_size,
        embedding_dim,
        jump_interval,
        scale_factor,
    };

    let header_section = unsafe {
        const HEADER_SIZE: usize = std::mem::size_of::<FileHeader>() / 4;
        // SAFETY: This is safe because `FileHeader` is `repr(C)` and has the same
        // alignment as `[u32; HEADER_SIZE]`.
        &*(&file_header as *const FileHeader as *const [u32; HEADER_SIZE])
    };

    // Serialize all sections to the output writer.
    for &word in header_section {
        output.write_u32::<LittleEndian>(word).map_err(|_| ())?;
    }
    for word in entropy_models_section {
        output.write_u16::<LittleEndian>(word).map_err(|_| ())?;
    }
    for JumpPointer { offset, state } in jump_table_section {
        output.write_u32::<LittleEndian>(offset).map_err(|_| ())?;
        output.write_u32::<LittleEndian>(state).map_err(|_| ())?;
    }
    for word in compressed_data_section {
        output.write_u16::<LittleEndian>(word).map_err(|_| ())?;
    }

    output.flush().map_err(|_| ())?;

    Ok(file_size as usize * 4)
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
                f64::NEG_INFINITY
            } else {
                count as f64 * ((weight + 1) as f64 / weight as f64).log2()
            };

            // How much the cross entropy would increase when decreasing the weight by one.
            let loss = if weight == 1 {
                f64::INFINITY
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
            f64::NEG_INFINITY
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
            f64::INFINITY
        } else {
            *seller_count as f64 * (*seller_weight as f64 / (*seller_weight - 1) as f64).log2()
        };

        let (_, buyer_count, buyer_weight, buyer_win, buyer_loss) =
            &mut symbols_counts_weights_wins_losses[buyer_index];
        *buyer_weight += 1;
        *buyer_win = if *buyer_weight == max_weight {
            f64::NEG_INFINITY
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
    ret.sort_by_key(|&(s, w)| (u16::MAX - w, s)); // Sort to make output deterministic.
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
    let mut diffs = RankThreeTensor::new(num_timesteps, vocab_size, embedding_dim);
    let mut diffs_view = diffs.as_view_mut();
    let mut counts = vec![HashMap::new(); num_timesteps];

    // Copy over first and last time step and create their `counts`.
    for &t in [0, num_timesteps - 1].iter() {
        let source_view = input.subview(t);
        let mut target_view = diffs_view.subview_mut(t);
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
            let left_view = input.subview(left_t);
            let right_view = input.subview(right_t);
            let center_view = input.subview(t);
            let mut target_view = diffs_view.subview_mut(t);
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
        const JUMP_INTERVAL: u32 = 20;

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

        const SCALE_FACTOR: f32 = 0.125;
        let mut compressed = Vec::<u8>::new();
        let file_size =
            write_compressed_dwe_file(uncompressed, JUMP_INTERVAL, SCALE_FACTOR, &mut compressed)
                .unwrap();

        assert_eq!(file_size, compressed.len());
        assert_eq!(file_size % 4, 0);
        assert_eq!(&compressed[0..4], b"\0dwe");

        let compressed = compressed
            .chunks_exact(4)
            .map(|chunk| {
                chunk[0] as u32
                    | ((chunk[1] as u32) << 8)
                    | ((chunk[2] as u32) << 16)
                    | ((chunk[3] as u32) << 24)
            })
            .collect::<Vec<u32>>();

        let compressed_len = compressed.len();

        let file = EmbeddingFile::new(compressed.into_boxed_slice()).unwrap();

        let header = file.header();
        assert_eq!(
            header,
            &FileHeader {
                magic: header.magic, // Already checked above.
                major_version: 1,
                minor_version: 0,
                file_size: compressed_len as u32,
                jump_table_address: header.jump_table_address, // Checked in `EmbeddingFile::new`.
                num_timesteps: NUM_TIMESTEPS,
                vocab_size: VOCAB_SIZE,
                embedding_dim: EMBEDDING_DIM,
                jump_interval: JUMP_INTERVAL,
                scale_factor: SCALE_FACTOR,
            }
        );

        let test_timestep = |t, expected: RankTwoTensorView<i16>| {
            let mut timestep = file.timestep(t).unwrap();
            assert_eq!(
                timestep.jump_table.len(),
                VOCAB_SIZE.div_ceil(JUMP_INTERVAL) as usize
            );

            let mut buf = [0i16; EMBEDDING_DIM as usize];
            for i in &[0, 1, 8, 19, 20, 25, 45, 59, 67, 68, 83, 99] {
                timestep.jump_to(*i).unwrap();
                timestep
                    .read_single_embedding_vector(buf.iter_mut(), |source, dest| {
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

        let center_timestep = (NUM_TIMESTEPS - 1) / 2;
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
