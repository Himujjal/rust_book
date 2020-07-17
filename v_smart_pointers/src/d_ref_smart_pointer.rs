fn _trying_with_box() {
  enum List {
    Cons(i32, Box<List>),
    Nil,
  }

  let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
  let _b = List::Cons(3, Box::new(a));
  // let c = List::Cons(4, Box::new(a)); // multiple owners not possible
}

fn _trying_with_rc() {
  use std::rc::Rc;

  enum List {
    Cons(i32, Rc<List>),
    Nil,
  }

  use List::{Cons, Nil};
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let _b = Cons(3, Rc::clone(&a));
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let _c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

pub fn _ref_smart_pointers_main() {
  println!("---------_ref_smart_pointers_main------------");

  _trying_with_rc();
}
