use signal::Signal;
use order::{Order, OrderBuilder, OrderKind};
use order::policy::{OrderPolicy, OrderPolicyError};

pub struct MarketOrderPolicy {}

impl MarketOrderPolicy {
    pub fn new() -> MarketOrderPolicy {
        MarketOrderPolicy {}
    }
}

impl OrderPolicy for MarketOrderPolicy {
    fn create_order(&self, signal: Signal) -> Result<Order, OrderPolicyError> {
        Ok(OrderBuilder::unallocated(
            OrderKind::MarketOrder,
            signal.symbol_id().clone(),
            signal.direction().clone()
        ).build())
    }
}
