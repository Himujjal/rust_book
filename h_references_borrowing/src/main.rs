fn main() {
    let s1 = String::from("hello"); // s1 comes into scope
    let len = calculate_length(&s1); // address of s1 passed

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello2");
    change_ref(&mut s2);
    println!("s2: {}.", s2);
    s2.push_str(" + Hello");
    println!("s2: {}.", s2);

    // only one mutable reference is allowed
    let mut s3 = String::from("hello3");
    let r1 = &mut s3;
    // let r2 = &mut s3;       // second mutable borrow will fail

    // println!("{}, {}", r1, r2);
    println!("r1: {}", r1);

    {
        let r2 = &mut s3;
        println!("This works: {}", r2);
        // println!("This works: {}", r1); // second mutable in the same scope
    }

    borrowing_mutable();

    let _reference_to_nothing = dangle();
}

fn borrowing_mutable() {
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // // let r3 = &mut s; // BIG PROBLEM - mutable borrow occurs here

    // println!("{}, {}, and ", r1, r2);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
} // s goes out of scope. its memory is dropped. Danger!!

// takes address of a String variable as arg
fn calculate_length(s: &String) -> usize {
    s.len()
}

// needs to be a mutable reference
fn change_ref(some_string: &mut String) {
    some_string.push_str(", world")
}
