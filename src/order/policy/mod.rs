use signal::Signal;
use order::Order;
use symbol::SymbolOhlcvSource;

mod market_order_policy;
pub use order::policy::market_order_policy::MarketOrderPolicy;

pub trait OrderPolicy {
    fn create_order<'symbol, S: 'symbol + SymbolOhlcvSource>(&self, signal: &'symbol Signal<'symbol, S>) -> Box<Order<'symbol, S> + 'symbol>;
}

