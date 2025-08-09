use rust_code_organization::product::{Product, Category};
use rust_code_organization::customer::Customer;
use rust_code_organization::order::{Order, status::OrderStatus};
use rust_code_organization::modules::{public_module};

fn main() {
    let p1 = Product::new(1, "Laptop".to_string(), 1000.0, Category::Electronics);
    let p2 = Product::new(2, "Mouse".to_string(), 50.0, Category::Electronics);

    let customer = Customer::new(1, "Mario Rossi".to_string(), "mario@example.com".to_string(), "123456789".to_string());

    let order = Order::new(customer, vec![p1, p2], OrderStatus::Pending, 2, "Via Roma, 10".to_string(), "123456789".to_string());

    println!("Costo spedizione: {}", order.calculate_shipping_cost());
    println!("Acquirente: {:?}", order.get_order_customer());

    public_module::public_function();
}
