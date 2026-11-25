#![no_std]
#![forbid(unsafe_code)]

use cypher_hyperion_vste::crypto_integrity::CryptoIntegrityEngine;
use cypher_hyperion_vste::manifest::EnclaveManifest;
use core::hint::black_box;

#[test]
fn test_hyperion_zero_alien_enclave_pipeline() {
    const N: usize = 4;

    // 1. Initialize alien-grade matrix and entropy vector strictly on the stack
    let base_matrix = black_box([[2.0f32; N]; N]);
    let entropy_vector = black_box([1.5f32; N]);

    // 2. Initialize an empty immutable Enclave Manifest on the stack
    let mut manifest = EnclaveManifest::<N>::empty();
    assert!(!manifest.is_verified);

    // 3. Seal the enclave manifest using SIMD + Sigmoid Crypto Diffusion
    manifest.seal(&base_matrix, &entropy_vector);
    assert!(manifest.is_verified);

    // 4. Perform live runtime integrity verification check
    let mut runtime_tag = black_box([0.0f32; N]);
    CryptoIntegrityEngine::verify_and_tag(
        black_box(&base_matrix),
        black_box(&entropy_vector),
        black_box(&mut runtime_tag),
    );

    // 5. Zero-allocation mathematical validation of the secure state signature
    for i in 0..N {
        let diff = (manifest.secure_signature[i] - runtime_tag[i]).abs();
        assert!(diff < 1e-5);
    }
}

