fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => Some(100500),
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);

    //
    let config_max = Some(44u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // the same as above but with if let
    let config_max = Some(22u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}