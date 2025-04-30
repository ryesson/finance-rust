use crate::common::currency::Currency;
use serde::Deserialize;
use std::ops::AddAssign;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Price {
    pub amount: f64,
    pub currency: Currency,
}

impl AddAssign<f64> for Price {
    fn add_assign(&mut self, rhs: f64) {
        self.amount += rhs;
    }
}

impl ToString for Price {
    fn to_string(&self) -> String {
        return format!("{}{}", self.currency.to_string(), self.amount);
    }
}
