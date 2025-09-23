#[derive(Debug, Serialize)]
pub struct PlaceOrderResponse {
    pub success: bool,
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
