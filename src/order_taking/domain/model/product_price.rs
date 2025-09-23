use super::ProductCode;
use crate::order_taking::{
    PricingError,
    model::{BillingAmount, PricedOrder, PricedOrderLine, ValidatedOrder, to_priced_order_line},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Price(f64);

impl Price {
    pub fn create(value: f64) -> Result<Price, String> {
        if value >= 0.0 {
            Ok(Price(value))
        } else {
            Err("Price must be non-negative".to_string())
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn multiply(&self, quantity: f64) -> Result<Price, String> {
        Price::create(quantity * self.0)
    }
}

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

pub type GetProductPrice = fn(ProductCode) -> Price;
