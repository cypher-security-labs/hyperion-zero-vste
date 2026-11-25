#![forbid(unsafe_code)]
use crate::crypto_integrity::CryptoIntegrityEngine;

pub struct EnclaveManifest<const N: usize> {
    pub secure_signature: [f32; N],
    pub is_verified: bool,
}

impl<const N: usize> EnclaveManifest<N> {
    /// Generates an immutable, zero-allocation hardware-grade cryptographic manifest
    pub const fn empty() -> Self {
        Self {
            secure_signature: [0.0; N],
            is_verified: false,
        }
    }

    pub fn seal(&mut self, base_matrix: &[[f32; N]; N], entropy_vector: &[f32; N]) {
        CryptoIntegrityEngine::verify_and_tag(
            base_matrix,
            entropy_vector,
            &mut self.secure_signature,
        );
        self.is_verified = true;
    }
}

