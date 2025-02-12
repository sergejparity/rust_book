fn main() {
    // "if let" example
    let favorite_color: Option<&str> = None; // Some("red");
    let is_tuesday = false;
    let age: Result<u8, _> = "ssds34".parse();

    // This will resolves as:
    // if color specified condition will be used if None skipped
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        // next evaluation
        println!("Tuesday is green day!");
    } else if let Ok(age) = age { // if we try "&& age > 30" - then  error[E0658]: `let` expressions in this position are unstable
        // if age can be successfully parsed from a str
        if age > 30 {
            // nesting necessary as "if let Ok(age) && age > 30" as shadowed if let Ok(age) is not valid until a {}
            // block after it
            println!("using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        // will be used if none of the conditions from above matched
        println!("Using blue as the background color");
    }

    // "while let" example
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // for loop example
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }


    // Functions parameters also a pattern
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Curent location: {x}, {y}");
}