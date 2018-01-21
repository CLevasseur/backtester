extern crate uuid;
extern crate serde_json;
use self::uuid::Uuid;
use strategy::StrategyId;
use order::{OrderId, OrderBuilder};
use signal::Signal;

pub struct OrderIdGenerator;

impl OrderIdGenerator {
    pub fn new() -> Self { OrderIdGenerator {} }

    pub fn get_id(&self, strategy_id: StrategyId, signal: &Signal, _order_builder: &OrderBuilder)
        -> OrderId
    {
        Uuid::new_v5(
            &uuid::NAMESPACE_OID,
            format!("{} - {}", strategy_id, signal.datetime()).as_str()
        ).hyphenated().to_string()
    }
}
