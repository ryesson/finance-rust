use crate::common::price::Price;
use crate::common::ticker::Ticker;
use uom::si::f64::*;

// Represent a stock reading for a given window
pub struct TickData {
    // TODO(ryesson): simulate volumes and keep track of volume-weighted average price
    pub ticker: Ticker,
    pub opening_price: Price,
    pub closing_price: Price,
    pub start_window_timestamp: Time,
    pub end_window_timestamp: Time,
}
