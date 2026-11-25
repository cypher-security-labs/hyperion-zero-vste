use criterion::{criterion_group, criterion_main, Criterion};
use cypher_hyperion_vste::simd_engine::SimdMatrixEngine;
use cypher_hyperion_vste::crypto_integrity::CryptoIntegrityEngine;
use core::hint::black_box;

fn bench_simd_matrix_4x4(c: &mut Criterion) {
    const N: usize = 4;
    let matrix = black_box([[1.0f32; N]; N]);
    let vector = black_box([1.0f32; N]);
    let mut result = black_box([0.0f32; N]);

    c.bench_function("simd_matrix_multiply_4x4", |b| {
        b.iter(|| {
            SimdMatrixEngine::<N>::multiply_vector(
                black_box(&matrix),
                black_box(&vector),
                black_box(&mut result),
            );
        })
    });
}

fn bench_crypto_integrity_4x4(c: &mut Criterion) {
    const N: usize = 4;
    let matrix = black_box([[1.0f32; N]; N]);
    let vector = black_box([1.0f32; N]);
    let mut tag = black_box([0.0f32; N]);

    c.bench_function("crypto_integrity_verify_tag_4x4", |b| {
        b.iter(|| {
            CryptoIntegrityEngine::verify_and_tag(
                black_box(&matrix),
                black_box(&vector),
                black_box(&mut tag),
            );
        })
    });
}

fn bench_crypto_integrity_16x16(c: &mut Criterion) {
    const N: usize = 16;
    let matrix = black_box([[1.0f32; N]; N]);
    let vector = black_box([1.0f32; N]);
    let mut tag = black_box([0.0f32; N]);

    c.bench_function("crypto_integrity_verify_tag_16x16", |b| {
        b.iter(|| {
            CryptoIntegrityEngine::verify_and_tag(
                black_box(&matrix),
                black_box(&vector),
                black_box(&mut tag),
            );
        })
    });
}

criterion_group!(
    benches, 
    bench_simd_matrix_4x4, 
    bench_crypto_integrity_4x4, 
    bench_crypto_integrity_16x16
);
criterion_main!(benches);

