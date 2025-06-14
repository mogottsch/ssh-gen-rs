use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use vanity_ssh_rs::core::keypair::generate_keypair;
use vanity_ssh_rs::core::pattern::{Pattern, public_key_matches_pattern};

fn bench_generate_key(c: &mut Criterion) {
    c.bench_function("generate_keypair", |b| {
        b.iter(|| {
            let key_pair = generate_keypair();
            black_box(key_pair)
        })
    });
}

fn bench_check_suffix(c: &mut Criterion) {
    let key_pair = generate_keypair();
    let pattern = Pattern::Suffix("yee".to_string());

    c.bench_function("public_key_matches_pattern", |b| {
        b.iter(|| public_key_matches_pattern(black_box(&key_pair.public_key), black_box(&pattern)))
    });
}

criterion_group!(benches, bench_generate_key, bench_check_suffix);
criterion_main!(benches);
