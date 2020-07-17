fn main() {
    let s = String::from("Hello world");
    let word = first_word(&s);
    println!("{}", word);
    // word is valid but there is no s = "Hello World"

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice1 = &s[..2]; // [0..2]
    let slice2 = &s[3..s.len()];
    println!("{}, {}", slice1, slice2);

    println!("{}, {}", hello, world);

    let s1 = String::from("hello world");

    let first_w = first_word(&s1);
    println!("The first word is: {}", first_w);
}

// fn second_word(s: &String) -> &str {}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == ' ' as u8 {
            return &s[0..i];
        }
    }

    &s[..]
}
