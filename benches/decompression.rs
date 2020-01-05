#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};
use rand::{rngs::StdRng, RngCore, SeedableRng};

use word_history_explorer_backend::ans::DistributionU8;

fn decompress_u32_16(c: &mut Criterion) {
    let mut decompressed = vec![0u8; 1024 * 1024]; // 1 MiB
    let distribution = DistributionU8::new(100, &[10, 1, 15, 0, 0, 7, 100, 110, 13]);
    let mut root_rng = StdRng::seed_from_u64(54389);

    c.bench_function("decompress_u32_16", |b| {
        let mut rng = StdRng::seed_from_u64(root_rng.next_u64());
        let uncompressed = distribution.generate_samples(decompressed.len(), &mut rng);
        let compressed = distribution.encode_u32_16(&uncompressed);

        b.iter(|| {
            distribution
                .decode_u32_16(black_box(&compressed), &mut decompressed)
                .unwrap();
            black_box(&mut decompressed);
        })
    });
}

criterion_group!(benches, decompress_u32_16);
criterion_main!(benches);
