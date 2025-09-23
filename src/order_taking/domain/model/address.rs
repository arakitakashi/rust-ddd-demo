use std::future::Future;
use std::pin::Pin;

use super::String50;

pub type ValidatedAddress = ();

#[derive(Debug, Clone)]
pub struct UnvalidatedAddress {
    pub address_line1: String,
    pub address_line2: String,
    pub address_line3: String,
    pub address_line4: String,
    pub city: String,
    pub zip_code: String,
}

#[derive(Debug, Clone)]
pub struct CheckedAddress {
    pub address_line1: String,
    pub address_line2: String,
    pub address_line3: String,
    pub address_line4: String,
    pub city: String,
    pub zip_code: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedAddress {
    pub address_line1: String,
    pub address_line2: String,
    pub address_line3: String,
    pub address_line4: String,
    pub city: String,
    pub zip_code: ZipCode,
}

#[derive(Debug, Clone)]
pub enum Address {
    UnvalidatedAddress(UnvalidatedAddress),
    ValidatedAddress(ValidatedAddress),
}

#[derive(Debug, Clone)]
pub struct ZipCode(String);

impl ZipCode {
    pub fn create(value: String) -> Self {
        if value.trim().is_empty() {
            panic!("ZipCode must not be empty");
        }
        ZipCode(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

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

pub async fn to_address(
    check_address_exists: CheckAddressExists,
    unvalidated_address: UnvalidatedAddress,
) -> Result<ValidatedAddress, AddressValidationError> {
    let checked_address = check_address_exists(unvalidated_address).await?;

    let address_line1 = String50::create(checked_address.address_line1);
    let address_line2 = String50::create(checked_address.address_line2);
    let address_line3 = String50::create(checked_address.address_line3);
    let address_line4 = String50::create(checked_address.address_line4);
    let city = String50::create(checked_address.city);
    let zip_code = ZipCode::create(checked_address.zip_code);

    let address = ValidatedAddress {
        address_line1,
        address_line2,
        address_line3,
        address_line4,
        city,
        zip_code,
    };

    Ok(address)
}

pub type CheckAddressExists =
    fn(UnvalidatedAddress) -> impl Future<Output = Result<CheckedAddress, AddressValidationError>>;
