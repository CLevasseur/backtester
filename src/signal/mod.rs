extern crate chrono;

use symbol::Symbol;
use direction::Direction;
use self::chrono::prelude::{DateTime, Utc};

pub mod detector;

pub struct Signal<'symbol> {
    symbol: &'symbol Symbol<'symbol>,
    direction: Direction,
    datetime: DateTime<Utc>,
    label: String
}

impl<'symbol> Signal<'symbol> {
    pub fn new(symbol: &'symbol Symbol<'symbol>, direction: Direction, datetime: DateTime<Utc>, label: String) -> Signal<'symbol> {
        Signal {
            symbol: symbol,
            direction: direction,
            datetime: datetime,
            label: label
        }
    }

    pub fn symbol(&self) -> &'symbol Symbol<'symbol> {
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
