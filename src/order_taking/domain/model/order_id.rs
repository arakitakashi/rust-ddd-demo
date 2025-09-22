#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId(String);

impl OrderId {
    pub fn create(id: String) -> Self {
        if id.trim().is_empty() {
            panic!("OrderId must not be empty");
        } else if id.len() > 50 {
            panic!("OrderId must not be more than 50 chars");
        } else {
            OrderId(id)
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
