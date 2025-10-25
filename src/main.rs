use std::collections::HashMap as Map;
use std::{self, io}; //# auto-import

// use: Generalmente ad inizio file
use crate::my_module::function_a;

fn main() {
    //# crate
    crate::module_a::module_b::greet();

    //# super
    crate::grandparent::parent::parent_function();
    crate::grandparent::grandparent_function();
    crate::grandparent::parent::child::call_grandparent();

    //# self
    crate::my_module::function_a();
    crate::my_module::function_b();

    //# use
    use crate::my_module::function_a;
    function_a();

    // use: O comunque prima del suo utilizzo
    use crate::my_module::function_b;
    function_b();
    function_b();

    //# Libreria standard
    let mut map: Map<String, i32> = Map::new();
    map.insert("key".to_string(), 42);
    println!("{:?}", map);

    //# auto-import
    /* let mut input = String::new();
    println!("Input: ");

    let content: usize = io::stdin().read_line(&mut input).unwrap();
    println!("{}", content);

    println!("{}", std::env::var("HOME").unwrap_or_default()); */

    //# pub struct
    let person = struct_module::Person::new("Alice", 30);
    println!("{}", person.name); // name è privato
                                 // println!("{}", person.age); // ERRORE: age è privato

    //# pub enum
    let color = enum_module::Color::Red;
    match color {
        enum_module::Color::Red => println!("Red"),
        enum_module::Color::Green => println!("Green"),
        enum_module::Color::Blue => println!("Blue"),
    }

    //# Accessor methods
    let mut user = User::new(1, "John".to_string());
    println!("{}", user.get_id()); // 1
    println!("{}", user.get_name()); // "John"

    user.set_id(2);
    user.set_name("Jane".to_string());
    println!("{}", user.get_id()); // 2
    println!("{}", user.get_name()); // "Jane"

    //# Modules accessibility
    parent::test(); // ✅ Funziona
                    // parent::child::only_parent_can_call(); ❌ ERRORE
    parent::test2(); // ✅ Funziona

    // parent::restricted_function();  ❌ ERRORE
    parent::test3(); // ✅ Funziona
}

//# crate (binario)
mod module_a {
    pub mod module_b {
        pub fn greet() {
            println!("Hello from module_b!");
        }
    }
}

//# super
mod grandparent {
    pub fn grandparent_function() {
        println!("Called grandparent_function");
    }

    pub mod parent {
        pub fn parent_function() {
            println!("Called parent_function");
        }

        pub mod child {
            pub fn call_grandparent() {
                super::super::grandparent_function(); // Risale di 2 livelli
            }
        }
    }
}

//# self
mod my_module {
    pub fn function_a() {
        println!("Called function_a");
    }

    pub fn function_b() {
        self::function_a(); // Accede a function_a
    }
}

//# pub struct
mod struct_module {
    pub struct Person {
        pub name: String, // pubblico
        age: u8,          // Privato di default
    }

    impl Person {
        pub fn new(name: &str, age: u8) -> Self {
            Self {
                name: name.to_string(),
                age,
            }
        }
    }
}

//# pub enum
mod enum_module {
    pub enum Color {
        Red,
        Green,
        Blue,
    }
}

//# Accessor methods
#[derive(Debug)]
struct User {
    // ❌ Campi privati per sicurezza
    id: u32,
    name: String,
}

impl User {
    // ✅ Costruttore pubblico
    pub fn new(id: u32, name: String) -> Self {
        User { id, name }
    }

    // ✅ Getter pubblici
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    // ✅ Setter privati (modificano lo stato)
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

//# Modules accessibility

//* pub crate
pub(crate) fn only_in_crate() {
    println!("Solo all'interno dello stesso crate");
}

//* pub super
mod parent {
    mod child {
        pub(super) fn only_parent_can_call() {
            println!("Only parent module can call this");
        }
    }

    pub fn test() {
        child::only_parent_can_call();
    }

    pub fn test2() {
        self::child::only_parent_can_call();
    }

    //* pub(in path)
    mod restricted_mod {
        pub(in crate::parent) fn restricted_function() {
            println!("Accessibile solo in parent");
        }

        // Ancora accessibile
        pub fn test3() {
            restricted_function();
        }
    }

    pub fn test4() {
        restricted_mod::restricted_function();
    }

    pub fn using_extra() {
      crate::extra_for_parent::test5();
    }
}

// Errore di sintassi non più supportato dall'edizione 2018
/* pub(in parent) fn test5() {
    println!("Accessibile solo in parent");
}*/

// Non visibile in quanto non è definita all'interno di parent
/* pub(in crate::parent) fn test5() {
    println!("Accessibile solo in parent");
} */

pub mod extra_for_parent {
  pub fn test5() {
      println!("Accessibile solo in parent");
  }
}
