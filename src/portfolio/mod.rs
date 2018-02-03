use std::collections::HashMap;
use order::{Order, OrderId, OrderStatus};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

    pub fn add_orders(&mut self, orders: Vec<Order>) {
        for order in orders {
            let mut property = match *order.status() {
                OrderStatus::Filled(_) => &mut self.closed_orders,
                OrderStatus::Cancelled(_) => &mut self.closed_orders,
                _ => &mut self.active_orders
            };
            assert_eq!(property.insert(order.id().clone(), order), None);
        }
    }

    pub fn update_orders(&mut self, order_updates: &HashMap<OrderId, OrderStatus>) {
        for (updated_order_id, updated_order_status) in order_updates {
            match *updated_order_status {
                OrderStatus::Filled(_) => self.move_active_order_to_closed_orders(updated_order_id, updated_order_status.clone()),
                OrderStatus::Cancelled(_) => self.move_active_order_to_closed_orders(updated_order_id, updated_order_status.clone()),
                _ => ()
            }
        }
    }

    fn move_active_order_to_closed_orders(&mut self, order_id: &OrderId, order_status: OrderStatus) {
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
            None => panic!("Order not found: {}", order_id)
        };
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
    use order::{OrderKind, OrderBuilder};

    #[test]
    fn add_order() {
        let symbol_id = SymbolId::from("Symbol");
        let order = OrderBuilder::unallocated(
            OrderKind::MarketOrder, symbol_id.clone(), Direction::Long
        ).set_id(OrderId::from("test order")).build().unwrap();
        let mut portfolio = Portfolio::new();
        assert!(portfolio.active_orders().is_empty());
        portfolio.add_orders(vec![order.clone()]);
        assert_eq!(
            portfolio.active_orders(),
            &[(order.id().clone(), order)].iter().cloned().collect::<HashMap<OrderId, Order>>()
        );
    }
}
