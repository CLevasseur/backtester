extern crate chrono;

use self::chrono::prelude::{DateTime, Utc};
use signal::detector::{DetectSignal, DetectSignalError};
use order::{Order};
use order::policy::{OrderPolicy, OrderPolicyError};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum StrategyError {
    DetectSignalError(DetectSignalError),
    CreateOrderError(OrderPolicyError)
}

pub struct Strategy<'symbol> {
    signal_detector: Box<DetectSignal<'symbol> + 'symbol>,
    order_policy: Box<OrderPolicy>
}

impl<'symbol> Strategy<'symbol> {
    
    pub fn new(signal_detector: Box<DetectSignal<'symbol> + 'symbol>, order_policy: Box<OrderPolicy>) -> Strategy<'symbol> {
        Strategy {
            signal_detector: signal_detector,
            order_policy: order_policy
        }
    }

    pub fn run(&self, datetime: &DateTime<Utc>) -> Result<Option<Order<'symbol>>, StrategyError> {
        match self.signal_detector.detect_signal(datetime) {
            Ok(Some(signal)) => match self.order_policy.create_order(signal) {
                Ok(order) => Ok(Some(order)),
                Err(e) => Err(StrategyError::CreateOrderError(e))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(StrategyError::DetectSignalError(e))
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use self::chrono::prelude::{TimeZone};
    use order::{Order, OrderBuilder, OrderKind};
    use signal::Signal;
    use signal::detector::{DetectSignal, DetectSignalError};
    use symbol::Symbol;
    use ohlcv::source::NullOhlcvSource;
    use direction::Direction;
    use order::policy::OrderPolicy;

    struct SomeSignal<'symbol> { symbol: &'symbol Symbol<'symbol> }
    impl<'symbol> DetectSignal<'symbol> for SomeSignal<'symbol> {
        fn detect_signal(&self, datetime: &DateTime<Utc>) -> Result<Option<Signal<'symbol>>, DetectSignalError> {
            Ok(Some(Signal::new(self.symbol, Direction::Long, datetime.clone(), String::new())))
        }
    }

    struct NoSignal;
    impl<'symbol> DetectSignal<'symbol> for NoSignal {
        fn detect_signal(&self, _datetime: &DateTime<Utc>) -> Result<Option<Signal<'symbol>>, DetectSignalError> {
            Ok(None)
        }
    }
    
    struct SignalError;
    impl<'symbol> DetectSignal<'symbol> for SignalError {
        fn detect_signal(&self, _datetime: &DateTime<Utc>) -> Result<Option<Signal<'symbol>>, DetectSignalError> {
            Err(DetectSignalError::IndicatorError)
        }
    }

    struct MockOrderPolicy;
    impl OrderPolicy for MockOrderPolicy {
        fn create_order<'symbol>(&self, signal: Signal<'symbol>) -> Result<Order<'symbol>, OrderPolicyError> {
            Ok(OrderBuilder::unallocated(OrderKind::MarketOrder, signal.symbol(),signal.direction().clone()).build())
        }
    }

    struct MockOrderPolicyError;
    impl OrderPolicy for MockOrderPolicyError {
        fn create_order<'symbol>(&self, _signal: Signal<'symbol>) -> Result<Order<'symbol>, OrderPolicyError> {
            Err(OrderPolicyError::IndicatorError)
        }
    }

    fn run_date() -> DateTime<Utc> { Utc.ymd(2016, 1, 3).and_hms(17, 0, 0) }

    #[test]
    fn run_some_signal() {
        let source = NullOhlcvSource {};
        let symbol = Symbol::new(String::new(), &source);
        let result = Strategy::new(Box::new(SomeSignal { symbol: &symbol}), Box::new(MockOrderPolicy {})).run(&run_date());
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
        let source = NullOhlcvSource {};
        let symbol = Symbol::new(String::new(), &source);
        let result = Strategy::new(Box::new(SomeSignal { symbol: &symbol}), Box::new(MockOrderPolicyError {})).run(&run_date());
        assert!(result.is_err());
    }
}
