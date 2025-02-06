// Examples to 15-3: Implementing the Drop Trait

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // will drop after the block ends and before the println! below
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // will be dropped explicitly
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // will be dropped implicitly after main ends
    let e = CustomSmartPointer {
        data: String::from("more stuff"),
    };

    {
        let x = c;
    }
    println!("CustomSmartPointers created.");
    // println!("CustomSmartPointer c: data `{}`", c.data); // error[E0382]: borrow of moved value: `c` it was dropped in the block above
    // let x = c;
    drop(d);
    println!("Finish main.");
}
