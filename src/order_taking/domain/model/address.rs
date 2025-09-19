use std::future::Future;
use std::pin::Pin;

pub type UnvalidatedAddress = ();
pub type ValidatedAddress = ();

#[derive(Debug, Clone)]
pub enum Address {
    UnvalidatedAddress(UnvalidatedAddress),
    ValidatedAddress(ValidatedAddress),
}

// TODO: Futureの理解
pub type CheckAddressExists =
    fn(
        UnvalidatedAddress,
    ) -> impl Future<Output = Result<ValidatedAddress, AddressValidationError>>;

#[derive(Debug, Clone)]
pub struct AddressValidationError(pub String);
