extern crate chrono;

use self::chrono::prelude::{DateTime, Utc};
use ohlcv::Ohlcv;
use ohlcv::source::{OhlcvSource, OhlcvSourceError};
use symbol::SymbolOhlcvSource;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NullOhlcvSource {}

impl OhlcvSource for NullOhlcvSource {
    fn ohlcv(&self, date: &DateTime<Utc>) -> Result<Ohlcv, OhlcvSourceError> { 
        Err(OhlcvSourceError::DateNotFound(date.clone()))
    }
}

impl SymbolOhlcvSource for NullOhlcvSource {}

impl NullOhlcvSource {

    pub fn new() -> NullOhlcvSource {
        NullOhlcvSource {}
    }

}
