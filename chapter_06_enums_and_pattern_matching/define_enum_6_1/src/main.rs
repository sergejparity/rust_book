// Simple enum example
enum IpAddrKind {
    V4,
    V6,
}

// Enum with data associated with each variant
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

/*
This enum has four variants with different types:

Quit has no data associated with it at all.
Move has named fields, like a struct does.
Write includes a single String.
ChangeColor includes three i32 values.
 */
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// The same using structs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


// Implementing methods on enums
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
    m.call();

// The Option enum. This is a standard library enum that is included in the prelude, so you don’t need to bring it into scope.
// The Option enum is used because sometimes it’s desirable to express that a value could be absent (None) and have a value (Some) at other times.
// Documentation: https://doc.rust-lang.org/std/option/enum.Option.html
enum Option<T> {
    None,
    Some(T),
}

// 