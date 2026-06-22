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

#[inline]
#[cfg(feature = "cosine")]
pub fn cosine_similarity(from: &[f64], to: &[f64]) -> Result<f64, VectorError> {
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
            return Ok(unsafe { cosine_similarity_avx2(from, to) });
        }
    }

    Ok(cosine_similarity_scalar(from, to))
}

/// Scalar cosine similarity. Returns -1.0 if either vector has zero magnitude.
#[inline]
#[cfg(feature = "cosine")]
fn cosine_similarity_scalar(from: &[f64], to: &[f64]) -> f64 {
    let len = from.len();
    let mut dot_product = 0.0;
    let mut magnitude_a = 0.0;
    let mut magnitude_b = 0.0;

    const CHUNK_SIZE: usize = 8;
    let chunks = len / CHUNK_SIZE;
    let remainder = len % CHUNK_SIZE;

    for i in 0..chunks {
        let offset = i * CHUNK_SIZE;
        let a_chunk = &from[offset..offset + CHUNK_SIZE];
        let b_chunk = &to[offset..offset + CHUNK_SIZE];

        let mut local_dot = 0.0;
        let mut local_mag_a = 0.0;
        let mut local_mag_b = 0.0;

        for j in 0..CHUNK_SIZE {
            let a_val = a_chunk[j];
            let b_val = b_chunk[j];
            local_dot += a_val * b_val;
            local_mag_a += a_val * a_val;
            local_mag_b += b_val * b_val;
        }

        dot_product += local_dot;
        magnitude_a += local_mag_a;
        magnitude_b += local_mag_b;
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

// SIMD implementation using AVX2 (256-bit vectors), selected at runtime.
#[cfg(all(feature = "cosine", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn cosine_similarity_avx2(a: &[f64], b: &[f64]) -> f64 {
    use std::arch::x86_64::*;

    let len = a.len();
    let chunks = len / 4; // AVX2 processes 4 f64 values at once

    unsafe {
        let mut dot_product = _mm256_setzero_pd();
        let mut magnitude_a = _mm256_setzero_pd();
        let mut magnitude_b = _mm256_setzero_pd();

        for i in 0..chunks {
            let offset = i * 4;
            let a_chunk = _mm256_loadu_pd(a.as_ptr().add(offset));
            let b_chunk = _mm256_loadu_pd(b.as_ptr().add(offset));

            dot_product = _mm256_add_pd(dot_product, _mm256_mul_pd(a_chunk, b_chunk));
            magnitude_a = _mm256_add_pd(magnitude_a, _mm256_mul_pd(a_chunk, a_chunk));
            magnitude_b = _mm256_add_pd(magnitude_b, _mm256_mul_pd(b_chunk, b_chunk));
        }

        let mut dot = horizontal_sum_pd(dot_product);
        let mut mag_a = horizontal_sum_pd(magnitude_a);
        let mut mag_b = horizontal_sum_pd(magnitude_b);

        // Handle the tail elements that don't fill a 4-wide lane.
        for i in (chunks * 4)..len {
            let a_val = *a.get_unchecked(i);
            let b_val = *b.get_unchecked(i);
            dot += a_val * b_val;
            mag_a += a_val * a_val;
            mag_b += b_val * b_val;
        }

        if mag_a == 0.0 || mag_b == 0.0 {
            return -1.0;
        }

        dot / (mag_a.sqrt() * mag_b.sqrt())
    }
}

// Sum the 4 doubles in an AVX2 vector down to a scalar.
#[cfg(all(feature = "cosine", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn horizontal_sum_pd(v: std::arch::x86_64::__m256d) -> f64 {
    use std::arch::x86_64::*;
    // Add the high 128 bits to the low 128 bits.
    let sum_hi_lo = _mm_add_pd(_mm256_castpd256_pd128(v), _mm256_extractf128_pd(v, 1));
    // Add the high 64 bits to the low 64 bits.
    let sum = _mm_add_sd(sum_hi_lo, _mm_unpackhi_pd(sum_hi_lo, sum_hi_lo));
    _mm_cvtsd_f64(sum)
}
