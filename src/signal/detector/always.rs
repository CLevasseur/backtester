extern crate chrono;
use self::chrono::prelude::{DateTime, Utc};
use direction::Direction;
use signal::Signal;
use signal::detector::{DetectSignal, DetectSignalError};
use symbol::SymbolId;

pub struct Always {
    symbol_id: SymbolId,
    direction: Direction
}

impl Always {
    pub fn new(symbol_id: SymbolId, direction: Direction) -> Self {
        Always { symbol_id, direction }
    }
}

impl DetectSignal for Always {
    fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
        let signal = Signal::new(
            self.symbol_id.clone(),
            self.direction.clone(),
            datetime.clone(),
            String::from("always detect signal")
        );
        Ok(Some(signal))
    }
}