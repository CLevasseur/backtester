use strategy::Strategy;
use order::Order;

pub type ModelId = String;

pub trait Model {
    
    fn id(&self) -> ModelId;
    fn entry_strategy(&self) -> Strategy;
    fn exit_strategies(&self, order: &Order) -> Vec<Strategy>;

}

impl<M: Model + ?Sized> Model for Box<M> {

    fn id(&self) -> ModelId {
        (**self).id()
    }

    fn entry_strategy(&self) -> Strategy {
        (**self).entry_strategy()
    }

    fn exit_strategies(&self, order: &Order) -> Vec<Strategy> {
        (**self).exit_strategies(order)
    }

}

impl<'a, M: Model + ?Sized> Model for &'a M {

    fn id(&self) -> ModelId {
        (**self).id()
    }

    fn entry_strategy(&self) -> Strategy {
        (**self).entry_strategy()
    }

    fn exit_strategies(&self, order: &Order) -> Vec<Strategy> {
        (**self).exit_strategies(order)
    }

}
