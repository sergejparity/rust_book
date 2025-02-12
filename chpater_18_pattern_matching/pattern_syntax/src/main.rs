struct Point {
    x: i32,
    y: i32,
}


fn main() {
    // Matching literals
    println!("=== Matching literals ====");
    let x = 2;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("======= end =======\n");

    // Matching Named Variables
    println!("======= Matching Named Variables =======");
    let x = Some(5);
    // let x = None;
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // y in Some(y) is a new shadowed variable and
                                                 // it will match to x, not y from before the block
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
    println!("======= end =======\n");

    // Matching multiple patterns
    println!("=== Matching multiple patterns ====");
    let x = 5;

    match x {
        1 | 2 => println!("Matched multiple pattern: '1 | 2'"), // Matching multiple patterns
        3..=7 => println!("Matched range: '3..=7'"), // Matching ranges
        8 => println!("Matched single pattern: 8"),
        _ => println!("anything"),
    }

    println!("======= end =======\n");

    // DESTRUCTURING
    // Destructuring structs
    println!("======= Destructuring structs =======\n");

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // breaking Point apart into 2 variables a and b
    let Point { x, y } = p; // its equal to 'let Point { x: x, y: y } = p;'
    // let Point { m, n } = p; - WRONG. Names should match original struct names
    println!("a: {a}, b: {b}");
    assert_eq!(0, a);
    assert_eq!(7, b);
    println!("x: {x}, y: {y}");

    //
    let xp = Point { x: 0, y: 7 };
    let yp = Point { x: 3, y: 0 };
    let zp = Point { x: 33, y: 1 };
    let zero = Point { x: 0, y: 0 };

    match_point(xp);
    match_point(yp);
    match_point(zp);
    match_point(zero);

    println!("======= end =======\n");

    // Destructuring enums
    println!("======= Destructuring enums =======\n");


    println!("======= end =======\n");

    // Complex destructuring
    //    (2 val tuple)  (Point)
    // let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    let test = 5;
    if test == test {
        println!("yes");
    } else {
        println!("no");
    }

}

// Match destructured point
fn match_point(p: Point) {
    // Matching points
    match p {
        Point { x: 0, y: 0 } => println!("At the center!"),
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}