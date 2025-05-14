use crate::common::price::Price;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub enum ExecutionMode {
    Simulated,
    RealWorld,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub execution_mode: ExecutionMode,
    pub generator_params: GeneratorParams,
    pub brownian_params: BrownianParams,
}

#[derive(Debug, Deserialize)]
pub enum GeneratorType {
    Brownian,
}

#[derive(Debug, Deserialize)]
pub struct GeneratorParams {
    pub generator_type: GeneratorType,
    pub starts: Vec<Price>,
    pub ticks: usize,
    // TODO(ryesson): use uom::si::time::second
    pub delta_time: f64,
}

#[derive(Debug, Deserialize)]
pub struct BrownianParams {
    pub variance: f64,
}

pub fn parse_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let content: String = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    return Ok(config);
}
