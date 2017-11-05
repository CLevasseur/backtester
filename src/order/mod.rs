mod order_id;
mod order_status;
mod base_order;
mod market_order;

pub mod policy;

pub use self::order_id::OrderId;
pub use self::order_status::OrderStatus;
pub use self::base_order::BaseOrder;
pub use self::market_order::MarketOrder;

use symbol::SymbolOhlcvSource;

//extern crate snowflake;

//use self::snowflake::ProcessUniqueId;
//use symbol::Symbol;
//use signal::Direction;

pub trait Order<'symbol, Source: 'symbol + SymbolOhlcvSource> {
    fn base(&self) -> &BaseOrder<'symbol, Source>;
    fn base_mut(&mut self) -> &mut BaseOrder<'symbol, Source>;
}



//pub struct LimitOrder<'symbol> {
    //base: BaseOrder<'symbol>,
    //limit_price: u32
//}
//impl<'symbol> Order<'symbol> for LimitOrder<'symbol> {
    //fn base(&self) -> &BaseOrder<'symbol> { &self.base }
    //fn base_mut(&mut self) -> &mut BaseOrder<'symbol> { &mut self.base }
//}

//pub struct StopOrder<'symbol> {
    //base: BaseOrder<'symbol>,
    //stop_price: u32
//}
//impl<'symbol> Order<'symbol> for StopOrder<'symbol> {
    //fn base(&self) -> &BaseOrder<'symbol> { &self.base }
    //fn base_mut(&mut self) -> &mut BaseOrder<'symbol> { &mut self.base }
//}

//impl<'symbol, T:Order<'symbol> + ?Sized> Order<'symbol> for Box<T> {
    //fn base(&self) -> &BaseOrder<'symbol> { (**self).base() }
    //fn base_mut(&mut self) -> &mut BaseOrder<'symbol> { (**self).base_mut() }
//}
////impl<'symbol, 'a, T: Order<'symbol> + ?Sized> Order<'symbol> for &'a T {
    ////fn base(&self) -> &BaseOrder<'symbol> { &*self.base() }
    ////fn base_mut(&mut self) -> &mut BaseOrder<'symbol> { &mut *self.base_mut() }
////}


//pub enum OrderStatus {
    //NotSent,
    //Sent,
    //PreSubmitted,
    //Submitted,
    //PartiallyFilled(u32, u32),
    //Filled(u32),
    //Cancelled(String)
//}

