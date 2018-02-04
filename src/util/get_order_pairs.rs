extern crate chrono;

use self::chrono::prelude::{DateTime, Utc};
use portfolio::Portfolio;
use strategy::{StrategyCollection, StrategyType};
use order::{Order, OrderStatus};

#[derive(PartialEq, Debug)]
pub struct OrderPair<'a> {
    pub entry_order: &'a Order,
    pub exit_order: &'a Order
}

fn get_execution_datetime(order: &Order) -> DateTime<Utc> {
    match *order.status() {
        OrderStatus::Filled(ref execution) => execution.datetime().clone(),
        _ => panic!("Order is not executed")
    }
}

pub fn get_order_pairs<'a>(portfolio: &'a Portfolio,
                           strategy_collection: &StrategyCollection) -> Vec<OrderPair<'a>>
{
    let mut result = vec![];
    for (order_id, order) in portfolio.closed_orders().iter() {
        let strategy_id = strategy_collection.order_strategy.get(order_id)
            .expect(format!("Can't find strategy for order #{}", order.id().clone()).as_str());
        let strategy_type = strategy_collection.strategy_types.get(strategy_id)
            .expect(format!("Can't find strategy type for strategy #{}", strategy_id).as_str());

        if let &StrategyType::ExitStrategy(_strategy_id, _model, ref entry_order_id) = strategy_type {
            if let &OrderStatus::Filled(_) = order.status() {
                result.push(OrderPair {
                    entry_order: portfolio.closed_orders().get(entry_order_id).unwrap(),
                    exit_order: order
                });
            }
        }
    }

    result.sort_by_key(|order_pair| get_execution_datetime(order_pair.entry_order));
    result
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate chrono;
    use self::chrono::prelude::{DateTime, Utc, TimeZone};
    use order::{OrderId, OrderStatus};
    use execution::Execution;
    use direction::Direction;
    use symbol::SymbolId;
    use strategy::{Strategy, StrategyType, StrategyCollection};
    use model::{Model, ModelId};
    use signal::detector::{DetectSignal, DetectSignalError};
    use order::policy::MarketOrderPolicy;
    use signal::Signal;


    pub struct AlwaysDetectSignal {
        symbol_id: SymbolId
    }

    impl DetectSignal for AlwaysDetectSignal {
        fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
            let signal = Signal::new(
                self.symbol_id.clone(),
                Direction::Long,
                datetime.clone(),
                String::from("always detect signal")
            );
            Ok(Some(signal))
        }
    }

    pub struct OrderEveryCandle {
        symbol_id: SymbolId
    }

    impl Model for OrderEveryCandle {

        fn id(&self) -> ModelId {
            ModelId::from("order every candle")
        }

        fn entry_strategy(&self) -> Strategy {
            Strategy::new(
                Box::new(AlwaysDetectSignal { symbol_id: self.symbol_id.clone() }),
                Box::new(MarketOrderPolicy::new())
            )
        }

        fn exit_strategies(&self, _order: &Order) -> Vec<Strategy> {
            let strategy = Strategy::new(
                Box::new(AlwaysDetectSignal { symbol_id: self.symbol_id.clone() }),
                Box::new(MarketOrderPolicy::new())
            );

            vec![strategy]
        }

    }
    #[test]
    fn test_get_order_pairs() {
        let mut portfolio = Portfolio::new();
        let symbol_id = SymbolId::from("eur/usd");
        let datetime = Utc.ymd(2017, 1, 1).and_hms(14, 0, 0);
        let model = OrderEveryCandle {symbol_id: symbol_id.clone()};
        let entry_strategy = model.entry_strategy();
        let entry_order_id = OrderId::from("entry order");
        let entry_order = entry_strategy.run(&datetime).unwrap().unwrap().1
            .set_id(entry_order_id.clone())
            .set_status(
                OrderStatus::Filled(
                    Execution::new(
                        SymbolId::from("eur/usd"),
                        0, 1., datetime.clone()
                    )
                )
            )
            .build()
            .expect("failed to create entry order");
        let exit_strategy = model.exit_strategies(&entry_order).remove(0);
        let exit_order_id = OrderId::from("exit order");
        let exit_order = exit_strategy.run(&datetime).unwrap().unwrap().1
            .set_id(exit_order_id.clone())
            .set_status(
                OrderStatus::Filled(
                    Execution::new(
                        SymbolId::from("eur/usd"),
                        0, 1., datetime.clone()
                    )
                )
            )
            .build()
            .expect("failed to create exit order");

        portfolio.add_orders(vec![entry_order, exit_order]);

        let mut strategy_collection = StrategyCollection::new();
        strategy_collection.order_strategy.insert(
            entry_order_id.clone(),
            entry_strategy.id().clone(),
        );
        strategy_collection.strategy_types.insert(
            entry_strategy.id().clone(),
            StrategyType::EntryStrategy(entry_strategy.id().clone(), &model)
        );

        strategy_collection.order_strategy.insert(
            exit_order_id.clone(),
            exit_strategy.id().clone(),
        );
        strategy_collection.strategy_types.insert(
            exit_strategy.id().clone(),
            StrategyType::ExitStrategy(
                entry_strategy.id().clone(), &model, entry_order_id.clone()
            )
        );

        assert_eq!(
            get_order_pairs(&portfolio, &strategy_collection),
            vec![
                OrderPair {
                    entry_order: portfolio.closed_orders().get(&entry_order_id).unwrap(),
                    exit_order: portfolio.closed_orders().get(&exit_order_id).unwrap()
                }
            ]
        );
    }
}