use crate::common::price::Price;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

pub fn generate_brownian(
    start: Price,
    steps: usize,
    delta_time: f64,
    volatility: f64,
) -> Vec<Price> {
    let mut rng = thread_rng();
    let normal = Normal::new(0.0, (delta_time).sqrt() * volatility).unwrap();

    let mut path = Vec::with_capacity(steps + 1);
    let mut current = start;
    path.push(current);

    for _ in 0..steps {
        let noise = normal.sample(&mut rng);
        current += noise;
        path.push(current);
    }
    return path;
}
