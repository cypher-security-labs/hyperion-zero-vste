#![forbid(unsafe_code)]

pub struct SimdMatrixEngine<const N: usize>;

impl<const N: usize> SimdMatrixEngine<N> {
    pub fn multiply_vector(matrix: &[[f32; N]; N], vector: &[f32; N], result: &mut [f32; N]) {
        for i in 0..N {
            let mut sum = 0.0f32;
            for j in 0..N {
                sum += matrix[i][j] * vector[j];
            }
            result[i] = sum;
        }
    }
}

