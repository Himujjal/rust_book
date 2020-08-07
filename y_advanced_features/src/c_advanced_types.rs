pub fn main() {
    println!("---------B_ADVANCED_TYPES-----------");
    type_alias();
    dynamic_size_types();
}

fn type_alias() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let _f: Thunk = Box::new(|| println!("hi!"));

    fn _takes_long_type(_f: Thunk) {
        // --- snip ----
    }
}

fn dynamic_size_types() {
    let _s1: &str = "hello there!";
    let _s2: &str = "How's it going?";
}
