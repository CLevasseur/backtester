extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate snowflake;

mod order_id;
mod order_id_generator;
mod order_status;

pub mod policy;

use self::chrono::prelude::{DateTime, Utc};
use direction::Direction;
use symbol::SymbolId;
use execution::Execution;
pub use self::order_id::OrderId;
pub use self::order_id_generator::{UUIDOrderIdGenerator, GenerateOrderId};
pub use self::order_status::{OrderStatus, CancellationReason};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum OrderKind {
    MarketOrder,
    LimitOrder(f64),
    StopOrder(f64)
}

pub type OcaGroup = String;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Order {
    id: OrderId,
    symbol_id: SymbolId,
    direction: Direction,
    quantity: u32,
    status: OrderStatus,
    kind: OrderKind,
    oca: Option<OcaGroup>,
    active_until: Option<DateTime<Utc>>,
    active_after: Option<DateTime<Utc>>,
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

    pub fn oca(&self) -> &Option<OcaGroup> {
        &self.oca
    }

    pub fn active_until(&self) -> &Option<DateTime<Utc>> {
        &self.active_until
    }

    pub fn active_after(&self) -> &Option<DateTime<Utc>> {
        &self.active_after
    }

    pub fn execution(&self) -> Option<&Execution> {
        match *self.status() {
            OrderStatus::Filled(ref execution) => Some(execution),
            _ => None
        }
    }

}

#[derive(Clone, PartialEq, Debug)]
pub struct OrderBuilder {
    id: Option<OrderId>,
    symbol_id: SymbolId,
    direction: Direction,
    quantity: u32,
    status: OrderStatus,
    kind: OrderKind,
    oca: Option<OcaGroup>,
    active_until: Option<DateTime<Utc>>,
    active_after: Option<DateTime<Utc>>,
}

impl OrderBuilder {

    pub fn unallocated(kind: OrderKind, symbol_id: SymbolId,
                       direction: Direction) -> OrderBuilder 
    {
        OrderBuilder {
            id: None,
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

    pub fn kind(&self) -> &OrderKind {
        &self.kind
    }

    pub fn symbol_id(&self) -> &SymbolId {
        &self.symbol_id
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn id(&self) -> &Option<OrderId> {
        &self.id
    }

    pub fn set_id(mut self, value: OrderId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn status(&self) -> &OrderStatus {
        &self.status
    }

    pub fn set_status(mut self, value: OrderStatus) -> Self {
        self.status = value;
        self
    }

    pub fn oca(&self) -> &Option<OcaGroup> {
        &self.oca
    }

    pub fn set_oca(mut self, value: Option<OcaGroup>) -> Self {
        self.oca = value;
        self
    }

    pub fn active_until(&self) -> &Option<DateTime<Utc>> {
        &self.active_until
    }

    pub fn set_active_until(mut self, value: Option<DateTime<Utc>>) -> Self {
        self.active_until = value;
        self
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn set_quantity(mut self, value: u32) -> Self {
        self.quantity = value;
        self
    }

    pub fn active_after(&self) -> &Option<DateTime<Utc>> {
        &self.active_after
    }

    pub fn set_active_after(mut self, value: Option<DateTime<Utc>>) -> Self {
        self.active_after = value;
        self
    }

    pub fn build(self) -> Result<Order, BuildOrderError> {
        Ok(
            Order {
                id: self.id.ok_or(BuildOrderError::UndefinedId)?,
                symbol_id: self.symbol_id,
                direction: self.direction,
                quantity: self.quantity,
                status: self.status,
                kind: self.kind,
                oca: self.oca,
                active_until: self.active_until,
                active_after: self.active_after
            }
        )
    }
}

#[derive(Debug)]
pub enum BuildOrderError {
    UndefinedId
}
