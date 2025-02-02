// Those two functions below are doing the same thing, but with different types
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// Function below will replace the two functions above using generics
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // T should implement std::cmp::PartialOrd in order to accept only types that can be compared
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Struct with generic types
struct Point<T, U> {
    // T and U are generic types. They can be any type. And they can be different types e.g. i32 and f64
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

struct PointMixed<X1, Y1> {
    x: X1,
    y: Y1,
}

// Example of a method to mixup two PointMixed structs
// it leaves the x value of the first struct and the y value of the second struct
impl<X1, Y1> PointMixed<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointMixed<X2, Y2>) -> PointMixed<X1, Y2> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}

// Enum with one generic type
enum Option<T> {
    Some(T),
    None,
}

// Enum with two generic types
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 3.0, y: 4.0 };
    let mixed = Point { x: 5, y: 4.9 };
    println!("integer.x = {} and integer.y = {}", integer.x, integer.y);
    println!("float.x = {} and float.y = {}", float.x, float.y);
    println!("mixed.x = {} and mixed.y = {}", mixed.x, mixed.y);

    let p = Point { x: 15.35, y: 10 };
    println!("p.x = {}", p.x());
    println!("Distance from origin: {}", float.distance_from_origin());

    let p1 = PointMixed { x: 5, y: 10.4 };
    let p2 = PointMixed { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {} and p3.y = {}", p3.x, p3.y);
}
