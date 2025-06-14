use criterion::{Criterion, criterion_group, criterion_main};
use ssh_gen_rs::{check_suffix, generate_key};
use std::hint::black_box;

fn bench_generate_key(c: &mut Criterion) {
    c.bench_function("generate_key", |b| {
        b.iter(|| {
            let key_pair = generate_key();
            black_box(key_pair)
        })
    });
}

fn bench_check_suffix(c: &mut Criterion) {
    let key_pair = generate_key();
    let suffix = "ye";

    c.bench_function("check_suffix", |b| {
        b.iter(|| check_suffix(black_box(&key_pair.public_key), black_box(suffix)))
    });
}

criterion_group!(benches, bench_generate_key, bench_check_suffix);
criterion_main!(benches);

