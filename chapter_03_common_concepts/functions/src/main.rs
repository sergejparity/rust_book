fn main() {
    println!("Hello, world!");

    another_function(5, 'h');
    // nested_function(); not visible here

    // Assigning an expression in a block to a variable
    let y = {
        let x = 3;
        x + 1 // No semicolon here, otherwise it would be a statement
    };
    println!("The value of y is: {}", y);

    println!("five returning value is: {}", five());
}

fn another_function(value: i32, character: char) {
    println!("Another function: {} - {}", value, character);

    fn nested_function() {
        println!("Nested function.");
    }

    nested_function();
}

// Function with return value
fn five() -> i32 {
    return 5 + 6;
    // 5 + 6 // This is also valid
}