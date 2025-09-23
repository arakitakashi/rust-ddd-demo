use crate::order_taking::{Address, CheckAddressExists};

// TODO: Futureの理解:
// 非同期とエラーの両方のエフェクトを持つことを示す。
pub const CHECK_ADDRESS_EXISTS: CheckAddressExists = |_address: &Address| true;
