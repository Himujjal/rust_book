use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

pub fn _deref() {
  println!("-----Deref Trait-------");

  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);

  deref_coercion();
}

fn deref_coercion() {
  fn hello(name: &str) {
    println!("Hello, {}!", name);
  }

  let m = MyBox::new(String::from("Rust"));
  hello(&m);
  // without deref_coercion:
  // hello(&(*m)[..]);
}

pub fn _deref1() {
  println!("-----Deref Trait-------");

  let x = 5;
  let y = &x;

  assert_eq!(5, x);
  assert_eq!(5, *y);
}
