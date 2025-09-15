use super::{OrderQuantity, ProductCode};

pub type OrderId = ();
pub type OrderLineId = ();
pub type CustomerId = ();

pub type CustomerInfo = ();
pub type BillingAddress = ();
pub type Price = ();
pub type BillingAmount = ();

pub type UnvalidatedAddress = ();
pub type ValidatedAddress = ();

#[derive(Debug, Clone)]
pub struct OrderLine {
    pub id: OrderLineId,
    pub order_id: OrderId,
    pub product_code: ProductCode,
    pub order_quantity: OrderQuantity,
    pub price: Price,
}

#[derive(Debug, Clone)]
pub struct UnvalidatedOrder {
    pub id: OrderId,
    pub customer_id: CustomerId,
    pub shipping_address: UnvalidatedAddress,
    pub billing_address: BillingAddress,
    pub order_lines: Vec<OrderLine>,
    pub amount_to_bill: BillingAmount,
}

#[derive(Debug, Clone)]
pub struct ValidatedOrder {
    pub id: OrderId,
    pub customer_id: CustomerId,
    pub shipping_address: ValidatedAddress,
    pub billing_address: BillingAddress,
    pub order_lines: Vec<OrderLine>,
    pub amount_to_bill: BillingAmount,
}
