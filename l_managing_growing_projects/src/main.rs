use rand::Rng;

#[allow(unused_imports)]
use std::{cmp::Ordering, io};

use std::collections::HashMap;

// no re-imports. provide a new name
use std::fmt::Result as fmtResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let _secret_number = rand::thread_rng().gen_range(1, 101);
}

fn _func(_r: fmtResult) {}

fn _func2(_r: io::Result<()>) {}
