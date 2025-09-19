use super::ProductCode;

pub type Price = ();

pub type GetProductPrice = fn(ProductCode) -> Price;
