use crate::order_taking::{
    CheckProductCodeExists, GetProductPrice, UnvalidatedOrder, validate_order,
};

use super::{
    CreateOrderAcknowledgementLetter, PricingError, SendOrderAcknowledgement, acknowledge_order,
    create_events, price_order,
};

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
    PricingError(PricingError),
}

impl From<ValidationError> for PlaceOrderError {
    fn from(err: ValidationError) -> Self {
        PlaceOrderError::ValidationError(err)
    }
}

// トップレベルのワークフロー関数型
pub type PlaceOrder = fn(UnvalidatedOrder) -> Result<PlaceOrderEvents, PlaceOrderError>;

pub fn create_place_order_workflow(
    check_product_code_exists: CheckProductCodeExists,
    check_address_exists: CheckAddressExists,
    get_product_price: GetProductPrice,
    create_acknowledgement_letter: CreateOrderAcknowledgementLetter,
    send_acknowledgement: SendOrderAcknowledgement,
) -> impl Fn(UnvalidatedOrder) -> Result<Vec<PlaceOrderEvents>, String> {
    move |unvalidated_order: UnvalidatedOrder| -> Result<Vec<PlaceOrderEvent>, String> {
        // 関数の部分適用
        let validate_order =
            |order| validate_order(check_product_code_exists, check_address_exists, order);
        let price_order = |order| price_order(get_product_price, order);
        let acknowledge_order = |order| {
            acknowledge_order(
                create_order_acknowledgement_letter,
                send_order_acknowledgement,
                order,
            )
        };

        // パイプライン処理
        let validated_order = validate_order(unvalidated_order);
        let priced_order = price_order(validated_order);
        let acknowledgement_option = acknowledge_order(priced_order);

        let events = create_events(priced_order, acknowledgement_option);

        Ok(events)
    }
}
