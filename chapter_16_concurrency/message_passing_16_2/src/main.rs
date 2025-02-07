use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // let handle =
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        // thread::sleep(Duration::from_millis(1));
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
        // println!("val is {val}"); // This will fail - val already borrowed use handle.join().unwrap(); to enable it
    });

    // handle.join().unwrap(); // this will allow thread to run after sending a message
    // let received = rx.recv().unwrap(); // will receive only one message
    for received in rx {
        println!("Got: {received}");
    }
}
