use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    generator_params: GeneratorParams,
    brownian_params: BrownianParams,
}

#[derive(Debug, Deserialize)]
enum GeneratorType {
    Brownian,
}

#[derive(Debug, Deserialize)]
pub struct GeneratorParams {
    generator: GeneratorType,
    starts: Vec<f64>,
    ticks: u16,
    // TODO(ryesson): use uom::si::time::second
    delta_time: i64,
}

#[derive(Debug, Deserialize)]
pub struct BrownianParams {
    variance: f64,
}

pub fn parse_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let content: String = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    return Ok(config);
}
