use crate::engine::{types::VectorError, vector_core::vector::HVector};

pub const MAX_DISTANCE: f64 = 2.0;
pub const ORTHOGONAL: f64 = 1.0;
pub const MIN_DISTANCE: f64 = 0.0;

pub trait DistanceCalc {
    fn distance(from: &HVector, to: &HVector) -> Result<f64, VectorError>;
}
impl<'a> DistanceCalc for HVector<'a> {
    /// Calculates the distance between two vectors.
    ///
    /// It normalizes the distance to be between 0 and 2.
    ///
    /// - 1.0 (most similar) → Distance 0.0 (closest)
    /// - 0.0 (orthogonal) → Distance 1.0
    /// - -1.0 (most dissimilar) → Distance 2.0 (furthest)
    #[inline(always)]
    #[cfg(feature = "cosine")]
    fn distance(from: &HVector, to: &HVector) -> Result<f64, VectorError> {
        cosine_similarity(from.data, to.data).map(|sim| 1.0 - sim)
    }
}

/// Cosine similarity over f32 vectors. Computed in f32 (AVX2 when available),
/// returned as f64 to match the engine's distance plumbing.
#[inline]
#[cfg(feature = "cosine")]
pub fn cosine_similarity(from: &[f32], to: &[f32]) -> Result<f64, VectorError> {
    if from.len() != to.len() {
        println!("mis-match in vector dimensions!\n{} != {}", from.len(), to.len());
        return Err(VectorError::InvalidVectorLength);
    }

    // Dispatch to an AVX2 implementation at runtime when the CPU supports it.
    // This works on stable without `-C target-cpu=native` and stays portable.
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            // SAFETY: only reached after confirming AVX2 support at runtime.
            return Ok(unsafe { cosine_similarity_avx2(from, to) } as f64);
        }
    }

    Ok(cosine_similarity_scalar(from, to) as f64)
}

/// Scalar cosine similarity. Returns -1.0 if either vector has zero magnitude.
#[inline]
#[cfg(feature = "cosine")]
fn cosine_similarity_scalar(from: &[f32], to: &[f32]) -> f32 {
    let len = from.len();
    let mut dot_product = 0.0f32;
    let mut magnitude_a = 0.0f32;
    let mut magnitude_b = 0.0f32;

    const CHUNK_SIZE: usize = 8;
    let chunks = len / CHUNK_SIZE;
    let remainder = len % CHUNK_SIZE;

    for i in 0..chunks {
        let offset = i * CHUNK_SIZE;
        let a_chunk = &from[offset..offset + CHUNK_SIZE];
        let b_chunk = &to[offset..offset + CHUNK_SIZE];

        for j in 0..CHUNK_SIZE {
            let a_val = a_chunk[j];
            let b_val = b_chunk[j];
            dot_product += a_val * b_val;
            magnitude_a += a_val * a_val;
            magnitude_b += b_val * b_val;
        }
    }

    let remainder_offset = chunks * CHUNK_SIZE;
    for i in 0..remainder {
        let a_val = from[remainder_offset + i];
        let b_val = to[remainder_offset + i];
        dot_product += a_val * b_val;
        magnitude_a += a_val * a_val;
        magnitude_b += b_val * b_val;
    }

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return -1.0;
    }

    dot_product / (magnitude_a.sqrt() * magnitude_b.sqrt())
}

// SIMD implementation using AVX2 (256-bit = 8 x f32), selected at runtime.
#[cfg(all(feature = "cosine", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn cosine_similarity_avx2(a: &[f32], b: &[f32]) -> f32 {
    use std::arch::x86_64::*;

    let len = a.len();
    let chunks = len / 8; // AVX2 processes 8 f32 values at once

    unsafe {
        let mut dot = _mm256_setzero_ps();
        let mut mag_a = _mm256_setzero_ps();
        let mut mag_b = _mm256_setzero_ps();

        for i in 0..chunks {
            let offset = i * 8;
            let av = _mm256_loadu_ps(a.as_ptr().add(offset));
            let bv = _mm256_loadu_ps(b.as_ptr().add(offset));

            dot = _mm256_add_ps(dot, _mm256_mul_ps(av, bv));
            mag_a = _mm256_add_ps(mag_a, _mm256_mul_ps(av, av));
            mag_b = _mm256_add_ps(mag_b, _mm256_mul_ps(bv, bv));
        }

        let mut dot_s = horizontal_sum_ps(dot);
        let mut mag_a_s = horizontal_sum_ps(mag_a);
        let mut mag_b_s = horizontal_sum_ps(mag_b);

        // Handle the tail elements that don't fill an 8-wide lane.
        for i in (chunks * 8)..len {
            let a_val = *a.get_unchecked(i);
            let b_val = *b.get_unchecked(i);
            dot_s += a_val * b_val;
            mag_a_s += a_val * a_val;
            mag_b_s += b_val * b_val;
        }

        if mag_a_s == 0.0 || mag_b_s == 0.0 {
            return -1.0;
        }

        dot_s / (mag_a_s.sqrt() * mag_b_s.sqrt())
    }
}

// Sum the 8 floats in an AVX2 vector down to a scalar.
#[cfg(all(feature = "cosine", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn horizontal_sum_ps(v: std::arch::x86_64::__m256) -> f32 {
    use std::arch::x86_64::*;
    let mut tmp = [0.0f32; 8];
    unsafe { _mm256_storeu_ps(tmp.as_mut_ptr(), v) };
    tmp.iter().sum()
}
