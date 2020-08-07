pub fn main() {
    println!("---------A_UNSAFE_TRAITS-----------");
    raw_pointers();
    ffi();
    mutating_static();
    mutating_static_2();
    unsafe_trait();
}

fn raw_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn ffi() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C");
}

static HELLO_WORLD: &str = "Hello World";

fn mutating_static() {
    println!("name is : {}", HELLO_WORLD);
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn mutating_static_2() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn unsafe_trait() {
    unsafe trait Foo {}

    unsafe impl Foo for i32 {}
}
