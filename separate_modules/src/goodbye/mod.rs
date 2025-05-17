pub fn description() -> () {
    println!("goodbye messages");
}

pub mod formal {

    pub fn english() -> () {
        println!("goodbye");
    }

    pub fn spanish() -> () {
        println!("adios");
    }
}

pub mod casual;
