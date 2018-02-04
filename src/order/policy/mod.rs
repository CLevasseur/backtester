use signal::Signal;
use order::OrderBuilder;

mod market_order_policy;
pub use order::policy::market_order_policy::MarketOrderPolicy;
mod simple_order_policy;
pub use order::policy::simple_order_policy::SimpleOrderPolicy;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum OrderPolicyError {
    IndicatorError
}

pub trait OrderPolicy {
    fn create_order(&self, signal: &Signal) -> Result<OrderBuilder, OrderPolicyError>;
}

