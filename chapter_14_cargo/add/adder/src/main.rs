use add_one;
use add_two;
use rand;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}", num, add_one::add_one(num));
    println!("add_two of {} is {}", num, add_two::add_two(num));
}
