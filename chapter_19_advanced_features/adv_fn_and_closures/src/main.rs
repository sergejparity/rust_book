fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_as_i_say(f: fn(i32) -> i32, arg: i32, repeat: i32) -> i32 {
    let mut result = 0;
    for i in 0..repeat {
        result += f(arg);
    }
    result
}

fn main() {
    let d2answer = do_twice(add_one, 5);
    println!("do_twice answer is: {d2answer}");

    let dais = do_as_i_say(add_one, 5, 3000000);
    println!("dais answer is: {dais}");
}
