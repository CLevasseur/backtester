use std::collections::{BTreeMap, HashMap};
use order::{Order, OrderId, OrderStatus};

#[derive(Debug, Clone, PartialEq)]
pub struct Portfolio {
    active_orders: BTreeMap<OrderId, Order>,
    closed_orders: BTreeMap<OrderId, Order>
}

impl Portfolio {

    pub fn new() -> Portfolio {
        Portfolio {
            active_orders: BTreeMap::new(),
            closed_orders: BTreeMap::new()
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.active_orders.insert(order.id().clone(), order);
    }

    pub fn add_orders(&mut self, orders: Vec<Order>) {
        for order in orders {
            self.add_order(order);
        }
    }

    fn move_order_to_closed_orders(&mut self, order_id: &OrderId, order_status: OrderStatus) {
        match self.active_orders.remove(order_id) {
            Some(mut order) => {
                order.set_status(order_status);
                match *order.status() {
                    OrderStatus::Filled(_) => {
                        self.closed_orders.insert(order_id.clone(), order);
                    },
                    OrderStatus::Cancelled(_) => {
                        self.closed_orders.insert(order_id.clone(), order);
                    },
                    _ => ()
                }
            },
            None => panic!("Order not found: {}", order_id)  // TODO: raise err
        };
    }

    fn update_order(&mut self, order_id: &OrderId, order_status: OrderStatus) {
        match order_status {
            OrderStatus::Filled(_) => self.move_order_to_closed_orders(order_id, order_status),
            OrderStatus::Cancelled(_) => self.move_order_to_closed_orders(order_id, order_status),
            _ => ()
        }
    }

    pub fn update_orders(&mut self, order_updates: &HashMap<OrderId, OrderStatus>) {
        for (updated_order_id, updated_order_status) in order_updates {
            self.update_order(&updated_order_id, updated_order_status.clone());
        }
    }

    pub fn active_orders(&self) -> &BTreeMap<OrderId, Order> {
        &self.active_orders
    }

    pub fn closed_orders(&self) -> &BTreeMap<OrderId, Order> {
        &self.closed_orders
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use direction::Direction;
    use symbol::SymbolId;
    use order::{OrderKind, OrderBuilder};

    #[test]
    fn add_order() {
        let symbol_id = SymbolId::from("Symbol");
        let order = OrderBuilder::unallocated(
            OrderKind::MarketOrder, symbol_id.clone(), Direction::Long
        ).build();
        let mut portfolio = Portfolio::new();
        assert!(portfolio.active_orders().is_empty());
        portfolio.add_order(order.clone());
        assert_eq!(
            portfolio.active_orders(),
            &[(order.id().clone(), order)].iter().cloned().collect::<BTreeMap<OrderId, Order>>()
        );
    }
}
