fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut v_iter = v.iter();
    println!("v_iter: {:?}", v_iter.next());
    println!("v_iter: {}", v_iter.next().unwrap());
    for val in v_iter {
        println!("Got: {}", val);
    }
    let mut v_iter = v.iter();
    let total: i32 = v_iter.sum();
    println!("Total: {}", total);
    let v2: Vec<_> = v.iter().map(|x| x * 10).collect();
    println!("v2: {:?}", v2);
}
