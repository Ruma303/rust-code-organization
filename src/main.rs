// Pre 2018
// extern crate rand;

// Post 2018
use rand::Rng;

// Usi di extern crate
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

lazy_static! {
  #[derive(Debug)]
	static ref DATA: HashMap<&'static str, i32> = {
		let mut m = HashMap::new();
		m.insert("key", 42);
		m
	};
}


fn main() {
  // Pre 2018
  // let value = rand::random::<u8>();
  // println!("Random value: {}", value);

  // Post 2018
  let mut rng = rand::thread_rng();
  let value = rng.gen::<u8>();
  println!("Random value: {}", value);

  println!("Data: {:?}", DATA);
}


// 
#[test]
fn test_random() {
    let x = rand::random::<u32>();
    assert!(x < 256);
}