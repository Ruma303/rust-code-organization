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
    crate::instruments::guitar();
}

//% Moudules accessibility

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
