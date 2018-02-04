extern crate uuid;
extern crate serde_json;
use self::uuid::Uuid;
use strategy::StrategyId;
use order::{OrderId, OrderBuilder};
use signal::Signal;

pub trait GenerateOrderId {

    fn get_id(&self, strategy_id: StrategyId, signal: &Signal,
                  order_builder: &OrderBuilder) -> OrderId;

}

impl<M: GenerateOrderId + ?Sized> GenerateOrderId for Box<M> {

    fn get_id(&self, strategy_id: StrategyId, signal: &Signal,
              order_builder: &OrderBuilder) -> OrderId
    {
        (**self).get_id(strategy_id, signal, order_builder)
    }

}

pub struct UUIDOrderIdGenerator;

impl UUIDOrderIdGenerator {
    pub fn new() -> Self { UUIDOrderIdGenerator {} }
}

impl GenerateOrderId for UUIDOrderIdGenerator {

    fn get_id(&self, strategy_id: StrategyId, signal: &Signal, _order_builder: &OrderBuilder)
        -> OrderId
    {
        Uuid::new_v5(
            &uuid::NAMESPACE_OID,
            format!("{} - {}", strategy_id, signal.datetime()).as_str()
        ).hyphenated().to_string()
    }

}
