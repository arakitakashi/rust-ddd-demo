// Address
#[derive(Debug, Clone)]
pub enum AddressValidationError {
    InvalidFormat(String),
    AddressNotFound(String),
}

impl std::fmt::Display for AddressValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressValidationError::InvalidFormat(msg) => {
                write!(f, "Invalid address format: {}", msg)
            }
            AddressValidationError::AddressNotFound(msg) => {
                write!(f, "Address not found: {}", msg)
            }
        }
    }
}

impl std::error::Error for AddressValidationError {}

// Price
#[derive(Debug, Clone)]
pub struct PricingError(pub String);

// Validation
#[derive(Debug, Clone)]
pub enum ValidationError {
    Address(AddressValidationError),
}

// Workflow
// ワークフロー成功時の出力
// #[derive(Debug, Clone, PartialEq)]
// pub struct ValidationError {
//     pub field_name: String,
//     pub error_description: String,
// }

#[derive(Debug, Clone)]
pub enum PlaceOrderError {
    Validation(Vec<ValidationError>),
    Pricing(PricingError),
}

impl From<ValidationError> for PlaceOrderError {
    fn from(err: ValidationError) -> Self {
        PlaceOrderError::Validation(err)
    }
}
