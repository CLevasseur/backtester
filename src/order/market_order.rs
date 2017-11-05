extern crate chrono;

use direction::Direction;
use symbol::{Symbol, SymbolOhlcvSource};
use order::{Order, BaseOrder, OrderId, OrderStatus};

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct MarketOrder<'symbol, Source: 'symbol + SymbolOhlcvSource> {
    base: BaseOrder<'symbol, Source>
}

impl<'symbol, Source: 'symbol + SymbolOhlcvSource> MarketOrder<'symbol, Source> {
    pub fn unallocated<'a>(symbol: &'symbol Symbol<'symbol, Source>, direction: Direction) -> MarketOrder<'symbol, Source> {
        MarketOrder {
            base: BaseOrder {
                id: OrderId::new(),
                symbol: symbol,
                direction: direction,
                quantity: 0,
                status: OrderStatus::NotSent
            }
        }
    }
}

impl<'symbol, Source: 'symbol + SymbolOhlcvSource> Order<'symbol, Source> for MarketOrder<'symbol, Source> {
    fn base(&self) -> &BaseOrder<'symbol, Source> { &self.base }
    fn base_mut(&mut self) -> &mut BaseOrder<'symbol, Source> { &mut self.base }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohlcv::source::NullOhlcvSource;

    #[test]
    fn unallocated() {
        let source = NullOhlcvSource::new();
        let symbol = Symbol::new(String::from("Instrument"), &source); 
        let order = MarketOrder::unallocated(&symbol, Direction::Long);
        assert_eq!(
            order.base(),
            &BaseOrder {
                id: order.base().id,
                symbol: &symbol,
                direction: Direction::Long,
                quantity: 0,
                status: OrderStatus::NotSent
            }
        )
    }
}
