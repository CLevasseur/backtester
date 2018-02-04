extern crate backtester;
extern crate chrono;
extern crate time;
extern crate csv;
extern crate serde_json;

use std::fs::File;
use std::io;
use std::cell::Cell;
use chrono::prelude::{DateTime, Utc, TimeZone};
use backtester::ohlcv::source::{OhlcvSource, CsvOhlcvSource};
use backtester::backtester::Backtester;
use backtester::model::{Model, ModelId};
use backtester::strategy::Strategy;
use backtester::signal::detector::{DetectSignal, DetectSignalError};
use backtester::direction::Direction;
use backtester::order::{Order, OrderStatus};
use backtester::order::policy::{MarketOrderPolicy, StopOrderPolicy};
use backtester::symbol::SymbolId;
use backtester::signal::Signal;
use backtester::util::record_parser::RecordParser;
use backtester::util::{get_order_pairs, write_order_pairs_to_csv};


pub struct AlwaysDetectSignal {
    symbol_id: SymbolId,
    direction: Direction
}

impl DetectSignal for AlwaysDetectSignal {
    fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
        let signal = Signal::new(
            self.symbol_id.clone(),
            self.direction.clone(),
            datetime.clone(),
            String::from("always detect signal")
        );
        Ok(Some(signal))
    }
}

pub struct OnceDetectSignal {
    symbol_id: SymbolId,
    direction: Direction,
    detected: Cell<bool>
}

impl DetectSignal for OnceDetectSignal {
    fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
        if self.detected.get() {
            Ok(None)
        }
        else {
            let signal = Signal::new(
                self.symbol_id.clone(),
                self.direction.clone(),
                datetime.clone(),
                String::from("once detect signal")
            );
            self.detected.set(true);
            Ok(Some(signal))
        }
    }
}

pub struct PositionTimeoutSignal {
    symbol_id: SymbolId,
    direction: Direction,
    timeout_datetime: DateTime<Utc>
}

impl DetectSignal for PositionTimeoutSignal {
    fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
        if &self.timeout_datetime <= datetime {
            let signal = Signal::new(
                self.symbol_id.clone(),
                self.direction.clone(),
                datetime.clone(),
                String::from("always detect signal")
            );
            Ok(Some(signal))
        }
        else {
            Ok(None)
        }
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
            Box::new(AlwaysDetectSignal {
                symbol_id: self.symbol_id.clone(),
                direction: Direction::Long
            }),
            Box::new(MarketOrderPolicy::new())
        )
    }

    fn exit_strategies(&self, order: &Order) -> Vec<Strategy> {
        if let &OrderStatus::Filled(ref execution) = order.status() {
            let timeout_strategy = Strategy::new(
                Box::new(PositionTimeoutSignal {
                    symbol_id: self.symbol_id.clone(),
                    direction: Direction::Short,
                    timeout_datetime: execution.datetime().clone() + time::Duration::minutes(120)
                }),
                //            Box::new(AlwaysDetectSignal { symbol_id: self.symbol_id.clone(), direction: Direction::Short }),
                Box::new(MarketOrderPolicy::new())
            );

            let stop_loss_strategy = Strategy::new(
                Box::new(OnceDetectSignal {
                    symbol_id: self.symbol_id.clone(),
                    direction: Direction::Short,
                    detected: Cell::new(false)
                }),
                Box::new(StopOrderPolicy::new(execution.price() - 0.0005))
            );

            return vec![stop_loss_strategy]
        }

        panic!("Entry order not executed: {:#?}", order)
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

    println!("Start backtesting at {}", Utc::now());
    // launch backtest
    let backtester = Backtester::new();
    let models: Vec<Box<Model>> = vec![Box::new(OrderEveryCandle { symbol_id: symbol_id.clone() })];
    let (portfolio, strategy_collection) = backtester.run(
        &models,
        source.ohlcv(
            &Utc.ymd(2016, 1, 1).and_hms(0, 0, 0),
            &Utc.ymd(2016, 6, 1).and_hms(0, 0, 0)
        ).unwrap().into_iter()
    ).unwrap();
    println!("End backtesting at {}", Utc::now());

    let mut writer = csv::Writer::from_path("/tmp/result.csv").unwrap();
    write_order_pairs_to_csv(&mut writer, &get_order_pairs(&portfolio, &strategy_collection));
}
