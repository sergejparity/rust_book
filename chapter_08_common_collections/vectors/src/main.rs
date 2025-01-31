fn main() {
    // Ways to create a new vector
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3: Vec<i32> = vec![1, 2];

    // Add elements to a vector
    v3.push(5);
    v3.push(56);
    v3.push(57);

    // Access elements of a vector
    let first = &v3[0];
    println!("Third element of v3 is: {} and 1st is {}", &v3[2], first);

    let fourth: Option<&i32> = v3.get(3);
    match fourth {
        Some(fourth) => println!("Fourth element of v3 is: {}", fourth),
        None => println!("There is no fourth element in v3"),
    }

    // Iterate over a vector with immutable references
    for i in &v3 {
        println!("{}", i);
    }

    // Iterate over a vector and modify its elements
    for i in &mut v3 {
        *i += 50;
        println!("{}", i);
    }

    // Using an enum to store multiple types in a vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
