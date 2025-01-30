struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("user 1"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("user 2"),
        active: user1.active, // Copying the value of active from user1
        sign_in_count: user1.sign_in_count, // Copying the value of sign_in_count from user1
    };

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2 // Short-hand syntax for copying the rest of the fields from user2
    };

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
    let destination = Point(10, 20, 30);

    println!("White color blue saturation: {}", white.2);

    struct UnitStruct;
    let unit = UnitStruct;
}
