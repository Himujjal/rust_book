use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(&config.filename)?;

  let search_results = match config.case_sensitive {
    true => search(&config.query, &contents),
    false => search_case_insensitive(&config.query, &contents),
  };

  if search_results.len() == 0 {
    eprintln!(
      "Couldn't match {} from the file {}",
      &config.query, &config.filename
    );
  }
  for line in search_results {
    println!("{}", line);
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}
// this means that the data returned by the search function
// will live as long as the data in the contents is present
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // --------- alternative -----------------

  // let mut results = Vec::new();
  // for line in contents.lines() {
  //   if line.contains(query) {
  //     results.push(line);
  //   }
  // }
  // results
  // ---------============-----------------

  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // --------- alternative -------------
  // let mut results = Vec::new();
  // for line in contents.lines() {
  //   if line.to_lowercase().contains(&query.to_lowercase()) {
  //     results.push(line);
  //   }
  // }
  // -----------------------------------
  contents
    .lines()
    .filter(|line| line.contains(&query.to_lowercase()))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
pick three
    ";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}