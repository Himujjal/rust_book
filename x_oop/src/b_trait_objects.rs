use std::io::{self, Write};

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // drawing a button
        for _ in 0..(self.label.len() + 4) {
            print!("_");
        }
        print!("\n| {} |\n", self.label);
        for _ in 0..(self.label.len() + 4) {
            print!("-");
        }
        io::stdout().flush().unwrap();
        println!();
    }
}

struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // drawing a SelectBox
        println!("___________\n| SELECT  |\n-----------");
        for option in self.options.iter() {
            println!("{}", option);
        }
    }
}

pub fn main() {
    println!("---------B_TRAIT_OBJECTS----------");
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes!"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
