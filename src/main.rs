use finance_rust::config::parse::parse_config;
use finance_rust::generator::generate::generate;

fn main() {
    let config = parse_config("/execution.yaml").unwrap();
    let _paths = generate(&config);
}
