use rust_code_organization::{test_connection, connect, disconnect};

use rust_code_organization::utils::{parse_input, format_output};

fn main() {
    // Modeless
    test_connection();
    
    println!("\nSingle connection test...!");
    connect();
    
    println!("\nSingle connection test...!");
    disconnect();
    
    // Uso di mod.rs
    println!("{:?}", parse_input("input").unwrap());
    println!("{:?}", format_output("output"));
}
