use signal::Signal;
use order::Order;

mod market_order_policy;
pub use order::policy::market_order_policy::MarketOrderPolicy;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum OrderPolicyError {
    IndicatorError
}

pub trait OrderPolicy {
    fn create_order(&self, signal: Signal) -> Result<Order, OrderPolicyError>;
}

