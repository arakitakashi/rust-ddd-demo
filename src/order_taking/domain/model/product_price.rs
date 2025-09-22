use super::ProductCode;

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

pub type GetProductPrice = fn(ProductCode) -> Price;
// TODO: 仮実装
pub const GET_PRODUCT_PRICE: GetProductPrice =
    |_product_code: &ProductCode| Price::create(10.0).unwrap();
