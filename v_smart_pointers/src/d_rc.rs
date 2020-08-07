use std::rc::Rc;

#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

use List::{Cons, Nil};
use List2::{Cons2, Nil2};

// throws error
fn _traditional_box() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let _b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
}

fn print_list(list: &List2) {
    println!("printing list...");
    match list {
        Cons2(i, list) => {
            println!("{}, ", i);
            print_list(&list);
        }
        Nil2 => {
            println!("Nil");
        }
    }
}

// Rc::clone() does only reference increment and not deep cloning
// a.clone() does deep copying
pub fn reference_counted() {
    println!("--------------reference_counted------------------");
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
        print_list(&c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    print_list(&b);
}
