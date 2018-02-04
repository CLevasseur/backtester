use signal::Signal;
use order::{OrderBuilder, OrderKind};
use order::policy::{OrderPolicy, OrderPolicyError};

pub struct StopOrderPolicy {
    price: f64
}

impl StopOrderPolicy {
    pub fn new(price: f64) -> StopOrderPolicy {
        StopOrderPolicy { price }
    }
}

impl OrderPolicy for StopOrderPolicy {
    fn create_order(&self, signal: &Signal) -> Result<OrderBuilder, OrderPolicyError> {
        Ok(
            OrderBuilder::unallocated(
                OrderKind::StopOrder(self.price),
                signal.symbol_id().clone(),
                signal.direction().clone()
            )
        )
    }
}
