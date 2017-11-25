extern crate chrono;
mod order_id;
mod order_status;

pub mod policy;

use self::chrono::prelude::{DateTime, Utc};
use direction::Direction;
use symbol::Symbol;
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
pub struct Order<'symbol> {
    id: OrderId,
    symbol: &'symbol Symbol<'symbol>,
    direction: Direction,
    quantity: u32,
    status: OrderStatus,
    kind: OrderKind,
    oca: Option<OcaGroup>,
    active_until: Option<DateTime<Utc>>,
    active_after: Option<DateTime<Utc>>
}

impl<'symbol> Order<'symbol> {

    pub fn id(&self) -> &OrderId {
        &self.id
    }

    pub fn symbol(&self) -> &'symbol Symbol<'symbol> {
        &self.symbol
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

pub struct OrderBuilder<'symbol> {
    id: OrderId,
    symbol: &'symbol Symbol<'symbol>,
    direction: Direction,
    quantity: u32,
    status: OrderStatus,
    kind: OrderKind,
    oca: Option<OcaGroup>,
    active_until: Option<DateTime<Utc>>,
    active_after: Option<DateTime<Utc>>
}

impl<'symbol> OrderBuilder<'symbol> {

    pub fn unallocated(kind: OrderKind, symbol: &'symbol Symbol<'symbol>,
                       direction: Direction) -> OrderBuilder<'symbol> 
    {
        OrderBuilder {
            id: OrderId::new(),
            symbol: symbol,
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

    pub fn build(self) -> Order<'symbol> {
        Order {
            id: self.id,
            symbol: self.symbol,
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
