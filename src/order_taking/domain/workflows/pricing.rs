use order_taking::domain::model::{GetProductPrice, PricedOrder, ValidatedOrder};

use crate::order_taking::{
    BillingAmount, GetProductPrice, PricedOrder, PricedOrderLine, to_priced_order_line,
};

pub fn price_order(
    get_product_price: GetProductPrice,
    validated_order: ValidatedOrder,
) -> Result<PricedOrder, PricingError> {
    let lines: Result<Vec<PricedOrderLine>, String> = validated_order
        .order_lines
        .into_iter()
        .map(|line| to_priced_order_line(get_product_price, line))
        .collect();

    let lines = lines?;

    let prices: Vec<Price> = lines.iter().map(|line| line.price.clone()).collect();
    let amount_to_bill = BillingAmount::sum_prices(prices);

    let priced_order = PricedOrder {
        id: validated_order.id,
        customer_info: validated_order.customer_info,
        shipping_address: validated_order.shipping_address,
        billing_address: validated_order.billing_address,
        order_lines,
        amount_to_bill,
    };

    Ok(priced_order)
}

#[derive(Debug, Clone)]
pub struct PricingError(pub String);
