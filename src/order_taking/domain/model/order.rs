use super::{
    Address, AddressValidationError, CheckAddressExists, CheckProductCodeExists, GetProductPrice,
    OrderQuantity, ProductCode, UnvalidatedAddress, to_address, to_validated_customer_info,
    to_validated_orderline,
};
use crate::order_taking::ValidatedAddress;
use crate::order_taking::model::{
    BillingAmount, OrderId, OrderLine, Price, UnvalidatedCustomerInfo, ValidatedCustomerInfo,
};

// TODO: 未定義
pub type CustomerId = ();

// TODO: 未定義
pub type BillingAddress = ();

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
    pub billing_address: ValidatedAddress,
    pub order_lines: Vec<ValidatedOrderLine>,
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

pub async fn validate_order(
    check_product_code_exists: CheckProductCodeExists,
    check_address_exists: CheckAddressExists,
    unvalidated_order: UnvalidatedOrder,
) -> Result<ValidatedOrder, ValidationError> {
    let order_id = OrderId::create(unvalidated_order);

    let validated_customer_info = to_validated_customer_info(unvalidated_order.customer_info);

    // 非同期処理をシミュレート
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    let validated_shipping_adderss =
        to_address(check_address_exists, unvalidated_order.shipping_address);

    // 非同期処理をシミュレート
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    let validated_billing_address =
        to_address(check_address_exists, unvalidated_order.billing_address);

    let validated_lines_futures = unvalidated_order.order_lines.into_iter().map(|line| {
        to_validated_orderline(check_product_code_exists, line);
    });

    let validated_lines = futures::future::try_join_all(validated_lines_futures).await?;

    Ok(ValidatedOrder {
        id,
        customer_id: unvalidated_order.customer_id,
        customer_info: validated_customer_info,
        shipping_address: validated_shipping_adderss,
        billing_address: validated_billing_address,
        order_lines: validated_lines,
    })
}
