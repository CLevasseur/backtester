use std::collections::HashMap;
use order::{Order, OrderId, OrderStatus};

#[derive(Debug, Clone, PartialEq)]
pub struct Portfolio {
    active_orders: HashMap<OrderId, Order>,
    closed_orders: HashMap<OrderId, Order>
}

impl Portfolio {

    pub fn new() -> Portfolio {
        Portfolio {
            active_orders: HashMap::new(),
            closed_orders: HashMap::new()
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.active_orders.insert(order.id().clone(), order);
    }

    pub fn update_order(&mut self, order_id: &OrderId, order_status: OrderStatus) {
        let mut order = self.active_orders.remove(order_id).unwrap();
        order.set_status(order_status);
        self.closed_orders.insert(order_id.clone(), order);
    }

    pub fn active_orders(&self) -> &HashMap<OrderId, Order> {
        &self.active_orders
    }

    pub fn closed_orders(&self) -> &HashMap<OrderId, Order> {
        &self.closed_orders
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use direction::Direction;
    use symbol::SymbolId;
    use ohlcv::source::NullOhlcvSource;
    use order::{OrderKind, OrderBuilder};

    #[test]
    fn add_order() {
        let symbol_id = SymbolId::from("Symbol");
        let order = OrderBuilder::unallocated(OrderKind::MarketOrder, symbol_id.clone(), Direction::Long).build();
        let mut portfolio = Portfolio::new();
        assert!(portfolio.active_orders().is_empty());
        portfolio.add_order(order.clone());
        assert_eq!(
            portfolio.active_orders(),
            &[(order.id().clone(), order)].iter().cloned().collect::<HashMap<OrderId, Order>>()
        );
    }
}
