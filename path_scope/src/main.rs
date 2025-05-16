// bring a path/module into scope, this specifically brings the spanish function into scope
use crate::greeting::{formal, casual};

fn main() {
    greeting::formal::english();

    // works since module was brought into scope no need to type greeting anymore
    formal::spanish();
    casual::spanish();
}


mod greeting {
    pub mod formal
    {
        pub fn english() -> () {
            println!("hello");
        }

        pub fn spanish() -> () {
            println!("hola");
        }
    }

    pub mod casual {
        pub fn english() -> () {
            println!("hey");
        }

        pub fn spanish() -> () {
            println!("oye");
        }
    }

}