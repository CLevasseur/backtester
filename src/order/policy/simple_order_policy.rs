use signal::Signal;
use order::OrderBuilder;
use order::policy::{OrderPolicy, OrderPolicyError};

pub struct SimpleOrderPolicy {
    order_builder: OrderBuilder
}

impl SimpleOrderPolicy {
    pub fn new(order_builder: OrderBuilder) -> SimpleOrderPolicy {
        SimpleOrderPolicy {
            order_builder
        }
    }
}

impl OrderPolicy for SimpleOrderPolicy {
    fn create_order(&self, _signal: &Signal) -> Result<OrderBuilder, OrderPolicyError> {
        Ok(self.order_builder.clone())
    }
}
