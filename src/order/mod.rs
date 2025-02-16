pub mod status; // Importa `order_status`

use crate::customer::Customer;
use crate::product::Product;
use self::status::OrderStatus; // Importa `OrderStatus` dal sotto-modulo

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub products: Vec<Product>,
    pub customer: Customer,
    pub status: OrderStatus,
    pub quantity: u32,
    pub total_price: f64,
    pub shipping_address: String,
    pub tracking_number: String,
}

impl Order {
    pub fn new(customer: Customer, products: Vec<Product>, status: OrderStatus, quantity: u32, shipping_address: String, tracking_number: String) -> Self {
        let total_price = products.iter().map(|p| p.get_price()).sum();
        Order {
            id: 0,
            products,
            customer,
            status,
            quantity,
            total_price,
            shipping_address,
            tracking_number,
        }
    }

    pub fn calculate_total_price(&self) -> f64 {
        self.products.iter().map(|p| p.get_price()).sum()
    }

    pub fn calculate_shipping_cost(&self) -> f64 {
        let prices: Vec<f64> = self.products.iter().map(|p| p.get_price()).collect();
        println!("Prezzi prodotti: {:?}", prices);

        prices.iter().sum::<f64>() * 0.05
    }

    pub fn get_order_customer(&self) -> &Customer {
        &self.customer
    }
}
