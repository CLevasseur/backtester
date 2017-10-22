//extern crate snowflake;

//use self::snowflake::ProcessUniqueId;
//use symbol::Symbol;
//use signal::Direction;

//#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
//pub struct OrderId(ProcessUniqueId);

//pub trait Order<'symbol> {
    //fn base(&self) -> &BaseOrder<'symbol>;
    //fn base_mut(&mut self) -> &mut BaseOrder<'symbol>;
//}

//pub struct BaseOrder<'symbol> {
    //id: OrderId,
    //symbol: &'symbol Symbol,
    //direction: Direction,
    //quantity: u32,
    //status: OrderStatus
//}

//impl<'symbol> BaseOrder<'symbol> {

    //pub fn id(&self) -> &OrderId {
        //&self.id
    //}

    //pub fn quantity(&self) -> u32 {
        //self.quantity
    //}

    //pub fn set_status(&mut self, status: OrderStatus) {
        ////self.status = status;
    //}

//}


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

