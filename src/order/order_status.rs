use execution::Execution;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum OrderStatus {
    NotSent,
    Sent,
    Filled(Execution),
    Cancelled(CancellationReason)
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum CancellationReason {
    FilledOca,
    OutdatedOrder
}
