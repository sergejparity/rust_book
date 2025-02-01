use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{}\n", score);

    print_hashmap(&scores);

    scores.insert(String::from("Blue"), 25); // overwrite the value

    print_hashmap(&scores);

    scores.entry(String::from("Yellow")).or_insert(150);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    print_hashmap(&scores);

    update_score(&mut scores, "Blue", 100);
    update_score(&mut scores, "Red", 9);
    update_score(&mut scores, "Yellow", 18);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // println!("{}", field_name); // error[E0382]: borrow of moved value: `field_name`
}

fn print_hashmap<K, V>(map: &HashMap<K, V>)
where
    K: std::fmt::Display,
    V: std::fmt::Display,
{
    for (key, value) in map {
        println!("{}: {}", key, value);
    }
    println!();
}

fn update_score(scores: &mut HashMap<String, i32>, team_name: &str, score: i32) {
    let old_score = scores.entry(String::from(team_name)).or_insert(0);
    *old_score += score;
    print_hashmap(&scores);
}