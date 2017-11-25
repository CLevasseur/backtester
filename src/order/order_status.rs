#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum OrderStatus {
    NotSent,
    Sent,
    PreSubmitted,
    Submitted,
    PartiallyFilled(u32, u32),
    Filled(u32),
    Cancelled(CancellationReason)
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum CancellationReason {
    FilledOca,
    OutdatedOrder
}
