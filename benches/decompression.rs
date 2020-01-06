#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};
use rand::{rngs::StdRng, RngCore, SeedableRng};

use word_history_explorer_backend::ans::DistributionU8;

fn decompress(c: &mut Criterion) {
    let mut decompressed = vec![0u8; 1024 * 1024]; // 1 MiB
    let distribution = DistributionU8::new(250, &[10, 1, 15, 0, 0, 7, 100, 110, 13]);
    let mut root_rng = StdRng::seed_from_u64(54389);

    c.bench_function("decompress", |b| {
        let mut rng = StdRng::seed_from_u64(root_rng.next_u64());
        let uncompressed = distribution.generate_samples(decompressed.len(), &mut rng);
        let compressed = distribution.encode(&uncompressed).unwrap();

        b.iter(|| {
            distribution
                .decode_all_to(black_box(&compressed), &mut decompressed)
                .unwrap();
            black_box(&mut decompressed);
        })
    });
}

criterion_group!(benches, decompress);
criterion_main!(benches);
