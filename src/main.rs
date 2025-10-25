//% Simple modules
mod instruments {

    //# Public function
    pub fn guitar() {
        println!("Playing guitar");
    }

    //# Private function
    fn piano() {
        println!("Playing piano");
    }

    //# Getter
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
}
