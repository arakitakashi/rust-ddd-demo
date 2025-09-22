#[derive(Debug, Clone)]
pub struct UnvalidatedCustomerInfo {
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
}

#[derive(Debug, Clone)]
pub struct PersonalName {
    pub first_name: String50,
    pub last_name: String50,
}

#[derive(Debug, Clone)]
pub struct ValidatedCustomerInfo {
    pub name: PersonalName,
    pub email_address: EmailAddress,
}

#[derive(Debug, Clone)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn create(value: String) -> Self {
        if !value.contains('@') {
            panic!("Invalid email address");
        }
        EmailAddress(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub fn to_validated_customer_info(customer: UnvalidatedCustomerInfo) -> ValidatedCustomerInfo {
    let first_name = String50::create(customer.first_name);
    let last_name = String50::create(customer.last_name);
    let email_address = EmailAddress::create(customer.email_address);
    let name = PersonalName {
        first_name,
        last_name,
    };
    let customer_info = ValidatedCustomerInfo {
        name,
        email_address,
    };

    customer_info
}
