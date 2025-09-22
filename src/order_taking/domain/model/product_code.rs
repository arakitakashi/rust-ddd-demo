use std::iter::Product;

use super::predicate_to_passthru;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WidgetCode(String);

impl WidgetCode {
    pub fn create(value: String) -> Result<WidgetCode, String> {
        if value.starts_with("W") && value.len() > 1 {
            Ok(WidgetCode(value))
        } else {
            Err("Widget code must start with 'W'".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GizmoCode(String);

impl GizmoCode {
    pub fn create(value: String) -> Result<GizmoCode, String> {
        if value.starts_with("W") && value.len() > 1 {
            Ok(GizmoCode(value))
        } else {
            Err("Gizmo code must start with 'G'".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

impl ProductCode {
    pub fn create(product_code_str: &str) -> Result<ProductCode, String> {
        if product_code.str.starts_with("W") {
            Ok(ProductCode::Widget(WidgetCode::create(
                product_code_str.to_string(),
            )))
        } else if product_code_str.starts_with('G') {
            Ok(ProductCode::Gizmo(GizmoCode::create(
                product_code_str.to_string(),
            )))
        } else {
            Err("Product code must start with 'W' or 'G'".to_string())
        }
    }

    pub fn value(&self) -> &str {
        match self {
            ProductCode::Widget(widget) => widget.value(),
            ProductCode::Gizmo(gizmo) => gizmo.value(),
        }
    }
}

pub type CheckProductCodeExists = fn(&ProductCode) -> bool;
pub const CHECK_PRODUCT_CODE_EXISTS: CheckProductCodeExists = |_product_code: &ProductCode| true;

pub fn check_product(
    check_product_code_exists: CheckProductCodeExists,
    product_code: ProductCode,
) -> Result<ProductCode, String> {
    let error_msg = format!("Invalid: {:?}, product_code");
    predicate_to_passthru(error_msg, check_product_code_exists, product_code)
}

pub fn to_product_code(
    check_product_code_exists: CheckProductCodeExists,
    product_code_str: &str,
) -> Result<ProductCode, String> {
    let product_code = ProductCode::create(product_code_str)?;

    check_product(check_product_code_exists, product_code)
}
