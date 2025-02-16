#[derive(Debug)]
pub struct Customer {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub phone: String,
}

impl Customer {
    pub fn new(id: u64, name: String, email: String, phone: String) -> Self {
        Customer {
            id,
            name,
            email,
            phone,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_phone(&self) -> &str {
        &self.phone
    }
}
