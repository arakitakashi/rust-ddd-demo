use crate::order_taking::{
    BillingAmount, Price,
    domain::model::{Address, OrderId, PricedOrder},
};

use super::OrderAcknowledgementSent;

pub type OrderPlaced = PricedOrder;

#[derive(Debug, Clone)]
pub struct BillableOrderPlaced {
    pub order_id: OrderId,
    pub billing_address: Address,
    pub amount_to_bill: BillingAmount,
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

// ワークフローの最終ステップでイベントのリストを発行する
pub type CreateEvents = fn(PricedOrder, Option<OrderAcknowledgementSent>) -> Vec<PlaceOrderEvent>;
pub fn create_events(
    priced_order: PricedOrder,
    acknowledgement_event_opt: Option<OrderAcknowledgementSent>,
) -> Vec<PlaceOrderEvent> {
    let mut events = vec![PlaceOrderEvent::OrderPlaced(priced_order.clone())];

    if let Some(ack_event) = acknowledgement_event_opt {
        events.push(PlacedOrderEvent::AcknowledgementSent(ack_event));
    }

    if let Some(billing_event) = create_billing_event(&priced_order) {
        events.push(PlaceOrderEvent::BillableOrderPlaced(billing_event));
    }

    events
}

fn create_billing_event(placed_order: &PricedOrder) -> Option<BillableOrderPlaced> {
    let billing_amount_value = placed_order.amount_to_bill.value();

    if billing_amount_value > 0.0 {
        let order = BillableOrderPlaced {
            order_id: placed_order.id.clone(),
            billing_address: placed_order.billing_address.clone(),
            amount_to_bill: placed_order.amount_to_bill.clone(),
        };

        Some(order)
    } else {
        None
    }
}
