extern crate chrono;
use self::chrono::prelude::{DateTime, Utc};

pub mod source;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Ohlcv {
    datetime: DateTime<Utc>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u32
}

impl Ohlcv {

    pub fn new(datetime: DateTime<Utc>, open: f64, high: f64, low: f64, close: f64, volume: u32) -> Ohlcv {
        Ohlcv {
            datetime: datetime,
            open: open,
            high: high,
            low: low,
            close: close,
            volume: volume
        }
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
