extern crate backtester;
extern crate chrono;

use chrono::prelude::{DateTime, Utc, TimeZone};
use backtester::backtester::Backtester;
use backtester::market_simulation::MarketSimulation;
use backtester::model::{Model, ModelId};
use backtester::strategy::Strategy;
use backtester::signal::detector::{DetectSignal, DetectSignalError};
use backtester::direction::Direction;
use backtester::order::Order;
use backtester::order::policy::MarketOrderPolicy;
use backtester::symbol::SymbolId;
use backtester::signal::Signal;


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
fn backtest_order_every_candle() {
    let models: Vec<Box<Model>> = vec![Box::new(OrderEveryCandle { symbol_id: SymbolId::from("eur/usd") })];
    let backtester = Backtester::new(MarketSimulation::new());
    
    backtester.run(&models, Utc.ymd(2006, 1, 1).and_hms(0, 0, 0), Utc.ymd(2017, 2, 1).and_hms(0, 0, 0));
}
