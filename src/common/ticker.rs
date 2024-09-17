pub struct Ticker(&'static str);

impl ToString for Ticker {
    fn to_string(&self) -> String {
        return self.0.to_string();
    }
}
