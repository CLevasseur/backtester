extern crate chrono;
use self::chrono::prelude::{DateTime, Utc};

pub mod source;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Ohlcv {
    pub datetime: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u32
}
