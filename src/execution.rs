extern crate chrono;
use self::chrono::prelude::{DateTime, Utc};
use symbol::SymbolId;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

    pub fn symbol_id(&self) -> &SymbolId {
        &self.symbol_id
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn price(&self) -> &f64 {
        &self.price
    }

    pub fn datetime(&self) -> &DateTime<Utc> {
        &self.datetime
    }
}