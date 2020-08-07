enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn print_list(list: Box<List>) {
    match *list {
        List::Cons(i, list) => {
            println!("{}, ", i);
            print_list(list);
        }
        List::Nil => {
            println!("Nil");
        }
    }
}

pub fn boxed() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    print_list(Box::new(list));
}
