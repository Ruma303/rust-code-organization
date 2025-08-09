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

fn main() {
    instruments::guitar();
    instruments::get_piano();
    instruments::electric::guitar();

    crate::instruments::guitar(); // Uso di crate

    use grandparent::parent::child::call_grandparent;
    call_grandparent();

    use my_module::function_b;
    function_b();
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
            use super::super::grandparent_function; // Risale di 2 livelli

            pub fn call_grandparent() {
                grandparent_function(); // Chiama direttamente grandparent_function
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
        self::function_a(); // Usa self per accedere a function_a
    }
}

//% Modules accessibility

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
