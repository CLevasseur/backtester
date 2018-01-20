extern crate chrono;

use std::collections::{HashMap, BTreeMap};
use self::chrono::prelude::{DateTime, Utc};
use model::Model;
use strategy::{Strategy, StrategyError, StrategyId};
use order::{Order, OrderId, OrderStatus};

pub enum StrategyType<'model> {
    EntryStrategy(StrategyId, &'model Model),
    ExitStrategy(StrategyId, &'model Model)
}

pub struct StrategyCollection<'model> {
    pub entry_strategies: Vec<Strategy>,
    pub exit_strategies: BTreeMap<StrategyId, Strategy>,
    pub order_strategy: HashMap<OrderId, StrategyId>,
    pub strategy_types: HashMap<StrategyId, StrategyType<'model>>
}

impl<'model> StrategyCollection<'model> {

    pub fn new() -> StrategyCollection<'model> {
        StrategyCollection {
            entry_strategies: vec![],
            exit_strategies: BTreeMap::new(),
            order_strategy: HashMap::new(),
            strategy_types: HashMap::new()
        }
    }

}

pub struct StrategyManager;

impl StrategyManager {

    pub fn new() -> StrategyManager {
        StrategyManager {}
    }

    /// Create a strategy collection with entry strategies from given models
    pub fn initialize_strategy_collection<'model>(&self, models: &'model Vec<Box<Model>>)
        -> StrategyCollection<'model>
    {
        let mut strategy_collection = StrategyCollection::new();

        for model in models {
            let entry_strategy = model.entry_strategy();
            strategy_collection.strategy_types.insert(
                entry_strategy.id().clone(),
                StrategyType::EntryStrategy(entry_strategy.id().clone(), model)
            );
            strategy_collection.entry_strategies.push(entry_strategy);
        }

        strategy_collection
    }

    /// Run all strategies of the collection at the specified date
    pub fn run_strategies(&self, strategies: &mut StrategyCollection,
                          datetime: &DateTime<Utc>) -> Result<Vec<Order>, StrategyError>
    {
        let mut orders = vec![];

        for strategy in strategies.entry_strategies.iter_mut().chain(strategies.exit_strategies.values_mut()) {
            let order = strategy.run(datetime)?;
            if let Some(o) = order {
                strategies.order_strategy.insert(o.id().clone(), strategy.id().clone());
                orders.push(o);
            }
        }

        Ok(orders)
    }

    /// Update strategies when an order is updated
    pub fn update_strategies(&self, strategy_collection: &mut StrategyCollection,
                             order_updates: &Vec<(&Order, OrderStatus)>)
    {
        for update in order_updates {
            match update.1 {
                OrderStatus::Filled(_) => {
                    self.update_exit_strategies(strategy_collection, update.0, &update.1);
                },
                OrderStatus::Cancelled(_) => {
                    self.update_exit_strategies(strategy_collection, update.0, &update.1);
                },
                _ => ()
            }
        }
    }

    /// Add exit strategies if an entry order is executed, remove exit strategy if its order
    /// is cancelled or executed
    fn update_exit_strategies<'model>(&self, strategies: &mut StrategyCollection<'model>,
                                      closed_order: &Order, order_status: &OrderStatus)
    {
        let strategy_updates;

        // Get strategy to add or remove
        {
            let strategy_id = strategies.order_strategy.get(closed_order.id()).unwrap();
            let strategy_type = strategies.strategy_types.get(strategy_id).unwrap();

            strategy_updates = match strategy_type {
                // Add exit strategies when an entry order is executed
                &StrategyType::EntryStrategy(_strategy_id, model) => {
                    match order_status {
                        &OrderStatus::Filled(_) => {
                            Some(StrategiesUpdate::AddExitStrategies(model.exit_strategies(closed_order), model))
                        },
                        _ => None
                    }
                },
                // Remove corresponding exit strategy when exit order is executed
                &StrategyType::ExitStrategy(_strategy_id, _model) => {
                    match order_status {
                        &OrderStatus::Filled(_) => {
                            Some(StrategiesUpdate::RemoveExitStrategy(*strategy_id))
                        },
                        &OrderStatus::Cancelled(_) => {
                            Some(StrategiesUpdate::RemoveExitStrategy(*strategy_id))
                        },
                        _ => None
                    }
                }
            }
        };

        // Apply strategy updates
        if let Some(updates) = strategy_updates {
            match updates {
                StrategiesUpdate::AddExitStrategies(new_strategies, model) => {
                    for strategy in new_strategies {
                        strategies.strategy_types.insert(
                            strategy.id().clone(),
                            StrategyType::ExitStrategy(strategy.id().clone(), model)
                        );
                        strategies.exit_strategies.insert(strategy.id().clone(), strategy);
                    }
                },
                StrategiesUpdate::RemoveExitStrategy(strategy_id) => {
                    strategies.exit_strategies.remove(&strategy_id);
                }
            }
        }
    }

}

enum StrategiesUpdate<'model> {
    AddExitStrategies(Vec<Strategy>, &'model Model),
    RemoveExitStrategy(StrategyId)
}

#[cfg(test)]
mod test {
    use super::*;
    use self::chrono::prelude::{TimeZone};
    use order::{Order, OrderBuilder, OrderKind, CancellationReason};
    use signal::Signal;
    use signal::detector::{DetectSignal, DetectSignalError};
    use symbol::SymbolId;
    use model::ModelId;
    use direction::Direction;
    use order::policy::MarketOrderPolicy;

    #[derive(Clone)]
    struct MockModel {
        symbol: SymbolId,
        err: bool
    }

    impl Model for MockModel {

        fn id(&self) -> ModelId {
            String::from("mock model")
        }

        fn entry_strategy(&self) -> Strategy {
            let signal: Box<DetectSignal>;
            if !self.err {
                signal = Box::new(SomeSignal { symbol: self.symbol.clone() });
            }
            else {
                signal = Box::new(SignalError {});
            }
            Strategy::new(signal, Box::new(MarketOrderPolicy::new()))
        }

        fn exit_strategies(&self, _order: &Order) -> Vec<Strategy> {
            vec![
                Strategy::new(Box::new(SomeSignal { symbol: self.symbol.clone() }), Box::new(MarketOrderPolicy::new())),
                Strategy::new(Box::new(SomeSignal { symbol: self.symbol.clone() }), Box::new(MarketOrderPolicy::new()))
            ]
        }

    }

    struct SomeSignal { symbol: SymbolId }
    impl DetectSignal for SomeSignal {
        fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
            Ok(Some(Signal::new(self.symbol.clone(), Direction::Long, datetime.clone(), String::new())))
        }
    }

    struct SignalError;
    impl DetectSignal for SignalError {
        fn detect_signal(&self, _datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
            Err(DetectSignalError::IndicatorError)
        }
    }

    /// Test that exit strategies are added to collection when the entry order is filled
    #[test]
    fn update_strategies_entry_order_filled() {
        let symbol_id = SymbolId::from("instrument");
        let model: Box<Model> = Box::new(MockModel { symbol: symbol_id.clone(), err: false });
        let strategy_manager = StrategyManager::new();
        let mut strategy_collection = StrategyCollection::new();
        let order_id = OrderId::new();
        strategy_collection.entry_strategies.push(model.entry_strategy());
        strategy_collection.order_strategy.insert(
            order_id.clone(),
            strategy_collection.entry_strategies.first().unwrap().id().clone()
        );
        strategy_collection.strategy_types.insert(
            strategy_collection.entry_strategies.first().unwrap().id().clone(),
            StrategyType::EntryStrategy(
                strategy_collection.entry_strategies.first().unwrap().id().clone(),
                &model
            )
        );
        strategy_manager.update_strategies(
            &mut strategy_collection,
            &vec![(
                &OrderBuilder::unallocated(OrderKind::MarketOrder, symbol_id.clone(), Direction::Long)
                    .id(order_id.clone()).build(),
                OrderStatus::Filled(1)
            )]
        );
        assert_eq!(strategy_collection.exit_strategies.len(), 2);
    }

    /// Test that exit strategy is removed from the collection when its order is filled
    #[test]
    fn update_strategies_exit_order_filled() {
        let symbol_id = SymbolId::from("instrument");
        let model: Box<Model> = Box::new(MockModel { symbol: symbol_id.clone(), err: false });
        let strategy_manager = StrategyManager::new();
        let mut strategy_collection = StrategyCollection::new();
        let order_id = OrderId::new();
        let entry_order = OrderBuilder::unallocated(OrderKind::MarketOrder, symbol_id.clone(), Direction::Long).build();
        let exit_strategy = model.exit_strategies(&entry_order).remove(0);
        strategy_collection.exit_strategies.insert(exit_strategy.id().clone(), exit_strategy);
        strategy_collection.order_strategy.insert(
            order_id.clone(),
            strategy_collection.exit_strategies.keys().next().unwrap().clone()
        );
        strategy_collection.strategy_types.insert(
            strategy_collection.exit_strategies.keys().next().unwrap().clone(),
            StrategyType::ExitStrategy(
                strategy_collection.exit_strategies.keys().next().unwrap().clone(),
                &model
            )
        );
        strategy_manager.update_strategies(
            &mut strategy_collection,
            &vec![(
                &OrderBuilder::unallocated(OrderKind::MarketOrder, symbol_id.clone(), Direction::Long)
                    .id(order_id.clone()).build(),
                OrderStatus::Filled(1)
            )]
        );
        assert_eq!(strategy_collection.exit_strategies.len(), 0);
    }

    /// Test that exit strategy is removed from the collection when its order is cancelled
    #[test]
    fn update_strategies_exit_order_cancelled() {
        let symbol_id = SymbolId::from("instrument");
        let model: Box<Model> = Box::new(MockModel { symbol: symbol_id.clone(), err: false });
        let strategy_manager = StrategyManager::new();
        let mut strategy_collection = StrategyCollection::new();
        let order_id = OrderId::new();
        let entry_order = OrderBuilder::unallocated(OrderKind::MarketOrder, symbol_id.clone(), Direction::Long).build();
        let exit_strategy = model.exit_strategies(&entry_order).remove(0);
        strategy_collection.exit_strategies.insert(exit_strategy.id().clone(), exit_strategy);
        strategy_collection.order_strategy.insert(
            order_id.clone(),
            strategy_collection.exit_strategies.keys().next().unwrap().clone()
        );
        strategy_collection.strategy_types.insert(
            strategy_collection.exit_strategies.keys().next().unwrap().clone(),
            StrategyType::ExitStrategy(
                strategy_collection.exit_strategies.keys().next().unwrap().clone(),
                &model
            )
        );
        strategy_manager.update_strategies(
            &mut strategy_collection,
            &vec![(
                &OrderBuilder::unallocated(OrderKind::MarketOrder, symbol_id.clone(), Direction::Long)
                    .id(order_id.clone()).build(),
                OrderStatus::Cancelled(CancellationReason::FilledOca)
            )]
        );
        assert_eq!(strategy_collection.exit_strategies.len(), 0);
    }

    #[test]
    fn run_strategies_ok() {
        let symbol = SymbolId::from("instrument");
        let models: Vec<Box<Model>> = vec![Box::new(MockModel { symbol: symbol.clone(), err: false })];
        let strategy_manager = StrategyManager::new();
        let mut strategy_collection = strategy_manager.initialize_strategy_collection(&models);
        let orders = strategy_manager.run_strategies(&mut strategy_collection, &Utc.ymd(2016, 1, 3).and_hms(17, 0, 0)).unwrap();
        assert!(orders.len() == 1);
        let expected = OrderBuilder::unallocated(
            OrderKind::MarketOrder,
            symbol.clone(),
            Direction::Long
        ).id(orders[0].id().clone()).build();

        assert!(orders[0] == expected);
    }

    #[test]
    fn run_strategies_err() {
        let symbol = SymbolId::from("symbol");
        let models: Vec<Box<Model>> = vec![Box::new(MockModel { symbol: symbol.clone(), err: true })];
        let strategy_manager = StrategyManager::new();
        let mut strategy_collection = strategy_manager.initialize_strategy_collection(&models);
        let orders = strategy_manager.run_strategies(&mut strategy_collection, &Utc.ymd(2016, 1, 3).and_hms(17, 0, 0));
        assert!(orders.is_err());
    }
}
