fn main() {
    let r: i32;

    // {
    //     let x = 5;
    //     r = &x;
    // }
    let l = longest("hellos", "world");
    let l2 = longest2("hellos", "world");
    for _ in 1..11 {
        println!("l: {}", l);
        println!("l: {}", l2);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    // let result = String::from("really long string");
    let result = x;
    result
}

fn longest3<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
