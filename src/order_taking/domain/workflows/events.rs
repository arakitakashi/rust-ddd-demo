use crate::order_taking::domain::model::{Address, OrderId, PricedOrder};

use super::OrderAcknowledgementSent;

pub type OrderPlaced = PricedOrder;

#[derive(Debug, Clone)]
pub struct BillableOrderPlaced {
    pub order_id: OrderId,
    pub billing_address: Address,
}

#[derive(Debug, Clone)]
pub enum PlaceOrderEvent {
    OrderPlaced(OrderPlaced),
    BillableOrderPlaced(BillableOrderPlaced),
    AcknowledgementSent(OrderAcknowledgementSent),
}

#[derive(Debug, Clone)]
pub struct PlaceOrderResult {
    pub order_placed: OrderPlaced,
    pub billable_order_placed: BillableOrderPlaced,
    pub order_acknowledgement_sent: Option<OrderAcknowledgementSent>,
}

pub type CreateEvents = fn(PricedOrder) -> Vec<PlaceOrderEvent>;
