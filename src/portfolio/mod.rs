use order::Order;

pub struct Portfolio<'symbol> {
    orders: Vec<Order<'symbol>>
}

impl<'symbol> Portfolio<'symbol> {

    pub fn new() -> Portfolio<'symbol> {
        Portfolio {
            orders: vec![]
        }
    }

    pub fn add_order(&mut self, order: Order<'symbol>) {
        self.orders.push(order);
    }

    pub fn orders(&self) -> &Vec<Order<'symbol>> {
        &self.orders
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use direction::Direction;
    use symbol::Symbol;
    use ohlcv::source::NullOhlcvSource;
    use order::{OrderKind, OrderBuilder};

    #[test]
    fn add_order() {
        let source = NullOhlcvSource::new();
        let symbol = Symbol::new(String::from("symbol"), &source);
        let order = OrderBuilder::unallocated(OrderKind::MarketOrder, &symbol, Direction::Long).build();
        let mut portfolio = Portfolio::new();
        assert!(portfolio.orders.is_empty());
        portfolio.add_order(order.clone());
        assert!(portfolio.orders == vec![order]);
    }

}
