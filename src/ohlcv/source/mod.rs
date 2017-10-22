extern crate chrono;
use std::fmt;
use self::chrono::prelude::{DateTime, Utc};
use ohlcv::Ohlcv;

pub mod csv;

pub trait OhlcvSource {
    fn ohlcv(&self, date: &DateTime<Utc>) -> Result<Ohlcv, OhlcvSourceError>;
}

#[derive(Debug)]
pub enum OhlcvSourceError {
    DateNotFound(DateTime<Utc>)
}

impl fmt::Display for OhlcvSourceError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            OhlcvSourceError::DateNotFound(date) => format!("Date not found: {}", date)
        })
    }
}
