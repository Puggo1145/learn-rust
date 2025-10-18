struct Rectangle {
    width: u32,
    height: u32,
}

// implement associated function for a struct
impl Rectangle {
    // we can use make a method with the same name of one of the fields
    // we can use it as a getter
    fn width(&self) -> bool {
        self.width > 0
    }
    // this is a normal method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // use associated function as a constructor
    // this is similar to the concept of "static method"
    // here, Self refers to struct itself
    // and self is an instancce of a struct
    fn square(size: u32) -> Self {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 12,
        height: 24
    };
    println!("The area of the rectangle is: {} ", rect1.area());
    println!("Is width > 0: {} ", rect1.width());
    let sqr = Rectangle::square(12);
    println!("{}", sqr.width)
}
