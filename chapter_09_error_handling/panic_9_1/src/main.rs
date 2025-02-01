use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _v = vec![1, 2, 3];

    // v[99]; // This will panic because the index is out of bounds

    // Example of recovering from a panic
    //
    //
    // let greeting_file_result = File::open("hello.txt");
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(error) => panic!("Problem creating the file: {:?}", error),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     }
    // };

    // // Alternative using unwrap_or_else
    //
    // let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("ds/hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // Unwrap and expect
    // Unwrap is a shortcut method that is used when you want the program to panic if the Result is an Err value
    // let greeting_file = File::open("hello.txt").unwrap();
    //
    // Expect is similar to unwrap but allows you to specify a custom error message
    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    // 1st version: Propagating errors
    // use std::io::{self, Read};
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let username_file_result = File::open("hello.txt");

    //     let mut username_file = match username_file_result {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    //     let mut username = String::new();

    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(e),
    //     }
    // }

    // 2nd version: A Shortcut for Propagating Errors: the ? Operator (function does the same as the function above)
    // use std::io::{self, Read};

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username_file = File::open("hello.txt")?;
    //     let mut username = String::new();
    //     username_file.read_to_string(&mut username)?;
    //     Ok(username)
    // }

    // 3rd version: And even shorter by chaining the calls
    // use std::io::{self, Read};
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username = String::new();

    //     File::open("hello.txt")?.read_to_string(&mut username)?;

    //     Ok(username)
    // }

    // 4th version: Even shorter by using fs::read_to_string
    use std::fs;
    use std::io;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // This is how you would call 3 functions from above
    let username = read_username_from_file();
    match username {
        Ok(username) => println!("Username: {}", username),
        Err(e) => panic!("Error reading username: {:?}", e),
    }

    // !!!!!! we’re only allowed to use the ? operator in a function that returns Result, Option, or another type that implements FromResidual !!!!!!
}
