use rust_code_organization::product::{Product, category::Category};
// use rust_code_organization::customer::Customer;
// use rust_code_organization::order::{Order, order_status::OrderStatus};
use array_tool::vec::Intersect;

fn main() {
    let p1 = Product::new(1, "Laptop".to_string(), 1000.0, Category::Electronics);
    let p2 = Product::new(2, "Mouse".to_string(), 50.0, Category::Electronics);
    let p3 = Product::new(3, "Keyboard".to_string(), 100.0, Category::Electronics);
    let p4 = Product::new(4, "Monitor".to_string(), 200.0, Category::Electronics);

    let cart1: Vec<&Product> = vec![&p1, &p2];
    let cart2: Vec<&Product> = vec![&p2, &p3, &p4];

    let cart1plus2: Vec<&Product> = cart1.intersect(cart2);
    println!("Cart 1 + Cart 2: {:?}", cart1plus2);

    // let customer = Customer::new(1, "Mario Rossi".to_string(), "mario@example.com".to_string(), "123456789".to_string());

    // let order = Order::new(customer, vec![p1, p2], OrderStatus::Pending, 2, "Via Roma, 10".to_string(), "123456789".to_string());

    // println!("Costo spedizione: {}", order.calculate_shipping_cost());
    // println!("Acquirente: {:?}", order.get_order_customer());
}
