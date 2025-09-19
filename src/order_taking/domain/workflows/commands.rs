use crate::order_taking::{UnvalidatedCancelOrder, UnvalidatedChangeOrder, UnvalidatedOrder};
use chrono::{DateTime, Utc};

// ワークフローの入力
pub type PlaceOrder = Command<UnvalidatedOrder>;
pub type ChangeOrder = Command<UnvalidatedChangeOrder>;
pub type CancelOrder = Command<UnvalidatedCancelOrder>;

pub enum OrderTakingCommand {
    Place(PlaceOrder),
    Change(ChangeOrder),
    Cancel(CancelOrder),
}

// コマンド共通
struct Command<TData> {
    data: TData,
    timestamp: DateTime<Utc>,
    user_id: String,
}

impl<TData> Command<TData> {
    pub fn new(data: TData, user_id: String) -> Self {
        Command {
            data,
            timestamp: Utc::now(),
            user_id,
        }
    }
}
