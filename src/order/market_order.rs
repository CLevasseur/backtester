//use order::BaseOrder;

//pub struct MarketOrder<'symbol> {
    //base: BaseOrder<'symbol>
//}

//impl<'a> MarketOrder<'a> {
    //pub fn unallocated(symbol: &Symbol, direction: Direction) -> MarketOrder {
        //MarketOrder {
            //base: BaseOrder {
                //id: OrderId(ProcessUniqueId::new()),
                //symbol: symbol,
                //direction: direction,
                //quantity: 0,
                //status: OrderStatus::NotSent
            //}
        //}
    //}
//}

//impl<'symbol> Order<'symbol> for MarketOrder<'symbol> {
    //fn base(&self) -> &BaseOrder<'symbol> { &self.base }
    //fn base_mut(&mut self) -> &mut BaseOrder<'symbol> { &mut self.base }
//}
