extern crate chrono;

use self::chrono::prelude::{DateTime, Utc};
use ohlcv::Ohlcv;
use ohlcv::source::{OhlcvSource, OhlcvSourceError};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NullOhlcvSource {}

impl OhlcvSource for NullOhlcvSource {
    fn ohlcv(&self, start_date: &DateTime<Utc>, _end_date: &DateTime<Utc>)
        -> Result<Vec<Ohlcv>, OhlcvSourceError> {
        Err(OhlcvSourceError::DateNotFound(start_date.clone()))
    }
}

impl NullOhlcvSource {
    pub fn new() -> NullOhlcvSource {
        NullOhlcvSource {}
    }
}
