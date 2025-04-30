use rand::distributions::Distribution;
use rand::distributions::Normal;
use rand::thread_rng;

fn generate(start: f64, steps: usize, dt: f64, volatility: f64) -> Vec<f64> {
    let mut rng = thread_rng();
    let normal = Normal::new(0.0, (dt).sqrt() * volatility).unwrap();

    let mut path = Vec::with_capacity(steps + 1);
    let mut current = start;
    path.push(current);

    for _ in 0..steps {
        let noise = normal.sample(&mut rng);
        current += noise;
        path.push(current);
    }

    path
}
