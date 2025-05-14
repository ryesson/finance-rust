use crate::common::price::Price;
use crate::common::tick_data::TickData;
use crate::common::ticker::Ticker;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use uom::si::f64::*;
use uom::si::time::{millisecond, second};

pub fn generate_brownian(
    start_price: Price,
    ticker: Ticker,
    steps: usize,
    delta_time: Time,
    volatility: f64,
) -> Vec<TickData> {
    let mut rng = thread_rng();
    let normal = Normal::new(0.0, (delta_time.get::<second>()).sqrt() * volatility).unwrap();

    let mut path: Vec<TickData> = Vec::with_capacity(steps + 1);
    let mut opening_price: Price = start_price;
    let mut closing_price: Price = start_price;
    let mut start_window_timestamp = Time::new::<millisecond>(0.0);
    let mut end_window_timestamp = Time::new::<millisecond>(0.0);

    for _ in 0..steps {
        let noise = normal.sample(&mut rng);
        closing_price += noise;
        end_window_timestamp += delta_time;

        let current_tick: TickData = TickData {
            ticker: ticker.clone(),
            opening_price,
            closing_price,
            start_window_timestamp,
            end_window_timestamp,
        };
        path.push(current_tick);
        opening_price = closing_price;
        start_window_timestamp = end_window_timestamp;
    }
    return path;
}
