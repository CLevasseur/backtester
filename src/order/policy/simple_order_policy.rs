extern crate chrono;
use self::chrono::prelude::{DateTime, Utc};
use signal::Signal;
use order::{OrderKind, OrderBuilder, OcaGroup};
use order::policy::{OrderPolicy, OrderPolicyError};

pub struct SimpleOrderPolicy {
    order_kind: OrderKind,
    oca: Option<OcaGroup>,
    active_until: Option<DateTime<Utc>>,
    active_after: Option<DateTime<Utc>>
}

impl SimpleOrderPolicy {
    pub fn new(order_kind: OrderKind) -> Self {
        SimpleOrderPolicy {
            order_kind: order_kind,
            oca: None,
            active_until: None,
            active_after: None
        }
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

    pub fn active_after(&self) -> &Option<DateTime<Utc>> {
        &self.active_after
    }

    pub fn set_active_after(mut self, value: Option<DateTime<Utc>>) -> Self {
        self.active_after = value;
        self
    }
}

impl OrderPolicy for SimpleOrderPolicy {
    fn create_order(&self, signal: &Signal) -> Result<OrderBuilder, OrderPolicyError> {
        Ok(
            OrderBuilder::unallocated(
                self.order_kind.clone(),
                signal.symbol_id().clone(),
                signal.direction().clone()
            )
                .set_active_after(self.active_after().clone())
                .set_active_until(self.active_until().clone())
                .set_oca(self.oca().clone())
        )
    }
}
