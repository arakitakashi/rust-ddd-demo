use crate::order_taking::{OrderAcknowledgement, SendOrderAcknowledgement, SendResult};

// TODO: 仮実装
pub const SEND_ORDER_ACKNOWLEDGEMENT: SendOrderAcknowledgement =
    |_order_acknowledgement: OrderAcknowledgement| async { SendResult::Sent };
