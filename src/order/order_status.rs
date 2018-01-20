use execution::Execution;

#[derive(Clone, PartialEq, Debug)]
pub enum OrderStatus {
    NotSent,
    Sent,
    Filled(Execution),
    Cancelled(CancellationReason)
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum CancellationReason {
    FilledOca,
    OutdatedOrder
}
