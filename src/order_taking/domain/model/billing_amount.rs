#[derive(Debug, Clone, PartialEq)]
pub struct BillingAmount(f64);

impl BillingAmount {
    pub fn create(value: f64) -> Result<BillingAmount, String> {
        if value >= 0.0 {
            Ok(BillingAmount(value))
        } else {
            Err("Billing amount must be non-negative".to_string())
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn sum_prices(prices: Vec<Price>) -> Result<BillingAmount, String> {
        let total: f64 = prices.iter().map(|price| price.value()).sum();
        BillingAmount::create(total)
    }
}
