use rand::{rngs::ThreadRng, Rng};

pub fn is_zero(x: f64) -> bool {
    (x - 0.0).abs() < f64::EPSILON
}

pub fn sample_exponential(mean: f64, rng: &mut ThreadRng) -> Option<f64> {
    match is_zero(mean) {
        true => None,
        false => Some(-1.0 * rng.gen::<f64>().ln() / mean),
    }
}
