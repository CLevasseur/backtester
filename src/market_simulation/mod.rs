extern crate chrono;
use std::collections::{HashSet, HashMap};
use ohlcv::Ohlcv;
use direction::Direction;
use order::{Order, OrderId, OrderStatus, OrderKind, OcaGroup, CancellationReason};

pub struct MarketSimulation;

impl MarketSimulation {
    pub fn new() -> MarketSimulation {
        MarketSimulation {}
    }

    pub fn update_orders<'a, I>(&self, orders: I, ohlcv: &Ohlcv) -> HashMap<OrderId, OrderStatus> where I: Iterator<Item=&'a Order> {
        let mut updates: HashMap<OrderId, OrderStatus> = HashMap::new();
        let mut filled_oca_groups: HashSet<OcaGroup> = HashSet::new();
        let mut oca_orders: HashMap<OcaGroup, Vec<&Order>> = HashMap::new();

        for order in orders {
            if let Some(oca_group) = order.oca() {
                if filled_oca_groups.contains(&oca_group) {
                    updates.insert(order.id().clone(), OrderStatus::Cancelled(CancellationReason::FilledOca));
                    continue;
                }

                oca_orders.entry(oca_group).or_insert(vec![]).push(&order);
            }

            if let Some(active_until) = order.active_until() {
                if &active_until <= ohlcv.datetime() {
                    updates.insert(order.id().clone(), OrderStatus::Cancelled(CancellationReason::OutdatedOrder));
                    continue;
                }
            }

            if let Some(active_after) = order.active_after() {
                if &active_after > ohlcv.datetime() {
                    continue;
                }
            }

            let is_executed = match *order.kind() {
                OrderKind::MarketOrder => true,
                OrderKind::LimitOrder(price) => match *order.direction() {
                    Direction::Long => ohlcv.low() < price,
                    Direction::Short => ohlcv.high() > price
                },
                OrderKind::StopOrder(price) => match *order.direction() {
                    Direction::Long => ohlcv.high() > price,
                    Direction::Short => ohlcv.low() < price
                }
            };

            if is_executed {
                if let Some(oca_group) = order.oca() {
                    filled_oca_groups.insert(oca_group);
                    if let Some(filled_oca_orders) = oca_orders.get(&oca_group) {
                        for cancelled_order in filled_oca_orders {
                            updates.insert(cancelled_order.id().clone(), OrderStatus::Cancelled(CancellationReason::FilledOca));
                        }
                    }
                }
                updates.insert(order.id().clone(), OrderStatus::Filled(order.quantity()));
            }
        }
        updates
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use self::chrono::prelude::{Utc, TimeZone};
    use order::{OrderBuilder, OrderKind};
    use symbol::SymbolId;
    use direction::Direction;

    #[test]
    fn update_market_order() {
        let market_simulation = MarketSimulation::new();
        let symbol = SymbolId::from("Symbol");
        let order = OrderBuilder::unallocated(OrderKind::MarketOrder, symbol.clone(), Direction::Long).build();
        let ohlcv = Ohlcv::new(Utc.ymd(2016, 1, 3).and_hms(17, 0, 0), 0., 0., 0., 0., 1);
        let updates = market_simulation.update_orders(vec![&order].into_iter(), &ohlcv);
        let expected: HashMap<OrderId, OrderStatus> = [(order.id().clone(), OrderStatus::Filled(order.quantity()))].iter().cloned().collect();
        assert!(updates == expected);
    }

    #[test]
    fn update_limit_long_order() {
        let market_simulation = MarketSimulation::new();
        let symbol = SymbolId::from("Symbol");
        let executed_order = OrderBuilder::unallocated(OrderKind::LimitOrder(100.), symbol.clone(), Direction::Long).build();
        let not_executed_order = OrderBuilder::unallocated(OrderKind::LimitOrder(99.), symbol.clone(), Direction::Long).build();
        let ohlcv = Ohlcv::new(Utc.ymd(2016, 1, 3).and_hms(17, 0, 0), 0., 0., 99., 0., 1);
        let updates = market_simulation.update_orders(vec![&executed_order, &not_executed_order].into_iter(), &ohlcv);
        let expected: HashMap<OrderId, OrderStatus> = [(executed_order.id().clone(), OrderStatus::Filled(executed_order.quantity()))].iter().cloned().collect();
        assert!(updates == expected);
    }

    #[test]
    fn update_limit_short_order() {
        let market_simulation = MarketSimulation::new();
        let symbol = SymbolId::from("Symbol");
        let executed_order = OrderBuilder::unallocated(OrderKind::LimitOrder(100.), symbol.clone(), Direction::Short).build();
        let not_executed_order = OrderBuilder::unallocated(OrderKind::LimitOrder(101.), symbol.clone(), Direction::Short).build();
        let ohlcv = Ohlcv::new(Utc.ymd(2016, 1, 3).and_hms(17, 0, 0), 0., 101., 0., 0., 1);
        let updates = market_simulation.update_orders(vec![&executed_order, &not_executed_order].into_iter(), &ohlcv);
        let expected: HashMap<OrderId, OrderStatus> = [(executed_order.id().clone(), OrderStatus::Filled(executed_order.quantity()))].iter().cloned().collect();
        assert!(updates == expected);
    }

    #[test]
    fn update_stop_long_order() {
        let market_simulation = MarketSimulation::new();
        let symbol = SymbolId::from("Symbol");
        let executed_order = OrderBuilder::unallocated(OrderKind::StopOrder(100.), symbol.clone(), Direction::Long).build();
        let not_executed_order = OrderBuilder::unallocated(OrderKind::StopOrder(101.), symbol.clone(), Direction::Long).build();
        let ohlcv = Ohlcv::new(Utc.ymd(2016, 1, 3).and_hms(17, 0, 0), 0., 101., 0., 0., 1);
        let updates = market_simulation.update_orders(vec![&executed_order, &not_executed_order].into_iter(), &ohlcv);
        let expected: HashMap<OrderId, OrderStatus> = [(executed_order.id().clone(), OrderStatus::Filled(executed_order.quantity()))].iter().cloned().collect();
        assert!(updates == expected);
    }

    #[test]
    fn update_stop_short_order() {
        let market_simulation = MarketSimulation::new();
        let symbol = SymbolId::from("Symbol");
        let executed_order = OrderBuilder::unallocated(OrderKind::StopOrder(100.), symbol.clone(), Direction::Short).build();
        let not_executed_order = OrderBuilder::unallocated(OrderKind::StopOrder(99.), symbol.clone(), Direction::Short).build();
        let ohlcv = Ohlcv::new(Utc.ymd(2016, 1, 3).and_hms(17, 0, 0), 0., 0., 99., 0., 1);
        let updates = market_simulation.update_orders(vec![&executed_order, &not_executed_order].into_iter(), &ohlcv);
        let expected: HashMap<OrderId, OrderStatus> = [(executed_order.id().clone(), OrderStatus::Filled(executed_order.quantity()))].iter().cloned().collect();
        assert!(updates == expected);
    }

    #[test]
    fn update_oca() {
        let market_simulation = MarketSimulation::new();
        let symbol = SymbolId::from("Symbol");
        let not_executed_order_1 = OrderBuilder::unallocated(OrderKind::LimitOrder(100.), symbol.clone(), Direction::Short).oca(0).build();
        let executed_order = OrderBuilder::unallocated(OrderKind::MarketOrder, symbol.clone(), Direction::Short).oca(0).build();
        let not_executed_order_2 = OrderBuilder::unallocated(OrderKind::StopOrder(98.), symbol.clone(), Direction::Short).oca(0).build();
        let ohlcv = Ohlcv::new(Utc.ymd(2016, 1, 3).and_hms(17, 0, 0), 0., 0., 99., 0., 1);
        let updates = market_simulation.update_orders(vec![&not_executed_order_1, &executed_order, &not_executed_order_2].into_iter(), &ohlcv);
        let expected: HashMap<OrderId, OrderStatus> = [
            (not_executed_order_1.id().clone(), OrderStatus::Cancelled(CancellationReason::FilledOca)),
            (executed_order.id().clone(), OrderStatus::Filled(executed_order.quantity())),
            (not_executed_order_2.id().clone(), OrderStatus::Cancelled(CancellationReason::FilledOca))
        ].iter().cloned().collect();
        println!("updates = {:?}", updates);
        assert!(updates == expected);
    }

    #[test]
    fn update_outdated_order() {
        let market_simulation = MarketSimulation::new();
        let symbol = SymbolId::from("Symbol");
        let executed_order = OrderBuilder::unallocated(OrderKind::StopOrder(100.), symbol.clone(), Direction::Short).build();
        let cancelled_order = OrderBuilder::unallocated(OrderKind::StopOrder(100.), symbol.clone(), Direction::Short)
            .active_until(Utc.ymd(2016, 1, 3).and_hms(16, 59, 59))
            .build();
        let ohlcv = Ohlcv::new(Utc.ymd(2016, 1, 3).and_hms(17, 0, 0), 0., 0., 99., 0., 1);
        let updates = market_simulation.update_orders(vec![&executed_order, &cancelled_order].into_iter(), &ohlcv);
        let expected: HashMap<OrderId, OrderStatus> = [
            (executed_order.id().clone(), OrderStatus::Filled(executed_order.quantity())),
            (cancelled_order.id().clone(), OrderStatus::Cancelled(CancellationReason::OutdatedOrder)),
        ].iter().cloned().collect();
        assert!(updates == expected);
    }

    #[test]
    fn update_inactive_order() {
        let market_simulation = MarketSimulation::new();
        let symbol = SymbolId::from("Symbol");
        let executed_order = OrderBuilder::unallocated(OrderKind::StopOrder(100.), symbol.clone(), Direction::Short).build();
        let not_executed_order = OrderBuilder::unallocated(OrderKind::StopOrder(100.), symbol.clone(), Direction::Short)
            .active_after(Utc.ymd(2016, 1, 3).and_hms(17, 0, 1))
            .build();
        let ohlcv = Ohlcv::new(Utc.ymd(2016, 1, 3).and_hms(17, 0, 0), 0., 0., 99., 0., 1);
        let updates = market_simulation.update_orders(vec![&executed_order, &not_executed_order].into_iter(), &ohlcv);
        let expected: HashMap<OrderId, OrderStatus> = [
            (executed_order.id().clone(), OrderStatus::Filled(executed_order.quantity()))
        ].iter().cloned().collect();
        assert!(updates == expected);
    }
}
