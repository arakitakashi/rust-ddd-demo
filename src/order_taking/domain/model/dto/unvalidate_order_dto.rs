#[derive(Debug, Deserialize)]
pub struct UnvalidatedOrderDto {
    pub order_id: String,
    pub customer_info: String,
    pub shipping_address: String,
    pub billing_address: String,
    pub lines: Vec<UnvalidatedOrderLineDto>,
}

#[derive(Debug, Deserialize)]
pub struct UnvalidatedOrderLineDto {
    pub order_line_id: String,
    pub product_code: String,
    pub quantity: String,
}
