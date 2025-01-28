use std::io;

fn main() {
    // Tuple example
    let z = (500, 6.4, 1);
    println!("The value of z is: {} - {}", z.0, z.2);

    // Array example
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
