use crate::common::price::Price;
use crate::common::ticker::Ticker;

pub enum Type {
    CALL,
    PUT,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match &self {
            Type::CALL => "call",
            Type::PUT => "put",
        }
        .to_string()
    }
}

struct Option {
    price: Price,
    strike: Price,
    ticker: Ticker,
    r#type: Type,
}

impl ToString for Option {
    fn to_string(&self) -> String {
        return format!("{}, {}, {}, {}", self.price.to_string(), self.strike.to_string(), self.ticker.to_string(), self.r#type.to_string());
    }
}
