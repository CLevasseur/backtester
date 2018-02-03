extern crate csv;
extern crate chrono;
use self::chrono::prelude::{DateTime, Utc};
use std::io;
use std::error::Error;
use util::OrderPair;
use order::{Order, OrderKind, OrderStatus};
use execution::Execution;
use direction::Direction;

fn order_kind_to_str(kind: &OrderKind) -> String {
    match *kind {
        OrderKind::MarketOrder => String::from("MKT"),
        OrderKind::LimitOrder(_) => String::from("LMT"),
        OrderKind::StopOrder(_) => String::from("STP")
    }
}

fn direction_to_str(direction: &Direction) -> String {
    match *direction {
        Direction::Long => String::from("LONG"),
        Direction::Short => String::from("SHORT")
    }
}

fn active_datetime_to_str(active_datetime: &Option<DateTime<Utc>>) -> String {
    match *active_datetime {
        Some(datetime) => datetime.to_string(),
        None => String::from("None")
    }
}

fn get_execution(order: &Order) -> &Execution {
    match *order.status() {
        OrderStatus::Filled(ref exec) => exec,
        _ => {
            panic!("Order is not filled: {:#?}", order);
        }
    }
}

pub fn write_order_pairs_to_csv<W>(writer: &mut csv::Writer<W>, order_pairs: &Vec<OrderPair>)
    -> Result<(), Box<Error>> where W: io::Write
{

    writer.write_record(&[
        "EntryOrderId",
        "EntryOrderKind",
        "EntryOrderActiveUntil",
        "EntryOrderActiveAfter",
        "EntryOrderDirection",
        "EntryOrderOCA",
        "EntrySymbolId",
        "EntryExecutionPrice",
        "EntryExecutionDatetime",
        "EntryExecutionQuantity",
        "ExitOrderId",
        "ExitOrderKind",
        "ExitOrderActiveUntil",
        "ExitOrderActiveAfter",
        "ExitOrderDirection",
        "ExitOrderOCA",
        "ExitSymbolId",
        "ExitExecutionPrice",
        "ExitExecutionDatetime",
        "ExitExecutionQuantity"
    ])?;

    for order_pair in order_pairs {
        let entry_execution = get_execution(order_pair.entry_order);
        let exit_execution = get_execution(order_pair.exit_order);

        writer.write_record(&[
            order_pair.entry_order.id().clone(),
            order_kind_to_str(order_pair.entry_order.kind()),
            active_datetime_to_str(order_pair.entry_order.active_until()),
            active_datetime_to_str(order_pair.entry_order.active_after()),
            direction_to_str(order_pair.entry_order.direction()),
            format!("{:?}", order_pair.entry_order.oca()),
            order_pair.entry_order.symbol_id().clone(),
            entry_execution.price().to_string().clone(),
            entry_execution.datetime().to_string().clone(),
            entry_execution.quantity().to_string().clone(),
            order_pair.exit_order.id().clone(),
            order_kind_to_str(order_pair.exit_order.kind()),
            active_datetime_to_str(order_pair.exit_order.active_until()),
            active_datetime_to_str(order_pair.exit_order.active_after()),
            direction_to_str(order_pair.exit_order.direction()),
            format!("{:?}", order_pair.exit_order.oca()),
            order_pair.exit_order.symbol_id().clone(),
            exit_execution.price().to_string().clone(),
            exit_execution.datetime().to_string().clone(),
            exit_execution.quantity().to_string().clone()
        ])?;
    }

    writer.flush()?;

    Ok(())
}
