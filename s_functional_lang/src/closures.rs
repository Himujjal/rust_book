use std::thread;
use std::time::Duration;

struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => {
        if v != arg {
          self.value = Some((self.calculation)(arg))
        }
        self.value.unwrap()
      }
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

pub fn closures() {
  println!("------- Closures ---------");

  fn generate_workout(intensity: u32, random_number: u32) {
    // type inference happens the first time the closure is called
    let mut expensive_result = Cacher::new(|num| {
      println!("Calculating slowly...");
      // thread::sleep(Duration::from_secs(1));
      num
    });

    if intensity < 25 {
      println!("Today, do {} pushups!", expensive_result.value(intensity));
      println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
      if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
      } else {
        println!(
          "Today, run for {} minutes!",
          expensive_result.value(intensity)
        );
      }
    }
  }

  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;
  generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[test]
fn call_with_different_values() {
  let mut c = Cacher::new(|a| a);
  let _v1 = c.value(1);
  let v2 = c.value(2);

  assert_eq!(v2, 2);
}

// cargo test environment_1
#[test]
fn environment_1() {
  let x = vec![1, 2, 3];

  // encloses surrounding
  // move will move the x variable to the closure scope
  let equal_to_x = move |z| z == x;

  // println!("can't use x here: {:?}", x); // ERROR

  let y = vec![1, 2, 3];

  assert!(equal_to_x(y));
}
