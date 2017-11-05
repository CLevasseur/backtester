extern crate chrono;

use symbol::{Symbol, SymbolOhlcvSource};
use direction::Direction;
use self::chrono::prelude::{DateTime, Utc};

pub struct Signal<'symbol, S: 'symbol + SymbolOhlcvSource> {
    symbol: &'symbol Symbol<'symbol, S>,
    direction: Direction,
    datetime: DateTime<Utc>,
    label: String
}

impl<'symbol, S: 'symbol + SymbolOhlcvSource> Signal<'symbol, S> {
    pub fn new(symbol: &'symbol Symbol<'symbol, S>, direction: Direction, datetime: DateTime<Utc>, label: String) -> Signal<'symbol, S> {
        Signal {
            symbol: symbol,
            direction: direction,
            datetime: datetime,
            label: label
        }
    }

    pub fn symbol(&self) -> &'symbol Symbol<'symbol, S> {
        self.symbol
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
