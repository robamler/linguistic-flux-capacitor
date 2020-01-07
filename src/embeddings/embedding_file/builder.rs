use super::super::compression::DistributionU8;
use super::super::tensors::{RankThreeTensor, RankThreeTensorView, RankTwoTensor};
use super::{EmbeddingFile, FileHeader};

use std::convert::TryInto;
use std::io::Write;

pub fn compress_quantized_tensor(
    uncompressed: RankThreeTensorView<i8>,
    chunk_size: u32,
    scale_factor: f32,
) -> Vec<u32> {
    let (num_timesteps, vocab_size, embedding_dim) = uncompressed.shape();
    let num_timesteps: u32 = num_timesteps.try_into().unwrap();
    let vocab_size: u32 = vocab_size.try_into().unwrap();
    let embedding_dim: u32 = embedding_dim.try_into().unwrap();

    assert!(num_timesteps >= 2);
    assert_eq!(vocab_size % chunk_size, 0);
    let uncompressed_timestep_byte_size = vocab_size.checked_mul(embedding_dim).unwrap();
    assert_eq!(uncompressed_timestep_byte_size % 4, 0);

    let (diffs, counts) = get_diffs(uncompressed);

    let timestep_addrs_addr = std::mem::size_of::<FileHeader>() as u32 / 4;
    let first_timestep_offset = timestep_addrs_addr + num_timesteps - 2;
    let last_timestep_offset = first_timestep_offset + uncompressed_timestep_byte_size / 4;
    let root_block_size = last_timestep_offset + uncompressed_timestep_byte_size / 4;

    let mut address = root_block_size;
    let counts_view = counts.as_view();
    let timestep_ir = counts_view
        .iter_subviews()
        .map(|counts| {
            let (smallest_symbol, num_symbols) = find_optimal_nonzero_range(counts);
            // The data format does not allow distributions of length 1.
            let num_symbols = usize::max(num_symbols, 2);
            let largest_symbol = (smallest_symbol as u8).wrapping_add((num_symbols - 1) as u8);

            let size = (2 + num_symbols as u32 + 3) / 4 + vocab_size / chunk_size;
            let current_address = address;
            address += size;

            TimeStepIR {
                address: current_address,
                smallest_symbol: smallest_symbol as u8,
                largest_symbol: largest_symbol,
                counts,
            }
        })
        .collect::<Vec<_>>();

    let mut compressed = Vec::new();
    // TODO: estimate file size based on entropies and reserve it.

    // Fill in header later, when we know the file size.
    compressed.resize(timestep_addrs_addr as usize, 0);

    // Write timestep addresses.
    for TimeStepIR { address, .. } in &timestep_ir {
        compressed.push(*address);
    }

    // Write embedding vectors of first and last time step.
    compressed.resize(root_block_size as usize, 0);

    let first_timestep_dest = get_i8_slice_mut(
        &mut compressed[first_timestep_offset as usize..last_timestep_offset as usize],
    );
    first_timestep_dest.copy_from_slice(uncompressed.subview(0).as_slice());

    let last_timestep_dest =
        get_i8_slice_mut(&mut compressed[last_timestep_offset as usize..root_block_size as usize]);
    last_timestep_dest.copy_from_slice(
        uncompressed
            .subview((num_timesteps - 1) as usize)
            .as_slice(),
    );

    // Skip over time step meta data since we don't know the chunk addresses yet.
    compressed.resize(address as usize, 0);

    // Write out compressed chunks.
    let mut timestep_llir = Vec::new();
    timestep_llir.resize_with((num_timesteps - 2) as usize, Default::default);

    traverse_subtree(
        2,
        0,
        0,
        num_timesteps as usize - 1,
        1,
        &mut |t, level, left_t, left_level, right_t, right_level| {
            let ir = &timestep_ir[t - 1];

            let mut shifted_counts = [0u32; 256];
            let num_nonzero_counts =
                ir.largest_symbol.wrapping_sub(ir.smallest_symbol) as usize + 1;
            if ir.largest_symbol > ir.smallest_symbol {
                for (dest, src) in shifted_counts[..num_nonzero_counts]
                    .iter_mut()
                    .zip(&ir.counts[ir.smallest_symbol as usize..=ir.largest_symbol as usize])
                {
                    *dest = (*src).try_into().unwrap()
                }
            } else {
                let len_part1 = 256 - ir.smallest_symbol as usize;
                for (dest, src) in shifted_counts[..len_part1]
                    .iter_mut()
                    .zip(&ir.counts[ir.smallest_symbol as usize..])
                {
                    *dest = (*src).try_into().unwrap()
                }
                for (dest, src) in shifted_counts[len_part1..]
                    .iter_mut()
                    .zip(&ir.counts[..=ir.largest_symbol as usize])
                {
                    *dest = (*src).try_into().unwrap()
                }
            }
            let shifted_counts = &shifted_counts[..num_nonzero_counts];
            let mut frequencies =
                quantized_frequencies(&shifted_counts, uncompressed_timestep_byte_size);

            if frequencies[ir.smallest_symbol as usize] == 256 {
                frequencies[ir.smallest_symbol as usize] = 255;
                frequencies[ir.smallest_symbol.wrapping_add(1) as usize] = 1;
            }

            let mut frequencies_u8 = [0u8; 256];
            for (freq_u8, freq_u32) in frequencies_u8[..num_nonzero_counts]
                .iter_mut()
                .zip(frequencies.iter())
            {
                *freq_u8 = (*freq_u32).try_into().unwrap();
            }

            let distribution =
                DistributionU8::new(ir.smallest_symbol, &frequencies_u8[..num_nonzero_counts]);

            let mut chunk_addresses = Vec::<u32>::new();
            chunk_addresses.reserve((vocab_size / chunk_size) as usize);

            let diffs_view = diffs.as_view();
            let diffs_subview = diffs_view.subview(t - 1);
            let diffs = diffs_subview.as_slice();
            for uncompressed_chunk in diffs.chunks((chunk_size * embedding_dim) as usize) {
                chunk_addresses.push(compressed.len().try_into().unwrap());

                let mut compressed_chunk = distribution.encode(uncompressed_chunk).unwrap();
                if compressed_chunk.len() % 2 == 1 {
                    compressed_chunk.push(0);
                }

                compressed.reserve(compressed_chunk.len() / 2);
                let mut iter = compressed_chunk.iter();
                while let Some(first) = iter.next() {
                    let second = iter.next().unwrap();
                    // Little endian byte order: first bytes are least significant.
                    compressed.push((*second as u32) << 16 | (*first as u32));
                }
            }

            timestep_llir[t - 1] = Some(TimeStepLowLevelIR {
                address: ir.address,
                smallest_symbol: ir.smallest_symbol,
                largest_symbol: ir.largest_symbol,
                frequencies: frequencies_u8,
                chunk_addresses,
            });
        },
    );

    // Write out time step meta data.
    for meta in timestep_llir.into_iter() {
        let meta = meta.unwrap();
        let num_frequencies = meta.largest_symbol.wrapping_sub(meta.smallest_symbol) as usize + 1;
        let header_end_u32 = meta.address as usize + (2 + num_frequencies + 3) / 4;
        let header = get_u8_slice_mut(&mut compressed[meta.address as usize..header_end_u32]);
        header[0] = meta.smallest_symbol;
        header[1] = meta.largest_symbol;
        header[2..num_frequencies + 2].copy_from_slice(&meta.frequencies[..num_frequencies]);
        assert!(meta.chunk_addresses.len() == (vocab_size / chunk_size) as usize);
        let body = &mut compressed[header_end_u32..header_end_u32 + meta.chunk_addresses.len()];
        body.copy_from_slice(&meta.chunk_addresses);
    }

    // Write file header.
    let file_header = FileHeader {
        magic: 0, // TODO
        major_version: 0,
        minor_version: 1,
        file_size: compressed.len() as u32,
        num_timesteps,
        vocab_size,
        embedding_dim,
        chunk_size,
        scale_factor,
    };

    let header_array = unsafe {
        const HEADER_SIZE: usize = std::mem::size_of::<FileHeader>() / 4;
        // This is safe because `FileHeader` is `repr(C)` and has the same alignment as
        // `[u32; HEADER_SIZE]
        &*(&file_header as *const FileHeader as *const [u32; HEADER_SIZE])
    };

    compressed[..header_array.len()].copy_from_slice(header_array);

    compressed
}

struct TimeStepIR<'a> {
    address: u32,
    smallest_symbol: u8,
    largest_symbol: u8,
    counts: &'a [u32],
}

struct TimeStepLowLevelIR {
    address: u32,
    smallest_symbol: u8,
    largest_symbol: u8,
    frequencies: [u8; 256],
    chunk_addresses: Vec<u32>,
}

/// Calculates checked differences and their statistics.
fn get_diffs(uncompressed: RankThreeTensorView<i8>) -> (RankThreeTensor<u8>, RankTwoTensor<u32>) {
    let (num_timesteps, vocab_size, embedding_dim) = uncompressed.shape();
    let mut diffs = RankThreeTensor::<u8>::new(
        (num_timesteps - 2) as usize,
        vocab_size as usize,
        embedding_dim as usize,
    );
    let mut diffs_view = diffs.as_view_mut();

    let mut counts = RankTwoTensor::<u32>::new((num_timesteps - 2) as usize, 256);
    let mut counts_view = counts.as_view_mut();

    let mut nonzero_counts = Vec::<u32>::new();
    nonzero_counts.resize((num_timesteps - 2) as usize, 0);

    traverse_subtree(
        2,
        0,
        0,
        num_timesteps - 1,
        1,
        &mut |t, level, left_t, left_level, right_t, right_level| {
            let left_view = uncompressed.subview(left_t as usize);
            let right_view = uncompressed.subview(right_t as usize);
            let center_view = uncompressed.subview(t as usize);
            let mut target_view = diffs_view.subview_mut((t - 1) as usize);
            let mut counts_subview = counts_view.subview_mut((t - 1) as usize);

            for (((target_val, left_val), right_val), center_val) in target_view
                .as_mut_slice()
                .iter_mut()
                .zip(left_view.as_slice())
                .zip(right_view.as_slice())
                .zip(center_view.as_slice())
            {
                let diff = *center_val as i16 - (*left_val as i16 + *right_val as i16) / 2;
                // Convert into `i8` and then interpret as `u8` for correct sign treatment.
                let diff: i8 = diff.try_into().unwrap();
                *target_val = diff as u8;

                counts_subview[diff as u8 as usize] += 1;
            }

            debug_assert_eq!(
                counts_subview.iter().cloned().sum::<u32>() as usize,
                vocab_size * embedding_dim
            );
        },
    );

    (diffs, counts)
}

/// Find the smallest contiguous region that contains all nonzero values. This is
/// likely going to wrap around at the end of the `counts` slice because the
/// indices into counts correspond to `i8` symbols casted to `u8`.
///
/// # Returns
///
/// A tuple `(first_symbol, len)`.
///
/// Note that `len` may be one even though the data format does not allow zero
/// entropy distributions.
///
/// # Panics
///
/// Panics if `counts` contains only zeros.
fn find_optimal_nonzero_range(counts: &[u32]) -> (usize, usize) {
    let first_zero = if let Some((first_zero, _)) = counts
        .iter()
        .enumerate()
        .find(|(symbol, &count)| count == 0)
    {
        first_zero
    } else {
        return (0, 255);
    };

    let mut end_candidate = if first_zero != 0 {
        first_zero
    } else {
        counts
            .iter()
            .enumerate()
            .rev()
            .find(|(symbol, &count)| count != 0)
            .unwrap()
            .0
            + 1
    };

    // `end_candidate` cannot be zero but it can be `256`, which is semantically
    // equivalent.
    let mut last_symbol_was_zero = true;
    let mut best_start = 0;
    let mut best_len = 256;

    for (symbol, &count) in counts.iter().enumerate().skip(first_zero + 1) {
        match (last_symbol_was_zero, count == 0) {
            (true, false) => {
                // `symbol` is a candidate to start the run of nonzero symbols.
                last_symbol_was_zero = false;
                let len = end_candidate.wrapping_sub(symbol) % 256;
                if len < best_len {
                    best_start = symbol;
                    best_len = len;
                }
            }
            (false, true) => {
                // `symbol` is a candidate to end the run of nonzero symbols.
                last_symbol_was_zero = true;
                end_candidate = symbol;
            }
            _ => (),
        }
    }

    (best_start, best_len)
}

/// # TODO
///
/// This is a hacky heuristic. In reality, we should minimize the cross entropy here.
fn quantized_frequencies(counts: &[u32], total_counts: u32) -> [u32; 256] {
    debug_assert_eq!(counts.iter().cloned().sum::<u32>(), total_counts);

    let mut frequencies = [0u32; 256];
    let mut total_frequency = 257u32;

    let mut total_frequency = 0;
    for (freq, count) in frequencies.iter_mut().zip(counts) {
        if *count != 0 {
            *freq = u32::max((*count * 256 + total_counts) / total_counts, 1);
            total_frequency += *freq;
        };
    }

    while total_frequency > 256 {
        *frequencies.iter_mut().max().unwrap() -= 1;
        total_frequency -= 1;
    }

    frequencies
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

#[cfg(target_endian = "little")]
fn get_i8_slice_mut(data: &mut [u32]) -> &mut [i8] {
    unsafe {
        // Transmuting from `&mut [u32]` to `&mut [i8]` is always safe, see, e.g.:
        // https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
        let ptr = data.as_mut_ptr();
        std::slice::from_raw_parts_mut(ptr as *mut i8, 4 * data.len())
    }
}

#[cfg(target_endian = "little")]
fn get_u8_slice_mut(data: &mut [u32]) -> &mut [u8] {
    unsafe {
        // Transmuting from `&mut [u32]` to `&mut [u8]` is always safe, see, e.g.:
        // https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
        let ptr = data.as_mut_ptr();
        std::slice::from_raw_parts_mut(ptr as *mut u8, 4 * data.len())
    }
}

#[cfg(test)]
mod test {
    use super::super::EmbeddingFile;
    use super::*;

    use byteorder::{LittleEndian, ReadBytesExt};

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn create_file() {
        let num_timesteps = 6;
        let vocab_size = 100;
        let embedding_dim = 16;

        let file_name = format!(
            "tests/fake_data_generation/random_{}_{}_{}",
            num_timesteps, vocab_size, embedding_dim
        );
        dbg!(&file_name);
        let mut input_file = File::open(file_name).unwrap();

        let mut buf = Vec::new();
        input_file.read_to_end(&mut buf);
        assert_eq!(buf.len(), num_timesteps * vocab_size * embedding_dim);

        // Check that negative values are treated correctly.
        assert_eq!(
            buf[3 * vocab_size * embedding_dim + 5 * embedding_dim + 10] as i8,
            -7
        );

        let uncompressed = RankThreeTensor::from_flattened(
            u8_slice_to_i8_slice(&buf).to_vec(),
            num_timesteps,
            vocab_size,
            embedding_dim,
        );

        let chunk_size = 20;
        let scale_factor = 1.5f32;
        let compressed =
            compress_quantized_tensor(uncompressed.as_view(), chunk_size, scale_factor);

        let decoded = EmbeddingFile::new(compressed.into_boxed_slice()).unwrap();

        let header = decoded.header();
        dbg!(header);
    }

    fn u8_slice_to_i8_slice(data: &[u8]) -> &[i8] {
        unsafe {
            let ptr = data.as_ptr();
            std::slice::from_raw_parts_mut(ptr as *mut i8, data.len())
        }
    }
}
