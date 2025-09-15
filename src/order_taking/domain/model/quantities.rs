#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitQuantity(i32);

impl UnitQuantity {
    pub fn create(quantity: i32) -> Result<UnitQuantity, String> {
        if quantity < 1 {
            Err("UnitQuantity can not be negative".to_string())
        } else if quantity > 1000 {
            Err("UnitQuantity can not be more than 1000".to_string())
        } else {
            Ok(UnitQuantity(quantity))
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KilogramQuantity(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}
