pub enum Currency {
    USD,
    GBP,
    EUR,
}

impl ToString for Currency {
    fn to_string(&self) -> String {
        match &self {
            Currency::USD => "$",
            Currency::GBP => "£",
            Currency::EUR => "€",
        }
        .to_string()
    }
}
