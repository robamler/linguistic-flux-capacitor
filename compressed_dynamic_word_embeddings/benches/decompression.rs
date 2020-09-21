#[macro_use]
extern crate criterion;

use std::{fs::File, io::BufReader};

use criterion::{black_box, Criterion};
use rand::prelude::*;

use compressed_dynamic_word_embeddings::embedding_file::{EmbeddingFile, TimestepReader};

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
            let checksum = decompress_t(rng.next_u32() % num_timesteps);
            black_box(checksum);
        })
    });

    // Benchmark for estimating the runtime to calculate most related words for a
    // given target time step. Takes into account that time steps that are closer
    // to the root (which tend to be more expensive) are needed more often.
    c.bench_function("decompress_bisect", |b| {
        b.iter(|| {
            let target_t = rng.next_u32() % num_timesteps;

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

criterion_group!(benches, decompress);
criterion_main!(benches);
