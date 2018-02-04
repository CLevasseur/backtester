extern crate chrono;
use self::chrono::prelude::{DateTime, Utc};
use std::cell::Cell;
use direction::Direction;
use signal::Signal;
use signal::detector::{DetectSignal, DetectSignalError};
use symbol::SymbolId;

pub struct Once {
    symbol_id: SymbolId,
    direction: Direction,
    detected: Cell<bool>
}

impl Once {
    pub fn new(symbol_id: SymbolId, direction: Direction) -> Once {
        Once {
            symbol_id: symbol_id,
            direction: direction,
            detected: Cell::new(false)
        }
    }
}

impl DetectSignal for Once {
    fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
        if self.detected.get() {
            Ok(None)
        }
        else {
            let signal = Signal::new(
                self.symbol_id.clone(),
                self.direction.clone(),
                datetime.clone(),
                String::from("once")
            );
            self.detected.set(true);
            Ok(Some(signal))
        }
    }
}

