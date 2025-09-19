use crate::order_taking::{OrderId, PricedOrder};

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

pub type SendOrderAcknowledgement = fn(OrderAcknowledgement) -> impl Future<Output = SendResult>;

pub type SendOrderAcknowledgementOption =
    fn(OrderAcknowledgement) -> Option<OrderAcknowledgementSent>;

pub type AcknowledgeOrder = fn(
    CreateOrderAcknowledgementLetter,
    SendOrderAcknowledgementOption,
    PricedOrder,
) -> impl Future<Output = Option<OrderAcknowledgementSent>>;
