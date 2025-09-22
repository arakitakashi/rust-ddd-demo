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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KilogramQuantity(f64);

impl KilogramQuantity {
    pub fn create(quantity: f64) -> Result<KilogramQuantity, String> {
        if quantity < 1 {
            Err("KilogramQuantity can not be negative".to_string())
        } else if quantity > 1000 {
            Err("KilogramQuantity can not be more than 1000".to_string())
        } else {
            Ok(KilogramQuantity(quantity))
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}

pub fn to_order_quantity(
    product_code: &ProductCode,
    quantity: &str,
) -> Result<OrderQuantity, String> {
    match product_code {
        ProductCode::Widget(_) => {
            let parsed_quantity = quantity
                .parse::<i32>()
                .map_err(|_| "Invalid quantity format".to_string())?;

            let unit_quantity = UnitQuantity::create(parsed_quantity)?;
            Ok(OrderQuantity::Unit(unit_quantity))
        }

        ProductCode::Gizmo(_) => {
            let parsed_quantity = quantity
                .parse::<f64>()
                .map_err(|_| "Invalid quantity format".to_string())?;

            let kiroguram_quantity = KilogramQuantity::create(quantity);
            Ok(OrderQuantity::Kilogram(kiroguram_quantity))
        }
    }
}
