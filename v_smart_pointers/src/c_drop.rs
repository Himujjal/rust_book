use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn dropped() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff!"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff!"),
    };

    println!("CustomSmartPointers created.");
    drop(_c);
    println!("Custom SmartPointer dropped before the end of main");
}
