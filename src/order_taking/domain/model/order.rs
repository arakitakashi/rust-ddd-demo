use super::{
    AddressValidationError, CheckAddressExists, CheckProductCodeExists, GetProductPrice,
    OrderQuantity, ProductCode, UnvalidatedAddress,
};
use crate::order_taking::ValidatedAddress;

pub type OrderId = ();
pub type OrderLineId = ();
pub type CustomerId = ();

pub type BillingAddress = ();
pub type Price = ();
pub type BillingAmount = ();

pub type UnvalidatedCustomerInfo = ();
pub type ValidatedCustomerInfo = ();

#[derive(Debug, Clone)]
pub struct OrderLine {
    pub id: OrderLineId,
    pub order_id: OrderId,
    pub product_code: ProductCode,
    pub order_quantity: OrderQuantity,
    pub price: Price,
}

// 仮実装: 価格計算済みの注文としてOrderLineとは本来異なる
#[derive(Debug, Clone)]
pub struct PricedOrderLine {
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
    pub customer_info: UnvalidatedCustomerInfo,
    pub shipping_address: UnvalidatedAddress,
    pub billing_address: BillingAddress,
    pub order_lines: Vec<OrderLine>,
    pub amount_to_bill: BillingAmount,
}

#[derive(Debug, Clone)]
pub struct ValidatedOrder {
    pub id: OrderId,
    pub customer_id: CustomerId,
    pub customer_info: ValidatedCustomerInfo,
    pub shipping_address: ValidatedAddress,
    pub billing_address: BillingAddress,
    pub order_lines: Vec<OrderLine>,
    pub amount_to_bill: BillingAmount,
}

#[derive(Debug, Clone)]
pub struct PricedOrder {
    pub id: OrderId,
    pub customer_info: ValidatedCustomerInfo,
    pub shipping_address: ValidatedAddress,
    pub billing_address: ValidatedAddress,
    pub order_lines: Vec<PricedOrderLine>,
    pub amount_to_bill: BillingAmount,
}

// 簡易実装
#[derive(Debug, Clone)]
pub struct UnvalidatedChangeOrder {
    pub order_id: String,
    pub changes: String,
}

#[derive(Debug, Clone)]
pub struct UnvalidatedCancelOrder {
    pub order_id: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub enum Order {
    Unvalidated(UnvalidatedOrder),
    Validated(ValidatedOrder),
    Priced(PricedOrder),
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    AddressError(AddressValidationError),
}

pub type ValidateOrder = fn(
    CheckProductCodeExists,
    CheckAddressExists,
    UnvalidatedOrder,
) -> impl Future<Output = Result<ValidatedOrder, ValidationError>>;
