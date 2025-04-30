use finance_rust::config::parse::parse_config;
use finance_rust::config::parse::Config;
//use generator::stock::geometric_brownian_motion::generate;

fn main() {
    let _config: Config = parse_config("/execution.yaml").unwrap();
}
