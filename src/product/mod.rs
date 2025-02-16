pub mod category;  // Importa il modulo `category`

#[derive(Debug)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub category: category::Category, // Usa `category::Category`
}

impl Product {
    pub fn new(id: u32, name: String, price: f64, category: category::Category) -> Self {
        Product {
            id,
            name,
            price,
            category,
        }
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }
}
