use q_testing;

pub fn add_two(a: i32) -> i32 {
  a + 2
}

#[cfg(test)]
mod tests {
  use super::add_two;

  #[test]
  fn it_adds_two_2s() {
    assert_eq!(4, add_two(3));
  }
}
