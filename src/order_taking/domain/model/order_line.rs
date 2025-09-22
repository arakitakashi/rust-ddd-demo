use crate::order_taking::domain::{OrderId, OrderQuantity, Price, ProductCode};

use super::{CheckProductCodeExists, GetProductPrice, product_code};

#[derive(Debug, Clone)]
pub struct UnvalidatedOrderLine {
    pub id: OrderLineId,
    pub order_id: OrderId,
    pub product_code: ProductCode,
    pub order_quantity: OrderQuantity,
}

#[derive(Debug, Clone)]
pub struct ValidatedOrderLine {
    pub id: OrderLineId,
    pub order_id: OrderId,
    pub product_code: ProductCode,
    pub order_quantity: OrderQuantity,
}

#[derive(Debug, Clone)]
pub struct PricedOrderLine {
    pub id: OrderLineId,
    pub order_id: OrderId,
    pub product_code: ProductCode,
    pub quantity: OrderQuantity,
    pub line_price: Price,
}

#[derive(Debug, Clone)]
pub struct OrderLineId(String);

impl OrderLineId {
    pub fn create(value: String) -> Self {
        if !value.contains('@') {
            panic!("Invalid orderline ID");
        }
        OrderLineId(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub fn to_validated_orderline(
    check_product_code_exists: CheckProductCodeExists,
    orderline: UnvalidatedOrderLine,
) {
    let order_line_id = OrderLineId::create(orderline.id);
    let order_id = orderline.order_id;
    // TODO: ヘルパー関数の実装
    let product_code = check_product_code_exists(&orderline.product_code);
    // TODO: ヘルパー関数の実装
    let order_quantity = orderline.order_quantity;
    let validated_order_line = ValidatedOrderLine {
        id,
        order_id,
        product_code,
        order_quantity,
    };
    validated_order_line
}

pub fn to_priced_order_line(
    get_product_price: GetProductPrice,
    line: ValidatedOrderLine,
) -> Result<PricedOrderLine, String> {
    let quantity = get_quantity_value(&line.quantity);

    let price = get_product_price(&line.product_code);

    let line_price = price.multiply(quantity)?;

    Ok(PricedOrderLine {
        id: line.id,
        order_id: line.order_id,
        product_code: line.product_code,
        quantity: line.quantity,
        line_price,
    })
}
