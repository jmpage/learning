#[derive(Debug)] // derived trait for debug print support
struct Rectangle {
    width: u32,
    height: u32,
}

// Defining methods, associated functions
// Note: there may be multiple impl blocks
impl Rectangle {
    // This is a method
    // &self ensures that we have an immutable reference. Without it, we would
    // take ownership of self which is unnecessary to calculate the area!
    fn area(&self) -> u32 {
        // Note: . automatically dereferences self here
        self.width * self.height
    }

    // This is an associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area() // Note: . automatically references rectangle here
    );

    // Debug print
    println!("rectangle is {:?}", rectangle); // `:#?` for pretty print

    println!("rectangle is {:?}", Rectangle::square(5));
}
