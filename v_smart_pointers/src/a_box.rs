pub fn box_func() {
  println!("----- Using Box<T> -----");

  let b = Box::new(5);
  println!("b = {}", b);

  cons_list();
}

enum List {
  Cons(i32, Box<List>),
  Nil,
}

fn cons_list() {
  use List::{Cons, Nil};
  let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
