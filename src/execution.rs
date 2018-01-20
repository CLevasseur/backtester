extern crate chrono;
use self::chrono::prelude::{DateTime, Utc};
use symbol::SymbolId;

#[derive(Clone, PartialEq, Debug)]
pub struct Execution {
    symbol_id: SymbolId,
    quantity: u32,
    price: f64,
    datetime: DateTime<Utc>
}

impl Execution {
    pub fn new(symbol_id: SymbolId, quantity: u32, price: f64, datetime: DateTime<Utc>) -> Self {
        Execution {
            symbol_id,
            quantity,
            price,
            datetime
        }
    }
}