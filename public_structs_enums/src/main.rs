fn main() {
    // fields are private by default and will have to mark it as pub along with any functions we are going to use a pub if needed
    // enums when made public all of its variants are public
    let rect: shape::Rectangle = shape::Rectangle::new(1.2, 3.4);
    println!("rect area is {}", rect.get_area());
    println!("rect width is {}", rect.width);

    let cirle: shape::Shape = shape::Shape::Circle;
}

mod shape {

    pub struct Rectangle {
        pub width: f64,
        height: f64,
    }

    pub enum Shape {
        Rectangle,
        Circle,
        Triangle,
    }
    impl Rectangle {
        pub fn new(width: f64, height: f64) -> Rectangle {
            Rectangle { width, height }
        }

        pub fn get_area(&self) -> f64 {
            self.width * self.height
        }
    }
}
