//! # Publish Crates module
//!
//! `publish_crates` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number
/// # Examples
/// ```
/// let arg = 5;
/// let answer = t_cargo_crates::publish_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}

pub fn publish_crate() {
  println!("---------publish_crate-----------");
  add_one(2);
}
