use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // _mean_median_mode();
    // _pig_latin();
    _mini_database();
}

fn _mean_median_mode() {
    println!("---------_mean_median_mode--------------");
    fn get_mean(vec: &mut Vec<i32>) -> f32 {
        let mut sum: i32 = 0;
        for i in vec.iter() {
            sum += i;
        }
        sum as f32 / (vec.len() as f32)
    }

    fn get_mode(vec: &mut Vec<i32>) -> i32 {
        vec.sort();
        match vec.get(vec.len() / 2) {
            Some(num) => *num,
            None => 0,
        }
    }

    // TODO: Implement the better algorithm
    fn get_median(vec: &Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for num in vec.iter() {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }
        let mut max_val = 0;
        let mut val = 0;
        for (k, v) in map.iter() {
            if *v > max_val {
                max_val = *v;
                val = **k;
            }
        }
        val
    }

    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 7, 6, 6];

    let mean = get_mean(&mut numbers);
    let mode = get_mode(&mut numbers);
    let median = get_median(&numbers);

    println!("list of numbers: {:?}", numbers);
    println!("mean: {}, median: {}, mode: {}", mean, mode, median);
}

fn _pig_latin() {
    println!("---------_pig_latin--------------");
    fn gen_pig_latin(st: &mut String) -> String {
        let nullified_word;

        let first_letter = st.chars().nth(1).unwrap();
        let first_letter = match first_letter {
            'a' | 'b' | 'c' | 'd' | 'e' => {
                nullified_word = st.to_string();
                'h'
            }
            _ => {
                nullified_word = st[1..st.len()].to_string();
                first_letter
            }
        };

        let st2 = nullified_word.to_string() + "-" + &first_letter.to_string() + "ay";
        st2
    }

    let word1 = gen_pig_latin(&mut String::from("getting there"));
    let word2 = gen_pig_latin(&mut String::from("apple"));

    println!("{}", word1);
    println!("{}", word2);
}

fn _mini_database() {
    println!("---------MINI DATABASE--------------");

    fn print_instructions() {
        println!("INSTRUCTIONS:");
        println!("To add a name to a department: \"Add <name> to <department>\"");
        println!(
            "To update a department of employee: \"Update <name> from <prev_department> to <new_department>\" "
        );
        println!("To update the name in department: \"Update <old_name> to <new_name> in <department>\" ");
        println!("To delete a name in department: \"Delete <name> in <department>\" ");
        println!("To get all the names: \"Get all names\" ");
        println!("To get all names in a department: \"Get all names in <department>\" ");
        println!("To quit: \"quit\" ");
        println!("To show instructions again: \"instructions\"");
       
    }

    print_instructions();

    let mut database: HashMap<String, Vec<String>> = HashMap::new();

    enum UpdateTypes {
        Move,
        Update,
    }

    enum GetTypes {
        GetAll,
        GetDepartment,
    }

    enum InstructionType {
        Add,
        Update(UpdateTypes),
        Delete,
        Get(GetTypes),
        Quit,
        Instructions,
        Invalid,
    }

    fn get_instruction_type(vector_instruction_string: &Vec<&str>) -> InstructionType {
        match vector_instruction_string.get(0) {
            Some(i) => match *i {
                "add" => match vector_instruction_string.len() {
                    4 => match vector_instruction_string.get(2) {
                        Some(third_str) => match *third_str {
                            "to" => InstructionType::Add,
                            _ => InstructionType::Invalid,
                        },
                        _ => InstructionType::Invalid,
                    },
                    _ => InstructionType::Invalid,
                },
                "update" => match vector_instruction_string.len() {
                    6 => match vector_instruction_string.get(2) {
                        Some(third_str) => match *third_str {
                            "to" => match vector_instruction_string.get(4) {
                                Some(fifth_str) => match *fifth_str {
                                    "in" => InstructionType::Update(UpdateTypes::Update),
                                    _ => InstructionType::Invalid,
                                },
                                None => InstructionType::Invalid,
                            },
                            "from" => match vector_instruction_string.get(4) {
                                Some(fifth_str) => match *fifth_str {
                                    "to" => InstructionType::Update(UpdateTypes::Move),
                                    _ => InstructionType::Invalid,
                                },
                                None => InstructionType::Invalid,
                            },
                            _ => InstructionType::Invalid,
                        },
                        _ => InstructionType::Invalid,
                    },
                    _ => InstructionType::Invalid,
                },
                "delete" => match vector_instruction_string.len() {
                    4 => match vector_instruction_string.get(2) {
                        Some(third_str) => match *third_str {
                            "in" => InstructionType::Delete,
                            _ => InstructionType::Invalid,
                        },
                        _ => InstructionType::Invalid,
                    },
                    _ => InstructionType::Invalid,
                },
                "get" => match vector_instruction_string.len() {
                    3 => match &format!(
                        "{} {}",
                        vector_instruction_string[1], vector_instruction_string[2]
                    )[..]
                    {
                        "all names" => InstructionType::Get(GetTypes::GetAll),
                        _ => InstructionType::Invalid,
                    },
                    5 => match &format!(
                        "{} {} {}",
                        vector_instruction_string[1],
                        vector_instruction_string[2],
                        vector_instruction_string[3]
                    )[..]
                    {
                        "all names in" => InstructionType::Get(GetTypes::GetDepartment),
                        _ => InstructionType::Invalid,
                    },
                    _ => InstructionType::Invalid,
                },
                "quit" => InstructionType::Quit,
                "instructions" => InstructionType::Instructions,
                _ => InstructionType::Invalid,
            },
            None => InstructionType::Invalid,
        }
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_lowercase();
        let vector_instruction_string: Vec<&str> = input.split_whitespace().collect();

        let instruction_type = get_instruction_type(&vector_instruction_string);

        match instruction_type {
            InstructionType::Add => {
               let name = vector_instruction_string[1].to_string();
               let department_name = vector_instruction_string[3].to_string();

               database.entry(department_name.clone()).and_modify(|v| {
                    if v.iter().any(|ele| *ele == name) {
                        println!("FAILURE! {} already present in {}", name, department_name);
                    } else {
                        println!("Successfully added: {} in {} department!", name.clone(), department_name);
                        v.push(name.clone());
                    }
                }).or_insert(vec![name.clone()]);
            }
            InstructionType::Delete => {
                let name = vector_instruction_string[1].to_string();
                let department = vector_instruction_string[3].to_string();

                database.entry(department.clone()).and_modify(|e| {
                    let mut is_present = false;
                    e.retain(|x| {
                        if !x.eq(&name) {
                            true
                        } else {
                            is_present = true;
                            false
                        }
                    });
                    if is_present {
                        println!("Successfully removed: {} in {} department!", name, department);
                    } else {
                        println!("FAILURE: {} not present in {} department", name, department);
                    }
                });
            }
            InstructionType::Get(GetTypes::GetAll) => {
                for (key, value) in &database {
                    println!("{}:", key);
                    for val in value.iter() {
                        println!("\t{}:", val);
                    }
                }
            }
            InstructionType::Get(GetTypes::GetDepartment) => {
                let department = vector_instruction_string[4];
                match database.get(department) {
                    Some(vec) => {
                        for val in vec.iter() {
                            println!("\t{}:", val);
                        }
                    },
                    None => { println!("Department Not found"); }
                }
            }
            InstructionType::Update(UpdateTypes::Move) => {
                let name = vector_instruction_string[1].to_string();
                let old_department = vector_instruction_string[3].to_string();
                let new_department = vector_instruction_string[5].to_string();

                let mut is_present = false;
                database.entry(old_department.clone()).and_modify(
                    |e| {
                        e.retain(|x| {
                            match !x.eq(&name) {
                                true => true,
                                false => {
                                    is_present = true;     
                                    false
                                }
                            }
                        });
                    });
                    if is_present {
                        database.entry(new_department.clone())
                        .and_modify(|e| e.push(name.clone()))
                        .or_insert(vec![name.clone()]);
                        println!("Successfully moved: {} from {} department to {} department!", name, old_department, new_department);
                    } else {
                        println!("Couldn't move {} from {} to {} as {} is not present in {}", name, old_department, new_department, name, old_department);
                    }
            }
            InstructionType::Update(UpdateTypes::Update) => {
                let old_name = vector_instruction_string[1].to_string();
                let new_name = vector_instruction_string[3].to_string();
                let department = vector_instruction_string[5].to_string();

                match database.get_mut(&department) {
                    Some(_v) => {
                        let mut is_present = false;
                        for i in 0.._v.len() {
                            if _v[i] == old_name {
                                is_present = true;
                                _v[i] = new_name.clone();
                            }
                        }
                        if !is_present {
                            println!("No such name present!");
                        } else {
                            println!("Successfully updated {} from {} in {} department!", old_name, new_name, department);
                        }
                    },
                    None => {
                        println!("Error! Department not present!");
                    }
                };
            }
            InstructionType::Quit => {
                break;
            }
            InstructionType::Instructions => {
                print_instructions();
            }
            InstructionType::Invalid => {
                println!("Invalid command! (tip: command \"Instructions\" is for printing the instructions again) ")
            }
        }
    }
}
