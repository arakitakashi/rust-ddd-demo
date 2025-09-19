use crate::order_taking::UnvalidatedOrder;

// ワークフロー成功時の出力
#[derive(Debug, Clone)]
pub struct PlaceOrderEvents {
    pub acknowledgement_sent: String,
    pub order_placed: String,
    pub billable_order_placed: String,
}

// ワークフロー成功時の出力
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub field_name: String,
    pub error_description: String,
}

#[derive(Debug, Clone)]
pub enum PlaceOrderError {
    ValidationError(Vec<ValidationError>),
}

// トップレベルのワークフロー関数型
pub type PlaceOrder = fn(UnvalidatedOrder) -> Result<PlaceOrderEvents, PlaceOrderError>;
