extern crate backtester;
extern crate chrono;
extern crate time;
extern crate csv;
extern crate serde_json;
extern crate ta_lib_wrapper;

use std::rc::Rc;
use std::collections::HashMap;
use chrono::prelude::{Utc, DateTime, TimeZone};
use ta_lib_wrapper::{TA_Integer, TA_Real, TA_RSI,  TA_RetCode, TA_SetUnstablePeriod, TA_FuncUnstId};
use backtester::ohlcv::source::{OhlcvSource, CsvOhlcvSource};
use backtester::backtester::Backtester;
use backtester::model::{Model, ModelId};
use backtester::strategy::Strategy;
use backtester::signal::Signal;
use backtester::signal::detector::{DetectSignal, DetectSignalError, Once};
use backtester::direction::Direction;
use backtester::order::{Order, OrderKind};
use backtester::order::policy::SimpleOrderPolicy;
use backtester::symbol::SymbolId;
use backtester::util::record_parser::RecordParser;
use backtester::util::{get_order_pairs, write_order_pairs_to_csv};


/// Compute RSI(period) on `close_prices`
/// This function returns a tuple containing the list of rsi values and the index of the first
/// close to have an associated rsi value
fn rsi(period: u32, close_prices: &Vec<TA_Real>) -> Vec<TA_Real> {
    let mut out: Vec<TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;

    unsafe {
        TA_SetUnstablePeriod(TA_FuncUnstId::TA_FUNC_UNST_RSI, 1024);
        let ret_code = TA_RSI(
            0,                              // index of the first close to use
            close_prices.len() as i32 - 1,  // index of the last close to use
            close_prices.as_ptr(),          // pointer to the first element of the vector
            period as i32,                  // period of the rsi
            &mut out_begin,                 // set to index of the first close to have an rsi value
            &mut out_size,                  // set to number of sma values computed
            out.as_mut_ptr()                // pointer to the first element of the output vector
        );
        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_RSI call
            TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code)
        }
    }

    out
}

pub struct ShortWhenRSICrosses70Signal {
    symbol_id: SymbolId,
    rsi_values: HashMap<DateTime<Utc>, f64>
}

impl DetectSignal for ShortWhenRSICrosses70Signal {
    fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {

        let previous_rsi = self.rsi_values.get(&(datetime.clone() - time::Duration::minutes(1)));
        let current_rsi = self.rsi_values.get(datetime);

        match (previous_rsi, current_rsi) {
            (Some(previous_value), Some(current_value)) => {
                if *previous_value >= 70. && *current_value < 70. {
                    let signal = Signal::new(
                        self.symbol_id.clone(),
                        Direction::Short,
                        datetime.clone(),
                        String::from("rsi crossed 70")
                    );
                    Ok(Some(signal))
                }
                else {
                    Ok(None)
                }
            },
            _ => Ok(None)
        }
    }
}

pub struct ShortWhenRSICrosses70 {
    symbol_id: SymbolId,
    rsi_values: HashMap<DateTime<Utc>, f64>
}

impl Model for ShortWhenRSICrosses70 {

    fn id(&self) -> ModelId {
        ModelId::from("short when rsi crosses 70")
    }

    fn entry_strategy(&self) -> Strategy {
        Strategy::new(
            Box::new(
                ShortWhenRSICrosses70Signal {
                    symbol_id: self.symbol_id.clone(),
                    rsi_values: self.rsi_values.clone()
                }
            ),
            Box::new(SimpleOrderPolicy::new(OrderKind::MarketOrder))
        )
    }

    fn exit_strategies(&self, entry_order: &Order) -> Vec<Strategy> {
        let execution = entry_order.execution().expect("Entry order not executed");

        let stop_loss_strategy = Strategy::new(
            Box::new(Once::new(self.symbol_id.clone(), Direction::Long)),
            Box::new(SimpleOrderPolicy::new(OrderKind::StopOrder(execution.price() - 0.0005))
                .set_oca(Some(entry_order.id().clone()))
            )
        );

        let timeout_strategy = Strategy::new(
            Box::new(Once::new(self.symbol_id.clone(), Direction::Long)),
            Box::new(SimpleOrderPolicy::new(OrderKind::MarketOrder)
                .set_active_after(Some(execution.datetime().clone() + time::Duration::minutes(120)))
                .set_oca(Some(entry_order.id().clone()))
            )
        );

        let take_profit_strategy = Strategy::new(
            Box::new(Once::new(self.symbol_id.clone(), Direction::Long)),
            Box::new(SimpleOrderPolicy::new(OrderKind::LimitOrder(execution.price() + 0.0005))
                .set_oca(Some(entry_order.id().clone()))
            )
        );

        return vec![stop_loss_strategy, timeout_strategy, take_profit_strategy]
    }

}

fn main() {
    let symbol_id = SymbolId::from("eur/usd");

    // create ohlcv source
    let path = String::from("tests/data/eurusd.csv");
    let reader = csv::ReaderBuilder::new().has_headers(false).delimiter(b';').from_path(&path).unwrap();
    let record_parser = RecordParser::new(symbol_id.clone(), String::from("%Y%m%d %H%M%S"));
    println!("Loading ohlcv data at {}", Utc::now());
    let source = Rc::new(CsvOhlcvSource::new(reader, record_parser).unwrap());

    println!("Start backtesting at {}", Utc::now());
    let ohlcv_values = source.ohlcv(
        &Utc.ymd(2016, 1, 1).and_hms(0, 0, 0),
        &Utc.ymd(2016, 12, 31).and_hms(0, 0, 0)
    ).expect("Can't get ohlcv values");

    let datetimes: Vec<DateTime<Utc>> = ohlcv_values.iter().map(|ohlcv| ohlcv.datetime().clone()).collect();
    let close_values = ohlcv_values.iter().map(|ohlcv| ohlcv.close() as TA_Real).collect();
    let rsi_values = rsi(14, &close_values);
    let rsi_values: HashMap<DateTime<Utc>, f64> = datetimes[(datetimes.len() - rsi_values.len())..]
        .iter().cloned().zip(rsi_values.into_iter()).into_iter().collect();

    // launch backtest
    let backtester = Backtester::new();
    let models: Vec<Box<Model>> = vec![
        Box::new(ShortWhenRSICrosses70 {
            symbol_id: symbol_id.clone(),
            rsi_values: rsi_values
        })
    ];
    let (portfolio, strategy_collection) = backtester.run(&models, ohlcv_values.into_iter()).expect("backtest failed");
    println!("End backtesting at {}", Utc::now());

    let mut writer = csv::Writer::from_path("/tmp/result.csv").unwrap();
    write_order_pairs_to_csv(&mut writer, &get_order_pairs(&portfolio, &strategy_collection))
        .expect("Failed to write order pairs to csv");
}
