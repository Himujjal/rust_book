#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", loopback);

    example_2();

    enum_methods();

    option_method();

    match_example();

    if_let_examples();
}

fn if_let_examples() {
    println!("-------- if_let_examples ------------");
    let some_u8_value: Option<u8> = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        None => println!("It is none"),
        _ => (),
    };

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("else block of if-else");
    }
}

fn match_example() {
    println!("--------- match_example ---------");

    #[derive(Debug)]
    #[allow(dead_code)]
    enum UsState {
        Alabama,
        Alaska,
    }

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        let coin_type = match coin {
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // state here is an US state
                println!("State quarter from {:?}!", state);
                25
            }
        };
        coin_type
    }

    let count = value_in_cents(Coin::Penny);
    println!("{}", count);

    let another_coin = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Another coin: {}", another_coin);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    match _none {
        None => println!("None produced!!"),
        Some(i) => println!("{}", i),
    };

    // ------ the _ placeholder ----------
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("lewl!!"),
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}

fn example_2() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    enum _IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loop_back = IpAddr::V6(String::from("::1"));
}

fn enum_methods() {
    println!("----- enum_methods ------");

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&mut self) {
            match self {
                Message::Quit => println!("Quit!"),
                Message::Move { x, y } => println!("Struct, {}, {}", x, y),
                Message::Write(str) => println!("{}", str),
                Message::ChangeColor(i, j, k) => println!("{}, {}, {}", i, j, k),
            }
        }
    }

    let mut m = Message::Write(String::from("hello"));
    m.call();
    m = Message::ChangeColor(1, 2, 3);
    m.call();
    m = Message::Quit;
    m.call();
    m = Message::Move { x: 1, y: 2 };
    m.call();
}

fn option_method() {
    // Match uses Some and None. Let's check it out
    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;

    let _x: i8 = 5;

    // let _sum = _x + some_number;
}
