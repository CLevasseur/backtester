extern crate chrono;

use signal::Signal;
use self::chrono::prelude::{DateTime, Utc};

mod once;
pub use signal::detector::once::Once;
mod always;
pub use signal::detector::always::Always;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DetectSignalError {
    IndicatorError
}

pub trait DetectSignal {
    fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError>;
}

