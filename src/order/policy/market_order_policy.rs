use order::Order;
use signal::Signal;
use order::MarketOrder;
use order::policy::OrderPolicy;
use symbol::SymbolOhlcvSource;

pub struct MarketOrderPolicy {}

impl OrderPolicy for MarketOrderPolicy {
    fn create_order<'symbol, S: 'symbol + SymbolOhlcvSource>(&self, signal: &'symbol Signal<'symbol, S>) -> Box<Order<'symbol, S> + 'symbol> {
        Box::new(MarketOrder::unallocated(signal.symbol(), signal.direction().clone()))
    }
}


