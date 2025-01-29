fn main() {
    let number = 33;

    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false.");
    }

    let condition = false;
    let numberx = if condition { 5 } else { 7 };
    println!("The value of number is: {}", numberx);

    loop_example();

    println!("\nLoop example 2:\n");
    loop_example2();

    println!("\nWhile example:\n");
    while_example();

    println!("\nWhile example 2:\n");
    while_example2();

    println!("\nFor example:\n");
    for_example();

    println!("\nFor example 2:\n");
    for_example2();
}

fn loop_example() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            println!("I'm at {counter}, I'm stopping.");
            break counter;
        }
        println!("I'm gonna stop at 10 but I'm at: {}", counter);
    };

    println!("The result is: {}", result);
}


fn loop_example2() {
    let mut counter = 0;
    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End: counter = {counter}");
}

fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn while_example2() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn for_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_example2() {
    for number in 1..11 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}