extern crate time;
extern crate chrono;

use model::Model;
use ohlcv::Ohlcv;
use market_simulation::MarketSimulation;
use portfolio::Portfolio;
use strategy::{StrategyManager, StrategyError};


pub struct Backtester {
    market_simulation: MarketSimulation,
    strategy_manager: StrategyManager
}

#[derive(Debug, Clone)]
pub enum BacktesterError {
    StrategyError(StrategyError)
}

impl Backtester {

    pub fn new() -> Self {
        Backtester {
            market_simulation: MarketSimulation::new(),
            strategy_manager: StrategyManager::new()
        }
    }

    pub fn run<I>(&self, models: &Vec<Box<Model>>, ohlcv: I) -> Result<Portfolio, BacktesterError>
        where I: Iterator<Item=Ohlcv>
    {
        let mut portfolio = Portfolio::new();
        let mut strategy_collection = self.strategy_manager.initialize_strategy_collection(models);

        for o in ohlcv {
            let updates = self.market_simulation.update_orders(portfolio.active_orders().values(), &o);
            self.strategy_manager.update_strategies(&mut strategy_collection, &portfolio.active_orders(), &updates);
            portfolio.update_orders(&updates);

            portfolio.add_orders(
                self.strategy_manager.run_strategies(&mut strategy_collection, o.datetime())
                    .map_err(|e| BacktesterError::StrategyError(e))?
            );
        }

        Ok(portfolio)
    }

    pub fn market_simulation(&self) -> &MarketSimulation {
        &self.market_simulation
    }

    pub fn set_market_simulation(&mut self, market_simulation: MarketSimulation) -> &Self {
        self.market_simulation = market_simulation;
        self
    }

    pub fn strategy_manager(&self) -> &StrategyManager {
        &self.strategy_manager
    }

    pub fn set_strategy_manager(&mut self, strategy_manager: StrategyManager) -> &Self {
        self.strategy_manager = strategy_manager;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate chrono;
    use self::chrono::prelude::{DateTime, Utc, TimeZone};
    use ohlcv::source::{OhlcvSource, CsvOhlcvSource};
    use backtester::Backtester;
    use model::{Model, ModelId};
    use strategy::Strategy;
    use signal::detector::{DetectSignal, DetectSignalError};
    use direction::Direction;
    use order::Order;
    use order::policy::MarketOrderPolicy;
    use symbol::SymbolId;
    use signal::Signal;
    use util::record_parser::RecordParser;


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
    fn test_run() {
        let symbol_id = SymbolId::from("eur/usd");
        let backtester = Backtester::new();
        let portfolio = backtester.run(
            &vec![Box::new(OrderEveryCandle { symbol_id: symbol_id.clone() })],
            vec![
                Ohlcv::new(SymbolId::from("eur/usd"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 0), 1., 1., 1., 1., 0),
                Ohlcv::new(SymbolId::from("usd/jpy"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 0), 1., 1., 1., 1., 0),
                Ohlcv::new(SymbolId::from("eur/usd"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 5), 1., 1.5, 1., 1.5, 3),
                Ohlcv::new(SymbolId::from("usd/jpy"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 5), 1., 1.5, 1., 1.5, 3)
            ].into_iter()
        ).unwrap();
    }

}
