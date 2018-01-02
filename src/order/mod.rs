extern crate chrono;
mod order_id;
mod order_status;

pub mod policy;

use self::chrono::prelude::{DateTime, Utc};
use direction::Direction;
use symbol::SymbolId;
pub use self::order_id::OrderId;
pub use self::order_status::{OrderStatus, CancellationReason};

#[derive(Clone, PartialEq, Debug)]
pub enum OrderKind {
    MarketOrder,
    LimitOrder(f64),
    StopOrder(f64)
}

pub type OcaGroup = u32;

#[derive(Clone, PartialEq, Debug)]
pub struct Order {
    id: OrderId,
    symbol_id: SymbolId,
    direction: Direction,
    quantity: u32,
    status: OrderStatus,
    kind: OrderKind,
    oca: Option<OcaGroup>,
    active_until: Option<DateTime<Utc>>,
    active_after: Option<DateTime<Utc>>
}

impl Order {

    pub fn id(&self) -> &OrderId {
        &self.id
    }

    pub fn symbol_id(&self) -> &SymbolId {
        &self.symbol_id
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn status(&self) -> &OrderStatus {
        &self.status
    }

    pub fn set_status(&mut self, value: OrderStatus) {
        self.status = value
    }

    pub fn kind(&self) -> &OrderKind {
        &self.kind
    }

    pub fn oca(&self) -> Option<OcaGroup> {
        self.oca
    }

    pub fn active_until(&self) -> Option<DateTime<Utc>> {
        self.active_until
    }

    pub fn active_after(&self) -> Option<DateTime<Utc>> {
        self.active_after
    }
}

pub struct OrderBuilder {
    id: OrderId,
    symbol_id: SymbolId,
    direction: Direction,
    quantity: u32,
    status: OrderStatus,
    kind: OrderKind,
    oca: Option<OcaGroup>,
    active_until: Option<DateTime<Utc>>,
    active_after: Option<DateTime<Utc>>
}

impl OrderBuilder {

    pub fn unallocated(kind: OrderKind, symbol_id: SymbolId,
                       direction: Direction) -> OrderBuilder 
    {
        OrderBuilder {
            id: OrderId::new(),
            symbol_id: symbol_id,
            direction: direction,
            quantity: 0,
            status: OrderStatus::NotSent,
            kind: kind,
            oca: None,
            active_until: None,
            active_after: None
        }
    }

    pub fn id(mut self, value: OrderId) -> Self {
        self.id = value;
        self
    }

    pub fn status(mut self, value: OrderStatus) -> Self {
        self.status = value;
        self
    }

    pub fn oca(mut self, value: OcaGroup) -> Self {
        self.oca = Some(value);
        self
    }

    pub fn active_until(mut self, value: DateTime<Utc>) -> Self {
        self.active_until = Some(value);
        self
    }

    pub fn active_after(mut self, value: DateTime<Utc>) -> Self {
        self.active_after = Some(value);
        self
    }

    pub fn build(self) -> Order {
        Order {
            id: self.id,
            symbol_id: self.symbol_id,
            direction: self.direction,
            quantity: self.quantity,
            status: self.status,
            kind: self.kind,
            oca: self.oca,
            active_until: self.active_until,
            active_after: self.active_after
        }
    }
}
