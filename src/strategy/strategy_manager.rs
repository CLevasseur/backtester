extern crate chrono;

use std::collections::HashMap;
use self::chrono::prelude::{DateTime, Utc};
use model::Model;
use strategy::{Strategy, StrategyError, StrategyId};
use order::{Order, OrderId};

pub enum StrategyType<'model> {
    EntryStrategy(StrategyId, &'model Model),
    ExitStrategy(StrategyId, &'model Model)
}

pub struct StrategyCollection<'model> {
    pub entry_strategies: Vec<Strategy>,
    pub exit_strategies: HashMap<StrategyId, Strategy>,
    pub order_strategy: HashMap<OrderId, StrategyId>,
    pub strategy_types: HashMap<StrategyId, StrategyType<'model>>
}

impl<'model> StrategyCollection<'model> {

    pub fn new() -> StrategyCollection<'model> {
        StrategyCollection {
            entry_strategies: vec![],
            exit_strategies: HashMap::new(),
            order_strategy: HashMap::new(),
            strategy_types: HashMap::new()
        }
    }

}

enum StrategiesUpdate<'model> {
    AddExitStrategies(Vec<Strategy>, &'model Model),
    RemoveExitStrategy(StrategyId)
}

pub struct StrategyManager;

impl StrategyManager {

    pub fn new() -> StrategyManager {
        StrategyManager {}
    }

    pub fn initialize_strategy_collection<'model>(&self, models: &'model Vec<Box<Model>>) -> StrategyCollection<'model> {
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

    pub fn update_exit_strategies<'model>(&self, 
                                          strategies: &mut StrategyCollection<'model>,
                                          closed_order: &Order)
    {
        let strategy_updates;
        {
            let strategy_id = strategies.order_strategy.get(closed_order.id()).unwrap();
            let strategy_type = strategies.strategy_types.get(strategy_id).unwrap();

            strategy_updates = match strategy_type {
                &StrategyType::EntryStrategy(_strategy_id, model) => {
                    StrategiesUpdate::AddExitStrategies(model.exit_strategies(closed_order), model)
                },
                &StrategyType::ExitStrategy(_strategy_id, _model) => {
                    StrategiesUpdate::RemoveExitStrategy(*strategy_id)
                }
            }
        };

        match strategy_updates {
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

}

#[cfg(test)]
mod test {
    use super::*;
    use self::chrono::prelude::{TimeZone};
    use order::{Order, OrderBuilder, OrderKind};
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
            vec![]
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
