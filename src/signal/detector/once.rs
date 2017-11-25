extern crate chrono;
use signal::Signal;
use signal::detector::{DetectSignal, DetectSignalError};
use self::chrono::prelude::{DateTime, Utc};

pub struct Once {}

impl Once {
    pub fn new() -> Once { Once {} }
}

impl<'symbol> DetectSignal<'symbol> for Once {
    fn detect_signal(&self, _datetime: &DateTime<Utc>) -> Result<Option<Signal<'symbol>>, DetectSignalError> {
        Ok(None)
    }
}
