use core::{
    keypair::{generate_and_check_key, generate_keypair},
    pattern::Pattern,
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn bench_generate_key(c: &mut Criterion) {
    c.bench_function("generate_keypair", |b| {
        b.iter(|| {
            let key_pair = generate_keypair();
            black_box(key_pair)
        })
    });
}

fn bench_generate_and_check_key(c: &mut Criterion) {
    let patterns = vec![
        Pattern::Suffix("yee".to_string()),
        Pattern::Suffix("abc".to_string()),
        Pattern::Suffix("xyz".to_string()),
    ];
    c.bench_function("generate_and_check_key", |b| {
        b.iter(|| {
            generate_and_check_key(&patterns);
        })
    });
}

criterion_group!(benches, bench_generate_key, bench_generate_and_check_key);
criterion_main!(benches);
