extern crate chrono;
use std::fmt;
use self::chrono::prelude::{DateTime, Utc};
use ohlcv::Ohlcv;

mod null;
mod csv;
pub use self::null::NullOhlcvSource;
pub use self::csv::CsvOhlcvSource;

pub trait OhlcvSource {
    fn ohlcv(&self, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) 
        -> Result<Vec<Ohlcv>, OhlcvSourceError>;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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
