use std::collections::HashMap;

fn main() {
    // _vectors();
    // _strings();
    _hashmaps();
}

fn _vectors() {
    println!("------ vectors -----------");
    let _v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];

    let mut _v3 = Vec::new();
    _v3.push(5);
    _v3.push(6);
    _v3.push(7);
    _v3.push(8);

    // reading elements of vectors
    let v4 = vec![1, 2, 3, 4, 5];
    let _third: &i32 = &v4[2];
    let fourth = v4[3];
    println!("The third element is {}", _third);
    println!("The fourth element is {}", fourth);

    match v4.get(7) {
        Some(third) => println!("The third element ios {}", third),
        None => println!("There is no third element."),
    }

    let v5 = vec![1, 2, 3, 4, 5];
    let _first = &v5[0]; // mutable borrow occurs here
                         // v.push(6);
                         // println!("The first element is: {}", first);

    let v6 = vec![100, 32, 57];
    for _i in &v6 {
        // println!("{}", i);
    }

    let mut v7 = vec![100, 32, 57];
    for i in &mut v7 {
        *i += 50;
    }
    v7.push(31);

    println!("v7: {:?}", v7);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        match i {
            SpreadsheetCell::Int(cell) => println!("Int: {:?}", cell),
            SpreadsheetCell::Float(fl) => println!("Float: {:?}", fl),
            _ => println!("Hello"),
        }
    }
}

fn _strings() {
    println!("------ strings -----------");
    let data = "initial contents";
    let _s = data.to_string();

    let _s = "initial contents".to_string();

    let _hello = String::from("नमस्ते");

    println!("{}", _hello);

    let mut s = String::from("foo");
    s.push('l');

    let s2 = "bar";
    s.push_str(s2); // takes a slice and thus ownership is not disturbed

    println!("s is {}, s2: {}", s, s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 gets borrowed to s3
    println!("s3: {}, s2: {}, {}", s3, s2, s3);

    let s1 = String::from("tic");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("s4: {}, s1: {}", s4, s1);

    // string slices
    let _hello = String::from("नमस्ते");
    let _slice = &_hello[6..12]; // slices are used
    println!("{}", _slice);

    let _hello = String::from("getting");
    let _slice = &_hello[0..1]; // slices are used
    println!("{}", _slice);

    println!("// --> for loops");
    for c in "नमस्ते".chars() {
        print!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn _hashmaps() {
    println!("------ hashmaps -----------");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("yellow")];

    let initial_scores = vec![10, 50];
    // scores will be the owners of teams' Strings
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);

    let field_name = String::from("favourite color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // println!("{}", field_name);      // ERROR!

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    println!(
        "team_name: {}, score: {}",
        team_name,
        match _score {
            Some(score) => score,
            None => &0,
        }
    );

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // --------- updating a hash map--------
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // insert only if the key Yellow or Blue is not available
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    _frequency_str();
}

fn _frequency_str() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // returns a `&mut V` reference to the value of the key
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
