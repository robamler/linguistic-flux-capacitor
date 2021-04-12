#[macro_use]
extern crate criterion;

use std::{fs::File, io::BufReader};

use byteorder::{LittleEndian, ReadBytesExt};
use constriction::{
    stream::{models::SmallNonContiguousLookupDecoderModel, stack::AnsCoder, Decode},
    Seek,
};
use criterion::{black_box, Criterion};
use rand::prelude::*;

use compressed_dynamic_word_embeddings::{
    embedding_file::{EmbeddingFile, FileHeader, TimestepReader, HEADER_SIZE},
    u12::unpack_u12s,
};

fn decompress(c: &mut Criterion) {
    let file_name = format!(
        "{}/../cli/stream-based/vbq_q32.0_b12_c100.dwe",
        env!("CARGO_MANIFEST_DIR"),
    );

    let file = BufReader::new(File::open(file_name).unwrap());
    let embedding_file = EmbeddingFile::from_reader(file).unwrap();

    let header = embedding_file.header();
    let num_timesteps = header.num_timesteps;
    let vocab_size = header.vocab_size;
    let embedding_dim = header.embedding_dim;
    dbg!(num_timesteps, vocab_size, embedding_dim);

    let decompress_t = |t: u32| {
        let mut checksum = 0i32;
        let mut timestep = embedding_file.timestep(t).unwrap();
        for _ in 0..vocab_size {
            timestep
                .read_single_embedding_vector(0..embedding_dim, |s, d| {
                    // Perform some more-or-less realistic amount of computation on each symbol.
                    checksum = checksum.wrapping_add(s as i32 * d as i32);
                })
                .unwrap();
        }
        checksum
    };

    let mut rng = rand::thread_rng();

    // Benchmark average runtime for decompressing a randomly selected time step.
    // The measured runtime can be used to estimate the runtime for calculating
    // trajectories (multiply it with `num_timesteps * jump_interval / vocab_size`).
    c.bench_function("decompress_random", |b| {
        b.iter(|| {
            black_box(decompress_t(black_box(rng.next_u32() % num_timesteps)));
        })
    });

    // Benchmark for estimating the runtime to calculate most related words for a
    // given target time step. Takes into account that time steps that are closer
    // to the root (which tend to be more expensive) are needed more often.
    c.bench_function("decompress_bisect", |b| {
        b.iter(|| {
            let target_t = black_box(rng.next_u32() % num_timesteps);

            let checksum = if target_t == 0 || target_t == num_timesteps - 1 {
                decompress_t(target_t)
            } else {
                let mut left_t = 0;
                let mut right_t = num_timesteps - 1;
                let mut checksum = decompress_t(left_t);
                checksum = checksum.wrapping_add(decompress_t(right_t));

                loop {
                    let t = (left_t + right_t) / 2;
                    checksum = checksum.wrapping_add(decompress_t(t));
                    match t.cmp(&target_t) {
                        std::cmp::Ordering::Equal => break checksum,
                        std::cmp::Ordering::Less => left_t = t,
                        std::cmp::Ordering::Greater => right_t = t,
                    }
                }
            };

            black_box(checksum);
        })
    });
}

fn construct_decoder_models(c: &mut Criterion) {
    let file_name = format!(
        "{}/../cli/stream-based/vbq_q32.0_b12_c100.dwe",
        env!("CARGO_MANIFEST_DIR"),
    );

    let mut file = BufReader::new(File::open(file_name).unwrap());
    let mut buf = Vec::new();
    buf.resize(HEADER_SIZE as usize, 0);
    file.read_u32_into::<LittleEndian>(&mut buf[..]).unwrap();

    let header = unsafe {
        // SAFETY: We made sure that buf.len() == HEADER_SIZE
        FileHeader::memory_map_unsafe(&buf)
    };
    let file_size = header.file_size;

    buf.reserve_exact((file_size - HEADER_SIZE) as usize);
    for _ in HEADER_SIZE..file_size {
        buf.push(file.read_u32::<LittleEndian>().map_err(|_| ()).unwrap());
    }
    let mut buf_container = Some(buf.into());

    c.bench_function("construct_decoder_models", |b| {
        b.iter(|| {
            let buf = buf_container.take().unwrap();
            let embedding_file = black_box(EmbeddingFile::new(black_box(buf)).unwrap());
            buf_container = Some(embedding_file.into_inner())
        })
    });
}

fn decompress_constriction(c: &mut Criterion) {
    let file_name = format!(
        "{}/../cli/stream-based/vbq_q32.0_b12_c100.dwe",
        env!("CARGO_MANIFEST_DIR"),
    );

    let file = BufReader::new(File::open(file_name).unwrap());
    let embedding_file = EmbeddingFile::from_reader(file).unwrap();

    let header = embedding_file.header();
    let num_timesteps = header.num_timesteps;
    let vocab_size = header.vocab_size;
    let embedding_dim = header.embedding_dim;
    dbg!(num_timesteps, vocab_size, embedding_dim);

    let data = embedding_file.as_slice_u32();

    // Manually construct `constriction`'s entropy models for each time step.
    let entropy_models_section =
        get_u16_slice(&data[HEADER_SIZE as usize..header.jump_table_address as usize]);
    let mut remainder = entropy_models_section;
    let decoder_models = (0..num_timesteps)
        .map(|_| {
            let (model, r) = deserialize_decoder_model(remainder).unwrap();
            remainder = r;
            model
        })
        .collect::<Vec<_>>();
    assert!(remainder.len() <= 1);

    let jump_points_per_timestep =
        ((header.vocab_size + header.jump_interval - 1) / header.jump_interval) as usize;
    let compressed_data_start = header.jump_table_address as usize
        + 2 * header.num_timesteps as usize * jump_points_per_timestep;

    #[repr(C)]
    struct JumpPointer {
        offset: u32,
        state: u32,
    }

    let full_jump_table_section = &data[header.jump_table_address as usize..compressed_data_start];
    let full_jump_table = unsafe {
        // SAFETY: Transmuting from `&[u32]` of even length to `&[JumpPointer]` is safe,
        // because `JumpPointer` is `repr(C)` and contains exactly two `u32`s.
        // See also https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
        let ptr = full_jump_table_section.as_ptr();
        std::slice::from_raw_parts(
            ptr as *const JumpPointer,
            header.num_timesteps as usize * jump_points_per_timestep,
        )
    };

    let compressed_data_section = get_u16_slice(&data[compressed_data_start..]);

    let decompress_t = |t: u32| {
        let decoder_model = &decoder_models[t as usize];
        let JumpPointer { offset, state } = full_jump_table[t as usize * jump_points_per_timestep];
        let mut decoder = AnsCoder::from_reversed_compressed(compressed_data_section).unwrap();
        decoder.seek((offset as usize, state)).unwrap();

        let mut checksum = 0i32;
        for _ in 0..vocab_size {
            // for s in 0..embedding_dim {
            //     let d = decoder.decode_symbol(decoder_model).unwrap();
            //     // Perform some more-or-less realistic amount of computation on each symbol.
            //     checksum = checksum.wrapping_add(s as i32 * d as i32);
            // }

            // for (s, d) in decoder
            //     .decode_iid_symbols(embedding_dim as usize, decoder_model)
            //     .map(Result::unwrap)
            //     .enumerate()
            // {
            //     // Perform some more-or-less realistic amount of computation on each symbol.
            //     checksum = checksum.wrapping_add(s as i32 * d as i32);
            // }

            decoder
                .map_decode_iid_symbols(0..embedding_dim, decoder_model, |s, d| {
                    // Perform some more-or-less realistic amount of computation on each symbol.
                    checksum = checksum.wrapping_add(s as i32 * d as i32);
                })
                .unwrap();
        }
        checksum
    };

    let mut rng = rand::thread_rng();

    // Benchmark average runtime for decompressing a randomly selected time step.
    // The measured runtime can be used to estimate the runtime for calculating
    // trajectories (multiply it with `num_timesteps * jump_interval / vocab_size`).
    c.bench_function("decompress_random_constriction", |b| {
        b.iter(|| {
            black_box(decompress_t(black_box(rng.next_u32() % num_timesteps)));
        })
    });

    // Benchmark for estimating the runtime to calculate most related words for a
    // given target time step. Takes into account that time steps that are closer
    // to the root (which tend to be more expensive) are needed more often.
    c.bench_function("decompress_bisect_constriction", |b| {
        b.iter(|| {
            let target_t = black_box(rng.next_u32() % num_timesteps);

            let checksum = if target_t == 0 || target_t == num_timesteps - 1 {
                decompress_t(target_t)
            } else {
                let mut left_t = 0;
                let mut right_t = num_timesteps - 1;
                let mut checksum = decompress_t(left_t);
                checksum = checksum.wrapping_add(decompress_t(right_t));

                loop {
                    let t = (left_t + right_t) / 2;
                    checksum = checksum.wrapping_add(decompress_t(t));
                    match t.cmp(&target_t) {
                        std::cmp::Ordering::Equal => break checksum,
                        std::cmp::Ordering::Less => left_t = t,
                        std::cmp::Ordering::Greater => right_t = t,
                    }
                }
            };

            black_box(checksum);
        })
    });
}

fn get_u16_slice(data: &[u32]) -> &[u16] {
    unsafe {
        // Transmuting from `&[u32]` to `&[u16]` is always safe, see, e.g.:
        // https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
        let ptr = data.as_ptr();
        std::slice::from_raw_parts(ptr as *const u16, 2 * data.len())
    }
}

fn deserialize_decoder_model(
    serialized: &[u16],
) -> Result<(SmallNonContiguousLookupDecoderModel<i16>, &[u16]), ()> {
    let num_symbols = serialized[0];
    let packed_size = 3 * num_symbols as usize / 4;

    // Extract remainder first to check most constrained bounds.
    let remainder = serialized
        .get(1 + num_symbols as usize + packed_size..)
        .ok_or(())?;
    let symbols = &serialized[1..1 + num_symbols as usize];
    let packed_frequencies =
        &serialized[1 + num_symbols as usize..1 + num_symbols as usize + packed_size];

    let model =
        SmallNonContiguousLookupDecoderModel::from_symbols_and_nonzero_fixed_point_probabilities(
            symbols.iter().map(|&s| s as i16),
            unpack_u12s(packed_frequencies, num_symbols - 1),
            true,
        )?;

    Ok((model, remainder))
}

criterion_group!(
    benches,
    decompress,
    decompress_constriction,
    construct_decoder_models
);
criterion_main!(benches);
