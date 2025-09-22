#[derive(Debug, Clone)]
pub struct String50(String);

impl String50 {
    pub fn create(value: String) -> Self {
        if value.len() > 50 {
            panic!("String must not be more than 50 chars");
        }
        String50(value)
    }

    pub fn create_option(value: Option<String>) -> Option<String50> {
        match value {
            Some(s) if !s.trim().is_empty() => Some(String50::create(s)),
            _ => None,
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
