use std::collections::HashMap as Map;
use std::{self, io}; //, auto-import

//% Simple modules
mod instruments {
    //, Public function
    pub fn guitar() {
        println!("Playing guitar");
    }

    //, Private function
    fn piano() {
        println!("Playing piano");
    }

    //, Getter
    pub fn get_piano() {
        self::piano();
    }

    //% Nested modules
    pub mod electric {
        pub fn guitar() {
            println!("Playing electric guitar");
        }
    }
}

// use: Generalmente ad inizio file
use crate::my_module::function_a;

fn main() {
    instruments::guitar();
    instruments::get_piano();
    instruments::electric::guitar();

    crate::instruments::guitar();

    //, crate
    crate::module_a::module_b::greet();

    //, super
    crate::grandparent::parent::parent_function();
    crate::grandparent::grandparent_function();
    crate::grandparent::parent::child::call_grandparent();

    //, self
    crate::my_module::function_a();
    crate:: my_module::function_b();

    //, use
    use crate::my_module::function_a;
    function_a();

    // use: O comunque prima del suo utilizzo
    use crate::my_module::function_b;
    function_b();
    function_b();

    //, Libreria standard
    let mut map: Map<String, i32> = Map::new();
    map.insert("key".to_string(), 42);
    println!("{:?}", map);


    //, auto-import
    let mut input = String::new();
    println!("Input: ");

    let content: usize = io::stdin().read_line(&mut input).unwrap();
    println!("{}", content);

    println!("{}", std::env::var("HOME").unwrap_or_default());

}

//, crate (binario)
mod module_a {
    pub mod module_b {
        pub fn greet() {
            println!("Hello from module_b!");
        }
    }
}

//, super
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

//, self
mod my_module {
    pub fn function_a() {
        println!("Called function_a");
    }

    pub fn function_b() {
        self::function_a(); // Accede a function_a
    }
}




//# Modules accessibility

//* Generic modules
pub mod public_module {
    pub fn public_function() {}
}

//* Crate modules
pub(crate) mod public_crate_module {
    pub struct PublicStruct;
    struct PrivateStruct;

    //* Super modules
    pub(super) mod public_super_module {
        pub struct PublicStruct;
        struct PrivateStruct;
    }

    pub mod child {
        pub use crate::instruments::electric::guitar as electric_guitar;
        pub(in crate::public_crate_module) fn restricted_function() {
            println!("Accessible only in parent module");
            electric_guitar();
        }
    }
}
