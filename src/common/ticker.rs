#[derive(Clone)]
pub struct Ticker(pub String);

impl ToString for Ticker {
    fn to_string(&self) -> String {
        return self.0.to_string();
    }
}
