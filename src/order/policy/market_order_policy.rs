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
    fn create_order<'symbol>(&self, signal: Signal<'symbol>) -> Result<Order<'symbol>, OrderPolicyError> {
        Ok(OrderBuilder::unallocated(OrderKind::MarketOrder, signal.symbol(), signal.direction().clone()).build())
    }
}
