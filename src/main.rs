use finance_rust::config::parse::{parse_config, ExecutionMode};
use finance_rust::generator::generate::generate;

fn main() {
    let config = parse_config("config/execution.toml").unwrap();
    match config.execution_mode {
        ExecutionMode::Simulated => {
            let _paths = generate(&config);
        }
        ExecutionMode::RealWorld => {}
    }
}
