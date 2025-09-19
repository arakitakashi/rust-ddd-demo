use order_taking::domain::model::{GetProductPrice, PricedOrder, ValidatedOrder};

pub type PriceOrder = fn(GetProductPrice, ValidatedOrder) -> Result<PricedOrder, PricingError>;

#[derive(Debug, Clone)]
pub struct PricingError(pub String);
