#[macro_export]
macro_rules! vec {
    ($($x: expr), *) => {
       {
           let mut temp_vec = Vec::new();
           $(
               temp_vec.push($x);
           )*
           temp_vec
       }
    };
}

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(Debug)]
struct Pancakes;

// real implementation
// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, macro! my name is Pancakes!");
//     }
// }

pub fn main() {
    println!("---------E_MACROS-----------");
    // Pancakes::hello_macro();
}
