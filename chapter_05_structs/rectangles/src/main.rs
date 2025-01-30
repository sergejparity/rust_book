#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("rect1 is {:?}", rect1); // {:?} is a debug print format
    println!("rect1 is {:#?}", rect1); // {:#?} is a pretty debug print format

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    dbg!(rect1); // dbg! is a macro that prints the value of the expression and returns it. But takes ownership of the value.
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
