use symbol::{Symbol, SymbolOhlcvSource};
use direction::Direction;
use order::{OrderId, OrderStatus};


#[derive(Eq, PartialEq, Hash, Debug)]
pub struct BaseOrder<'symbol, Source: 'symbol + SymbolOhlcvSource> {
    pub id: OrderId,
    pub symbol: &'symbol Symbol<'symbol, Source>,
    pub direction: Direction,
    pub quantity: u32,
    pub status: OrderStatus
}
