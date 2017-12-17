extern crate snowflake;
extern crate chrono;

mod strategy_manager;

use self::snowflake::ProcessUniqueId;
use self::chrono::prelude::{DateTime, Utc};
use signal::detector::{DetectSignal, DetectSignalError};
use order::{Order};
use order::policy::{OrderPolicy, OrderPolicyError};
pub use strategy::strategy_manager::StrategyManager;


pub type StrategyId = ProcessUniqueId;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum StrategyError {
    DetectSignalError(DetectSignalError),
    CreateOrderError(OrderPolicyError)
}

pub struct Strategy {
    id: StrategyId,
    signal_detector: Box<DetectSignal>,
    order_policy: Box<OrderPolicy>
}

impl Strategy {
    
    pub fn new(signal_detector: Box<DetectSignal>, order_policy: Box<OrderPolicy>) -> Strategy {
        Strategy {
            id: StrategyId::new(),
            signal_detector: signal_detector,
            order_policy: order_policy
        }
    }

    pub fn run(&self, datetime: &DateTime<Utc>) -> Result<Option<Order>, StrategyError> {
        match self.signal_detector.detect_signal(datetime) {
            Ok(Some(signal)) => match self.order_policy.create_order(signal) {
                Ok(order) => Ok(Some(order)),
                Err(e) => Err(StrategyError::CreateOrderError(e))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(StrategyError::DetectSignalError(e))
        }
    }

    pub fn id(&self) -> &StrategyId {
        &self.id
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use self::chrono::prelude::{TimeZone};
    use order::{Order, OrderBuilder, OrderKind};
    use signal::Signal;
    use signal::detector::{DetectSignal, DetectSignalError};
    use symbol::SymbolId;
    use direction::Direction;
    use order::policy::OrderPolicy;

    struct SomeSignal { symbol_id: SymbolId }
    impl DetectSignal for SomeSignal {
        fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
            Ok(Some(Signal::new(self.symbol_id.clone(), Direction::Long, datetime.clone(), String::new())))
        }
    }

    struct NoSignal;
    impl DetectSignal for NoSignal {
        fn detect_signal(&self, _datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
            Ok(None)
        }
    }
    
    struct SignalError;
    impl DetectSignal for SignalError {
        fn detect_signal(&self, _datetime: &DateTime<Utc>) -> Result<Option<Signal>, DetectSignalError> {
            Err(DetectSignalError::IndicatorError)
        }
    }

    struct MockOrderPolicy;
    impl OrderPolicy for MockOrderPolicy {
        fn create_order(&self, signal: Signal) -> Result<Order, OrderPolicyError> {
            Ok(OrderBuilder::unallocated(OrderKind::MarketOrder, signal.symbol_id().clone(),signal.direction().clone()).build())
        }
    }

    struct MockOrderPolicyError;
    impl OrderPolicy for MockOrderPolicyError {
        fn create_order(&self, _signal: Signal) -> Result<Order, OrderPolicyError> {
            Err(OrderPolicyError::IndicatorError)
        }
    }

    fn run_date() -> DateTime<Utc> { Utc.ymd(2016, 1, 3).and_hms(17, 0, 0) }

    #[test]
    fn run_some_signal() {
        let symbol_id = SymbolId::from("symbol");
        let result = Strategy::new(Box::new(SomeSignal { symbol_id: symbol_id.clone()}), Box::new(MockOrderPolicy {})).run(&run_date());
        assert!(result.is_ok());
    }

    #[test]
    fn run_no_signal() {
        let result = Strategy::new(Box::new(NoSignal {}), Box::new(MockOrderPolicy {})).run(&run_date());
        assert!(result.is_ok());
        assert!(result.unwrap().is_none())
    }

    #[test]
    fn run_signal_error() {
        let result = Strategy::new(Box::new(SignalError {}), Box::new(MockOrderPolicy {})).run(&run_date());
        assert!(result.is_err());
    }

    #[test]
    fn run_create_order_error() {
        let symbol_id = SymbolId::from("symbol");
        let result = Strategy::new(Box::new(SomeSignal { symbol_id: symbol_id.clone()}), Box::new(MockOrderPolicyError {})).run(&run_date());
        assert!(result.is_err());
    }
}
