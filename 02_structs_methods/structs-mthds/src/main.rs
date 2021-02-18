// Inherits std::fmt::Debug for the next struct
#[derive(Debug)]
// Creates the rectangle struct with w and h float attributes
struct Rectangle {
    w: f64,
    h: f64,
}

// Creates methods for Rectangle struct
// impl means implementation
impl Rectangle {
    // Borrows the attributes from a Rectangle instance
    // Returns a f64 object
    fn area(&self) -> f64 {
        self.w * self.h
    }

    fn contains(&self, o: &Rectangle) -> bool {
        // Borrow thw other rectangle instead of taking ownership
        // if the rect executing the method has a greater height
        // and a greater width than the other rectangle, it will return true
        self.w > o.w && self.h > o.h
    }
}


fn main() {
    // Create an instance of a rectangle
    let rect1 = Rectangle {w: 30.2, h: 16.6};
    let rect2 = Rectangle {w: 20.1, h: 10.2};
    println!(
        "The area of the rectangle is {}", rect1.area()
    );

    if rect1.contains(&rect2) {

        println!(
            "The first rectangle can contain a {} by {} rectangle",
            rect2.w, rect2.h
        );
    } else {
        println!(
            "The first rectangle cannot contain a {} by {} rectangle",
            rect2.w, rect2.h
        );
    }

}
