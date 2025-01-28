fn main() {
    // Mutability example
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing example
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Shadowing example with type change
    let spaces = "    "; // String
    let spaces = spaces.len(); // Integer
    println!("The value of spaces is: {spaces}");

}
