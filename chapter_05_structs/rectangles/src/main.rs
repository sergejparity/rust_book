#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function to create a square rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle { // Multiple impl blocks are allowed
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // let scale = 2;
    let rect1 = Rectangle {
        // width: dbg!(30 * scale),
        width: 30,
        height: 50,
    };

    let square = Rectangle::square(15);

    /* EXAMPLE OF DEBUG PRINTING
    dbg!(&rect1);

    println!("rect1 is {:?}", rect1); // {:?} is a debug print format
    println!("rect1 is {:#?}", rect1); // {:#?} is a pretty debug print format

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    dbg!(rect1); // dbg! is a macro that prints the value of the expression and returns it. But takes ownership of the value.
    */

    // USE METHOD
    if rect1.width() {
        println!("The width of the rectangle is greater than 0.");
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    } else {
        println!("The width of the rectangle is 0.");
    }

    println!("\nSquare area is {}", square.area());
}

/* Function to calculate the area of a rectangle. This function is replaced by the method area() in the Rectangle struct.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
    */
