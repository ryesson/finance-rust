use crate::common::currency::Currency;

pub struct Price {
    amount: f64,
    currency: Currency,
}

impl ToString for Price {
    fn to_string(&self) -> String {
        return format!("{}{}", self.currency.to_string(), self.amount);
    }
}
