use crate::common::tick_data::TickData;
use crate::common::ticker::Ticker;
use crate::config::parse::{Config, GeneratorType};
use crate::generator::stock::geometric_brownian_motion::generate_brownian;
use uom::si::f64::*;
use uom::si::time::second;

// TODO(ryesson) handle case AAZ -> ABA
fn increment_ticker_string(s: &mut String) {
    if let Some(last_char) = s.pop() {
        let next_char = std::char::from_u32(last_char as u32 + 1).expect("Invalid char increment");
        s.push(next_char);
    }
}

pub fn generate(config: &Config) -> Vec<Vec<TickData>> {
    let mut paths: Vec<Vec<TickData>> = vec![];
    let ticker_string = String::from("AAA");
    let mut current_ticker = Ticker(ticker_string.clone());
    match config.generator_params.generator_type {
        GeneratorType::Brownian => {
            for start in &config.generator_params.starts {
                let path: Vec<TickData> = generate_brownian(
                    *start,
                    current_ticker,
                    config.generator_params.ticks,
                    Time::new::<second>(config.generator_params.delta_time),
                    config.brownian_params.variance,
                );
                paths.push(path);
                increment_ticker_string(&mut ticker_string.clone());
                current_ticker = Ticker(ticker_string.clone());
            }
        }
    }
    return paths;
}
