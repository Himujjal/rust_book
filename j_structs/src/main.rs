#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        // entire struct has to be mutable
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("a@b.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);

    let user3 = build_user(String::from("a@b.com"), String::from("password"));

    println!("{:#?},{:?},{:?}, {:?}", user2, black, origin, user3);
    area_example();

    methods();
}

fn methods() {
    println!("------ Methods ----------");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // implementation block defines methods of a struct
    // type of self is Rectangle
    // impl blocks can be multiple. but let's not do that
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn square(size: u32) -> Rectangle {
            let rect = Rectangle {
                width: size,
                height: size,
            };
            rect
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 60,
    };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect1));

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    let sq1 = Rectangle::square(2);

    println!("{:#?}", sq1);
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area_example() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area2(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area3(&rect2)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(r: &Rectangle) -> u32 {
    r.height * r.width
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
