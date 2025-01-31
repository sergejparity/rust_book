fn main() {
    let s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let mut s = "initial contents".to_string();
    let s2 = String::from("initial contents2");
    s.push_str(" bar ");
    s.push('X');

    let s3 = s + &s2; // s is moved here and can no longer be used

    // println!("s {} and {}", s, s2);
    println!("s3 {}", s3);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s {}", s);

    let hello = "Здравствуйте";
    let s = &hello[0..2];
    println!("s {}", &s);

    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }
}
