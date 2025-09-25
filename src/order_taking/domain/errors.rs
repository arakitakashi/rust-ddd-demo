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
pub enum PricingError {
    InvalidPrice(String),
    PriceNotFound(String),
    CalculationError(String),
}

impl std::fmt::Display for PricingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PricingError::InvalidPrice(msg) => write!(f, "Invalid price: {}", msg),
            PricingError::PriceNotFound(msg) => write!(f, "Price not found: {}", msg),
            PricingError::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
        }
    }
}

impl std::error::Error for PricingError {}

// Remote service
#[derive(Debug, Clone)]
pub struct RemoteServiceError {
    pub service: String,
    pub message: String,
}

impl std::fmt::Display for RemoteServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Service '{}' error: {}", self.service, self.message)
    }
}

impl std::error::Error for RemoteServiceError {}

// Validation
#[derive(Debug, Clone)]
pub enum ValidationError {
    Address(AddressValidationError),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::Address(msg) => write!(f, "Invalid address: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

// Workflow
// ワークフロー成功時の出力
// #[derive(Debug, Clone, PartialEq)]
// pub struct ValidationError {
//     pub field_name: String,
//     pub error_description: String,
// }

// Fromトレイトで自動変換を実現
#[derive(Debug, Clone)]
pub enum PlaceOrderError {
    Validation(Vec<ValidationError>),
    Pricing(PricingError),
    RemoteService(RemoteServiceError),
}

impl From<ValidationError> for PlaceOrderError {
    fn from(err: ValidationError) -> Self {
        PlaceOrderError::Validation(err)
    }
}
impl From<PricingError> for PlaceOrderError {
    fn from(err: PricingError) -> Self {
        PlaceOrderError::Pricing(err)
    }
}
impl From<RemoteServiceError> for PlaceOrderError {
    fn from(err: RemoteServiceError) -> Self {
        PlaceOrderError::RemoteService(err)
    }
}

impl std::fmt::Display for PlaceOrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaceOrderError::Validation(e) => write!(f, "Validation error: {}", e),
            PlaceOrderError::Pricing(e) => write!(f, "Pricing error: {}", e),
            PlaceOrderError::RemoteService(e) => write!(f, "Remote service error: {}", e),
        }
    }
}

impl std::error::Error for PlaceOrderError {}
