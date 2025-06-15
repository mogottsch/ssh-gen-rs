use core::keypair::generate_keypair;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn bench_public_key_string(c: &mut Criterion) {
    let key_pair = generate_keypair();

    c.bench_function("public_key_string", |b| {
        b.iter(|| {
            let public_key_string = key_pair.public_key_string();
            black_box(public_key_string)
        })
    });
}

criterion_group!(benches, bench_public_key_string);
criterion_main!(benches);
