fn main() {
    let s = String::from("Hello, world!");

    let slice1 = &s[0..5];
    let slice2 = &s[7..13];
    println!("slice1: {}, slice2: {}", slice1, slice2);

    let slice3 = &s[..8]; // from the start to the 8th index
    let slice4 = &s[3..]; // from the 3rd index to the end
    let len = s.len();
    let slice5 = &s[4..len]; // from the 4th index to the end
    println!("slice3: {}, slice4: {}, slice5: {}", slice3, slice4, slice5);

    let slice6 = &s[..]; // the whole string
    let slice7 = &s[0..len]; // the whole string
    println!("slice6: {}, slice7: {}", slice6, slice7);

    // example from the book

    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);


    // Array slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let fw = first_word("everybody knows");
    println!("first word: {}", fw);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
