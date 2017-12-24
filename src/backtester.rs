extern crate time;
extern crate chrono;

use std::collections::HashMap;
use self::chrono::prelude::{DateTime, Utc, TimeZone, Timelike};
use model::Model;
use symbol::SymbolId;
use ohlcv::Ohlcv;
use ohlcv::source::{OhlcvSource, OhlcvSourceError};
use market_simulation::MarketSimulation;
use portfolio::Portfolio;
use strategy::{StrategyManager, StrategyError};


pub struct Backtester<'a> {
    market_simulation: MarketSimulation,
    symbol_sources: HashMap<SymbolId, &'a OhlcvSource>
}

#[derive(Debug, Clone)]
pub enum BacktesterError {
    OhlcvSourceError(OhlcvSourceError),
    StrategyError(StrategyError)
}

impl<'a> Backtester<'a> {

    pub fn new(market_simulation: MarketSimulation, symbol_sources: HashMap<SymbolId, &'a OhlcvSource>) -> Self {
        Backtester {
            market_simulation: market_simulation,
            symbol_sources: symbol_sources
        }
    }

    fn get_ohlcv(&self, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> Result<Vec<(&SymbolId, Ohlcv)>, OhlcvSourceError> {
        let mut ohlcv = vec![];

        for (symbol_id, source) in &self.symbol_sources {
            ohlcv.append(&mut source.ohlcv(start_date, end_date)?.iter().map(|ohlcv| (symbol_id, *ohlcv)).collect());
        }
        //ohlcv.sort_unstable_by_key(get_datetime);
        ohlcv.sort_unstable_by_key(|k| *k.1.datetime());
        Ok(ohlcv)
    }

    pub fn run(&self, models: &Vec<Box<Model>>,
               start_date: &DateTime<Utc>,
               end_date: &DateTime<Utc>) -> Result<Portfolio, BacktesterError>
    {
        let mut current_date = start_date.clone();
        let mut portfolio = Portfolio::new();

        let strategy_manager = StrategyManager::new();
        let mut strategy_collection = strategy_manager.initialize_strategy_collection(models);

        let all_ohlcv = self.get_ohlcv(start_date, end_date);
        if let Err(e) = all_ohlcv {
            return Err(BacktesterError::OhlcvSourceError(e))
        }

        for (symbol_id, ohlcv) in all_ohlcv.unwrap() {
            let updates = self.market_simulation.update_orders(portfolio.active_orders().values(), symbol_id, &ohlcv);
            for (updated_order_id, updated_order_status) in updates {
                strategy_manager.update_exit_strategies(
                    &mut strategy_collection,
                    &portfolio.active_orders().get(&updated_order_id).unwrap()
                );
                portfolio.update_order(&updated_order_id, updated_order_status);
            }
                    
            match strategy_manager.run_strategies(&mut strategy_collection, &current_date) {
                Ok(orders) => {
                    for order in orders {
                        portfolio.add_order(order);
                    }
                },
                Err(e) => return Err(BacktesterError::StrategyError(e))
            }

            current_date = current_date + time::Duration::minutes(1);
        }

        Ok(portfolio)
    }
}
