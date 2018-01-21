extern crate time;
extern crate chrono;

use model::Model;
use ohlcv::Ohlcv;
use market_simulation::MarketSimulation;
use portfolio::Portfolio;
use strategy::{StrategyManager, StrategyError};
use order::OrderIdGenerator;


pub struct Backtester {
    market_simulation: MarketSimulation,
    strategy_manager: StrategyManager,
    order_id_generator: OrderIdGenerator
}

#[derive(Debug, Clone)]
pub enum BacktesterError {
    StrategyError(StrategyError)
}

// TODO: need start/end date on ohlcv
impl Backtester {

    pub fn new() -> Self {
        Backtester {
            market_simulation: MarketSimulation::new(),
            strategy_manager: StrategyManager::new(),
            order_id_generator: OrderIdGenerator::new()
        }
    }

    pub fn run<I>(&self, models: &Vec<Box<Model>>, ohlcv: I) -> Result<Portfolio, BacktesterError>
        where I: Iterator<Item=Ohlcv>
    {
        let mut portfolio = Portfolio::new();
        let mut strategy_collection = self.strategy_manager.initialize_strategy_collection(models);
        let mut previous_datetime = None;

        for o in ohlcv {
            let updates = self.market_simulation.update_orders(portfolio.active_orders().values(), &o);

            self.strategy_manager.update_strategies(
                &mut strategy_collection,
                &updates.iter().map(
                    |update| (portfolio.active_orders().get(update.0).unwrap(), update.1.clone())
                ).collect()
            );
            portfolio.update_orders(&updates);

            // run strategies only once per date, multiple ohlcv can have the same datetime
            // when there is more than one symbol
            match previous_datetime {
                Some(ref datetime) if datetime == o.datetime() => (),
                _ => {
                    portfolio.add_orders(
                        self.strategy_manager.run_strategies(
                            &mut strategy_collection, o.datetime(), &self.order_id_generator
                        )
                            .map_err(|e| BacktesterError::StrategyError(e))?
                            .into_iter().map(|order_builder| order_builder.build().unwrap())
                            .collect()
                    );
                }
            }

            previous_datetime = Some(*o.datetime());
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
    use backtester::Backtester;
    use model::{Model, ModelId};
    use strategy::Strategy;
    use signal::detector::{DetectSignal, DetectSignalError};
    use direction::Direction;
    use order::{Order, OrderBuilder, OrderKind, OrderStatus};
    use order::policy::MarketOrderPolicy;
    use execution::Execution;
    use symbol::SymbolId;
    use signal::Signal;

    pub struct AlwaysDetectSignal { direction: Direction }
    impl DetectSignal for AlwaysDetectSignal {
        fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
            let signal = Signal::new(
                SymbolId::from("eur/usd"), self.direction,
                datetime.clone(), String::from("always detect signal")
            );
            Ok(Some(signal))
        }
    }

    pub struct OrderEveryCandle;
    impl Model for OrderEveryCandle {

        fn id(&self) -> ModelId { ModelId::from("order every candle") }

        fn entry_strategy(&self) -> Strategy {
            Strategy::new(
                Box::new(AlwaysDetectSignal { direction: Direction::Long }),
                Box::new(MarketOrderPolicy::new())
            )
        }

        fn exit_strategies(&self, _order: &Order) -> Vec<Strategy> {
            vec![
                Strategy::new(
                    Box::new(AlwaysDetectSignal { direction: Direction::Short }),
                    Box::new(MarketOrderPolicy::new())
                )
            ]
        }

    }

    #[test]
    fn test_run() {
        let backtester = Backtester::new();
        let portfolio = backtester.run(
            &vec![Box::new(OrderEveryCandle {})],
            vec![
                Ohlcv::new(SymbolId::from("eur/usd"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 0), 1., 1., 1., 1., 0),
                Ohlcv::new(SymbolId::from("usd/jpy"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 0), 1., 1., 1., 1., 0),
                Ohlcv::new(SymbolId::from("eur/usd"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 5), 1., 1.5, 1., 1.5, 3),
                Ohlcv::new(SymbolId::from("usd/jpy"), Utc.ymd(2017, 12, 29).and_hms(12, 0, 5), 1., 1.5, 1., 1.5, 3)
            ].into_iter()
        ).unwrap();

        let active_orders = portfolio.active_orders().values().collect::<Vec<&Order>>();
        println!("Active orders: {:#?}", &active_orders);
        assert_eq!(active_orders.len(), 2);
        let closed_orders = portfolio.closed_orders().values().collect::<Vec<&Order>>();
        println!("Closed orders: {:#?}", &closed_orders);
        assert_eq!(closed_orders.len(), 1);

        let expected_active_orders: Vec<Order> = vec![
            // Long order from the second detection made by the entry strategy
            OrderBuilder::unallocated(
                OrderKind::MarketOrder, SymbolId::from("eur/usd"), Direction::Long
            ).set_id(active_orders[0].id().clone()).build().unwrap(),
            // Short order from the exit strategy linked to the first entry order
            OrderBuilder::unallocated(
                OrderKind::MarketOrder, SymbolId::from("eur/usd"), Direction::Short
            ).set_id(active_orders[1].id().clone()).build().unwrap()
        ];
        let expected_closed_orders: Vec<Order> = vec![
            // First entry order has been filled
            OrderBuilder::unallocated(
                OrderKind::MarketOrder, SymbolId::from("eur/usd"), Direction::Long
            )
                .set_id(closed_orders[0].id().clone())
                .set_status(
                    OrderStatus::Filled(
                        Execution::new(
                            SymbolId::from("eur/usd"),
                            0,
                            1.,
                            Utc.ymd(2017, 12, 29).and_hms(12, 0, 5)
                        )
                    )
                ).build().unwrap()
        ];

        assert_eq!(
            active_orders,
            expected_active_orders.iter().collect::<Vec<&Order>>()
        );
        assert_eq!(
            closed_orders,
            expected_closed_orders.iter().collect::<Vec<&Order>>()
        );
    }

}
