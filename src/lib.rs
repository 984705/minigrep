use std::collections::HashMap;
use std::{error::Error, fs};
use std::env;

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return  Err("not enough arguments");
    }

    let query = args[1].clone();
    let file_path = args[2].clone();
    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Ok(Config{query, file_path, ignore_case})
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;
  
  // using hashmap instead of vector to record the query word in which line
  let results = if config.ignore_case {
    search_insentitve(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for (line_no, line) in results.iter() {
    println!("in line {line_no} : {line}");
  }
  
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> HashMap<i32, &'a str> { 
  let mut results = HashMap::new();
  let mut line_no = 0;

  for line in contents.lines() {
    line_no += 1;
    if line.contains(query) {
      results.insert(line_no, line);
    }
  }

  results
}

pub fn search_insentitve<'a>(query: &str, contents: &'a str) -> HashMap<i32, &'a str> {
  let query = query.to_lowercase();
  let mut results = HashMap::new();
  let mut line_no = 0;

  for line in contents.lines() {
    line_no += 1;
    if line.to_lowercase().contains(&query) {
      results.insert(line_no, line);
    }
  }

  results
}

// hashmap init marco, like vec!
macro_rules! hashmap {
  ($( $key: expr => $val: expr ),*) => {{
       let mut map = ::std::collections::HashMap::new();
       $( map.insert($key, $val); )*
       map
  }}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_basic() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!( hashmap![2 => "safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

    assert_eq!( hashmap![2 => "safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Trust me.";

    assert_eq!( hashmap![1 => "Rust:", 3 => "Trust me."], search_insentitve(query, contents));
  }

}