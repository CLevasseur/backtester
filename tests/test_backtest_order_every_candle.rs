extern crate backtester;
extern crate chrono;
extern crate csv;
extern crate serde_json;

use std::fs::File;
use std::io;
use chrono::prelude::{DateTime, Utc, TimeZone};
use backtester::ohlcv::source::{OhlcvSource, CsvOhlcvSource};
use backtester::backtester::Backtester;
use backtester::model::{Model, ModelId};
use backtester::strategy::Strategy;
use backtester::signal::detector::{DetectSignal, DetectSignalError};
use backtester::direction::Direction;
use backtester::order::Order;
use backtester::order::policy::MarketOrderPolicy;
use backtester::symbol::SymbolId;
use backtester::signal::Signal;
use backtester::util::record_parser::RecordParser;
use backtester::util::{get_order_pairs, write_order_pairs_to_csv};


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
    let symbol_id = SymbolId::from("eur/usd");

    // create ohlcv source
    let path = String::from("tests/data/eurusd.csv");
    let reader = csv::ReaderBuilder::new().has_headers(false).delimiter(b';').from_path(&path).unwrap();
    let record_parser = RecordParser::new(symbol_id.clone(), String::from("%Y%m%d %H%M%S"));
    let source = CsvOhlcvSource::new(reader, record_parser).unwrap();

    // launch backtest
    let backtester = Backtester::new();
    let models: Vec<Box<Model>> = vec![Box::new(OrderEveryCandle { symbol_id: symbol_id.clone() })];
    let (portfolio, strategy_collection) = backtester.run(
        &models,
        source.ohlcv(
            &Utc.ymd(2016, 6, 1).and_hms(0, 0, 0),
            &Utc.ymd(2016, 12, 1).and_hms(0, 0, 0)
        ).unwrap().into_iter()
    ).unwrap();

    let mut writer = csv::Writer::from_path("/tmp/result.csv").unwrap();
    write_order_pairs_to_csv(&mut writer, &get_order_pairs(&portfolio, &strategy_collection));
}
