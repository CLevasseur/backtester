extern crate chrono;

use symbol::SymbolId;
use direction::Direction;
use self::chrono::prelude::{DateTime, Utc};

pub mod detector;

pub struct Signal {
    symbol_id: SymbolId,
    direction: Direction,
    datetime: DateTime<Utc>,
    label: String
}

impl Signal {
    pub fn new(symbol_id: SymbolId, direction: Direction, datetime: DateTime<Utc>, label: String) -> Signal {
        Signal {
            symbol_id: symbol_id,
            direction: direction,
            datetime: datetime,
            label: label
        }
    }

    pub fn symbol_id(&self) -> &SymbolId {
        &self.symbol_id
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
    
    pub fn datetime(&self) -> &DateTime<Utc> {
        &self.datetime
    }

    pub fn label(&self) -> &String {
        &self.label
    }
}
