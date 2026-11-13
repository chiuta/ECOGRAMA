//! ECOGRAMA core: windowing and a reference (naive) spectrum used to test faster implementations.

use std::f64::consts::PI;

/// Periodic Hann window of length `n`.
pub fn hann(n: usize) -> Vec<f32> {
    (0..n)
        .map(|i| (0.5 - 0.5 * (2.0 * PI * i as f64 / n as f64).cos()) as f32)
        .collect()
}

/// Magnitude of bins `0..=n/2` by the textbook O(n²) DFT. Slow on purpose: it is the oracle
/// that FFT-based code must match.
pub fn dft_magnitude(x: &[f32]) -> Vec<f32> {
    let n = x.len();
    if n == 0 {
        return Vec::new();
    }
    (0..=n / 2)
        .map(|k| {
            let (mut re, mut im) = (0.0f64, 0.0f64);
            for (t, &v) in x.iter().enumerate() {
                let a = -2.0 * PI * k as f64 * t as f64 / n as f64;
                re += f64::from(v) * a.cos();
                im += f64::from(v) * a.sin();
            }
            (re * re + im * im).sqrt() as f32
        })
        .collect()
}

/// Index of the largest magnitude.
pub fn peak_bin(mags: &[f32]) -> Option<usize> {
    mags.iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .map(|(i, _)| i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hann_shape() {
        let w = hann(4);
        let want = [0.0, 0.5, 1.0, 0.5];
        for (a, b) in w.iter().zip(want) {
            assert!((a - b).abs() < 1e-6);
        }
    }

    #[test]
    fn sine_peaks_in_its_bin() {
        let n = 64;
        let w = hann(n);
        let x: Vec<f32> = (0..n)
            .map(|i| ((2.0 * PI * 8.0 * i as f64 / n as f64).sin() as f32) * w[i])
            .collect();
        assert_eq!(peak_bin(&dft_magnitude(&x)), Some(8));
    }

    #[test]
    fn empty_input() {
        assert!(dft_magnitude(&[]).is_empty());
        assert_eq!(peak_bin(&[]), None);
    }
}
