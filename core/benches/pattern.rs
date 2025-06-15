use core::keypair::generate_keypair;
use core::pattern::Pattern;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn bench_public_key_matches_pattern(c: &mut Criterion) {
    let key_pair = generate_keypair();
    let pattern = Pattern::Suffix("yee".to_string());

    c.bench_function("public_key_matches_pattern", |b| {
        b.iter(|| {
            let matches = pattern.matches_keypair(&key_pair);
            black_box(matches)
        })
    });
}

criterion_group!(benches, bench_public_key_matches_pattern);
criterion_main!(benches);
