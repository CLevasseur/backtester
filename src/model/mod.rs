use strategy::Strategy;
use order::Order;

pub trait Model {
    
    fn entry_strategy(&self) -> Strategy;
    fn exit_strategies<'symbol>(&self, order: Order<'symbol>) -> Vec<Strategy>;

}
