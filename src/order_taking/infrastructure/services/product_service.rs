use crate::order_taking::{CheckProductCodeExists, ProductCode};

pub const CHECK_PRODUCT_CODE_EXISTS: CheckProductCodeExists = |_product_code: &ProductCode| true;
