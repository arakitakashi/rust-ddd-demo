use crate::order_taking::{GetProductPrice, Price, ProductCode};

// TODO: 仮実装
pub const GET_PRODUCT_PRICE: GetProductPrice =
    |_product_code: &ProductCode| Price::create(10.0).unwrap();
