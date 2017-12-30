extern crate chrono;
use self::chrono::prelude::{DateTime, Utc};

pub mod source;

use symbol::SymbolId;

#[derive(PartialEq, Clone, Debug)]
pub struct Ohlcv {
    symbol_id: SymbolId,
    datetime: DateTime<Utc>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u32
}

impl Ohlcv {

    pub fn new(symbol_id: SymbolId, datetime: DateTime<Utc>, open: f64, high: f64, low: f64, close: f64, volume: u32) -> Ohlcv {
        Ohlcv {
            symbol_id,
            datetime,
            open,
            high,
            low,
            close,
            volume
        }
    }

    pub fn symbol_id(&self) -> &SymbolId {
        &self.symbol_id
    }

    pub fn datetime(&self) -> &DateTime<Utc> {
        &self.datetime
    }

    pub fn open(&self) -> f64 {
        self.open
    }

    pub fn high(&self) -> f64 {
        self.high
    }

    pub fn low(&self) -> f64 {
        self.low
    }

    pub fn close(&self) -> f64 {
        self.close
    }

    pub fn volume(&self) -> u32 {
        self.volume
    }
}
