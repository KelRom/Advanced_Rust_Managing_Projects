fn main() {

    // by default all modules are private
    // parent modules cannot use private items in a child module
    // absolute path, starts from the root crate
    // crate::hello::english();
    // crate::hello::spanish();
    // crate::hello::casual::english(); 

    // relative path
    hello::english();
    hello::casual::english();
}

mod hello {
    pub fn english() {
        println!("hello");
        
        // relative
        spanish(); // this works since it is within the module even though it is private
        // casual::english();
    }

    fn spanish() {
        println!("hola");
    }

    pub mod casual {
        pub fn english() {
            println!("hey");
            // absolute path
            //crate::hello::spanish();
            // relative path
            super::spanish();
        }
    }
}