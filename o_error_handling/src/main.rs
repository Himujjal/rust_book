// run with `$env:RUST_BACKTRACE=1 ; cargo run`
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    unrecoverable_errors();
    recoverable_errors();
    panic_or_not_panic();
}

fn unrecoverable_errors() {
    let _v = vec![1, 2, 3];

    // v[99]; // panic from the library
}

fn recoverable_errors() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    fn _read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_username_from_file_2() -> Result<String, io::Error> {
        // '?': returns an error on error for the whole function
        //      or open another file
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    match read_username_from_file_2() {
        Ok(s) => println!("{}", s),
        Err(_) => {
            println!("Error!");
        }
    };
}

fn panic_or_not_panic() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read from input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess_obj = Guess::new(guess);

        println!("The read value is: {}", guess_obj.value());
    }
}
