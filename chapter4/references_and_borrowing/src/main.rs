fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");

    let r1 = &s;
    println!("{r1}");
    let r2 = &s;
    println!("r1 {r1} and r2 {r2}"); // r1 and r2 are immutable references and r1 is still in scope
    let r3 = &mut s;
    println!("r3 {r3}"); // r3 is a mutable reference and r1 and r2 is no longer in scope
    let r4 = &s;
    println!("r4 {r4}"); // r4 is an immutable reference and r3 is no longer in scope
    // let r5 = &mut s + "5";
    // println!("r5 {r5}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}