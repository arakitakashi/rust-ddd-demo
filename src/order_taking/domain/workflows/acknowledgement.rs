use crate::order_taking::{EmailAddress, OrderId, Price, PricedOrder};

#[derive(Debug, Clone)]
pub struct HtmlString(pub String);

#[derive(Debug, Clone)]
pub struct OrderAcknowledgement {
    pub email_address: String,
    pub letter: HtmlString,
}

#[derive(Debug, Clone)]
pub enum SendResult {
    Sent,
    NotSent,
}

#[derive(Debug, Clone)]
pub struct OrderAcknowledgementSent {
    pub order_id: OrderId,
    pub email_address: String,
}

pub type CreateOrderAcknowledgementLetter = fn(PricedOrder) -> HtmlString;
pub const CREATE_ORDER_ACKNOWLEDGEMENT_LETTER: CreateOrderAcknowledgementLetter =
    |_priced_order: PricedOrder| HtmlString(String::from("<html></html>"));

pub type SendOrderAcknowledgement = fn(OrderAcknowledgement) -> impl Future<Output = SendResult>;
// TODO: 仮実装
pub const SEND_ORDER_ACKNOWLEDGEMENT: SendOrderAcknowledgement =
    |_order_acknowledgement: OrderAcknowledgement| async { SendResult::Sent };

pub type SendOrderAcknowledgementOption =
    fn(OrderAcknowledgement) -> Option<OrderAcknowledgementSent>;

pub type AcknowledgeOrder = fn(
    CreateOrderAcknowledgementLetter,
    SendOrderAcknowledgementOption,
    PricedOrder,
) -> impl Future<Output = Option<OrderAcknowledgementSent>>;
pub fn acknowledge_order(
    create_order_acknowledgement_letter: CreateOrderAcknowledgementLetter,
    send_order_acknowledgement: SendOrderAcknowledgement,
    priced_order: PricedOrder,
) -> impl Future<Output = Option<OrderAcknowledgementSent>> {
    let letter = create_order_acknowledgement_letter(&priced_order);
    let email_address = EmailAddress::create("test@test.com".to_string());

    let acknowledgement = OrderAcknowledgement {
        email_address: priced_order.customer_info.email_address,
        letter,
    };

    match send_order_acknowledgement(&acknowledgement) {
        SendResult::Sent => Some(acknowledgement),
        SendResult::NotSent => None,
    }
}
