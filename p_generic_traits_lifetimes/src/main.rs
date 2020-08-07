use std::fmt::{Debug, Display};

fn main() {
    generic_data_types();
    traits();
    lifetimes();
}

fn generic_data_types() {
    println!("----------- generic_data_types ----------------");
    // ----------- WITHOUT GENERICS --------------
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // ------------ WITH GENERICS -----------------
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    println!("The largest is: {}", largest(&number_list));

    #[allow(dead_code)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    // let mix = Point { x: 1, y: 10.0 };       // ERROR!
    #[allow(dead_code)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let _both_integer = Point2 { x: 5, y: 10 };
    let _both_float = Point2 { x: 1.0, y: 4.0 };
    let _integer_and_float = Point2 { x: 5, y: 4.0 };

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        // only works for f32 values of Point
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    println!("p.x = {}", _integer.x());

    println!("Distance: {}", _float.distance_from_origin());
    // println!("{}", _integer.distance_from_origin()); // Gives an error

    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn traits() {
    println!("---------- traits ------------");

    pub trait Summary {
        fn summarize(&self) -> String;
        fn summarize_author(&self) -> String;
        // BELOW: Default implementation
        fn extra(&self) -> String {
            format!("[Read more from {}...]", self.summarize_author()) // default implementations
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!(
                "{}, by {} ({}) {}",
                self.headline,
                self.author,
                self.location,
                self.extra()
            )
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {} {}", self.username, self.content, self.extra())
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("The Bhagavad Gita"),
        location: String::from("India"),
        author: String::from("Ved Vyas"),
        content: String::from("Lol"),
    };

    println!("1 new article: {}", article.summarize());

    pub fn _notify(item: &impl Summary) {
        println!("Breaking news!: {}", item.summarize());
    }

    pub fn _notify2<T: Summary>(item: T) {
        println!("Breaking news!: {}", item.summarize());
    }

    pub fn _notify3<T: Summary + Display>(item: T) {
        // mutliple traits
        println!("{}", item.extra());
    }

    fn _some_function<T, U>(_t: T, _u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        1
    }

    // returning types that implement traits
    // NOTE: you can't return two different types of struct though
    // even if they implemented the same traits
    fn _returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn _new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display> ToString for Pair<T> {
        fn to_string(&self) -> String {
            format!("({}, {})", self.x, self.y)
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn _cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let _p1 = Pair { x: 1, y: 2 };
    let _p2 = Pair { x: 3, y: 2 };

    // create all the required methods for pair without operator overloading
}

fn lifetimes() {
    println!("---------- lifetimes ------------");
    let mut _r: u32 = 5;
    {
        let _x = 5;
        // _r = &x; // doesn't work
    }
    println!("r: {}", _r);

    // GENERIC LIFETIMES
    fn gen_lifetimes() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);

        let string1 = String::from("long string is long");

        {
            // string2 has different lifetime than string1
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }

        {
            fn _first_word<'a>(s: &'a str) -> &'a str {
                let bytes = s.as_bytes();
                for (i, &item) in bytes.iter().enumerate() {
                    if item == b' ' {
                        return &s[0..i];
                    }
                }
                &s[..]
            }

            struct ImportantExcerpt<'a> {
                part: &'a str,
            }

            impl<'a> ImportantExcerpt<'a> {
                fn level(&self) -> i32 {
                    3
                }

                fn _announce_and_return_part(&self, announcement: &str) -> &str {
                    println!("Attention please: {}", announcement);
                    self.part
                }
            }

            let novel = String::from("Call me Ishmael. Some years ago...");
            let first_sentence = novel.split('.').next().expect("Couldn't find a '.'");
            let i = ImportantExcerpt {
                part: first_sentence,
            };

            println!("{}, level: {}", i.part, i.level());
        }
    }

    fn longest_with_an_announcement<'a>(x: &'a str, y: &'a str) -> &'a str {
        println!("Announcement! {}", y);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    println!("{}", longest_with_an_announcement("Hello", "Yo"));

    gen_lifetimes();
}
