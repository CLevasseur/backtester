extern crate backtester;
extern crate chrono;
extern crate time;
extern crate csv;
extern crate serde_json;

use chrono::prelude::{Utc, TimeZone};
use backtester::ohlcv::source::{OhlcvSource, CsvOhlcvSource};
use backtester::backtester::Backtester;
use backtester::model::{Model, ModelId};
use backtester::strategy::Strategy;
use backtester::signal::detector::{Once, Always};
use backtester::direction::Direction;
use backtester::order::{Order, OrderStatus, OrderBuilder, OrderKind};
use backtester::order::policy::{MarketOrderPolicy, SimpleOrderPolicy};
use backtester::symbol::SymbolId;
use backtester::util::record_parser::RecordParser;
use backtester::util::{get_order_pairs, write_order_pairs_to_csv};


pub struct OrderEveryCandle {
    symbol_id: SymbolId
}

impl Model for OrderEveryCandle {

    fn id(&self) -> ModelId {
        ModelId::from("order every candle")
    }

    fn entry_strategy(&self) -> Strategy {
        Strategy::new(
            Box::new(Always::new(self.symbol_id.clone(), Direction::Long)),
            Box::new(MarketOrderPolicy::new())
        )
    }

    fn exit_strategies(&self, order: &Order) -> Vec<Strategy> {
        if let &OrderStatus::Filled(ref execution) = order.status() {
            let timeout_strategy = Strategy::new(
                Box::new(Once::new(self.symbol_id.clone(), Direction::Short)),
                Box::new(SimpleOrderPolicy::new(
                    OrderBuilder::unallocated(
                        OrderKind::MarketOrder,
                        self.symbol_id.clone(),
                        Direction::Short
                    )
                        .set_active_after(execution.datetime().clone() + time::Duration::minutes(120))
                        .set_oca(order.id().clone())
                ))
            );

            let stop_loss_strategy = Strategy::new(
                Box::new(Once::new(self.symbol_id.clone(), Direction::Short)),
                Box::new(SimpleOrderPolicy::new(
                    OrderBuilder::unallocated(
                        OrderKind::StopOrder(execution.price() - 0.0005),
                        self.symbol_id.clone(),
                        Direction::Short
                    ).set_oca(order.id().clone())
                ))
            );

            return vec![timeout_strategy, stop_loss_strategy]
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
    write_order_pairs_to_csv(&mut writer, &get_order_pairs(&portfolio, &strategy_collection))
        .expect("Failed to write order pairs to csv");
}
