extern crate time;
extern crate chrono;

use self::chrono::prelude::{DateTime, Utc, TimeZone};
use model::Model;
use ohlcv::Ohlcv;
use market_simulation::MarketSimulation;
use portfolio::Portfolio;
use strategy::StrategyManager;


pub struct Backtester {
    market_simulation: MarketSimulation
}

impl Backtester {

    pub fn new(market_simulation: MarketSimulation) -> Self {
        Backtester {
            market_simulation: market_simulation
        }
    }

    pub fn run(&self, models: &Vec<Box<Model>>,
               start_date: DateTime<Utc>,
               end_date: DateTime<Utc>)
    {
        let mut current_date = start_date.clone();
        let mut portfolio = Portfolio::new();

        let strategy_manager = StrategyManager::new();
        let mut strategy_collection = strategy_manager.initialize_strategy_collection(models);

        while current_date < end_date {
            let ohlcv = Ohlcv::new(Utc.ymd(2017, 12, 13).and_hms(14, 0, 0), 1., 1., 1., 1., 4);
            let updates = self.market_simulation.update_orders(portfolio.active_orders().values(), &ohlcv);
            for (updated_order_id, updated_order_status) in updates {
                strategy_manager.update_exit_strategies(
                    &mut strategy_collection,
                    &portfolio.active_orders().get(&updated_order_id).unwrap()
                );
                portfolio.update_order(&updated_order_id, updated_order_status);
            }
                    
            let orders = strategy_manager.run_strategies(&mut strategy_collection, &current_date).unwrap();
            for order in orders {
                portfolio.add_order(order);
            }
            current_date = current_date + time::Duration::minutes(1);
        }
    }
}
