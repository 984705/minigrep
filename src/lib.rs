use std::collections::HashMap;
use std::{error::Error, fs};
use std::path::PathBuf;
use clap::Parser;
use regex::Regex;
use itertools::Itertools;

pub struct Config {
  pub query: String,
  pub files: Vec<String>,
  pub ignore_case: bool,
  pub reg: bool,
}

#[derive(Parser)]
#[command(version = "0.1.0", author = "984705", about = "a minigrep for study")]
pub struct Cli {
  query: String,
  files: Vec<String>,

  /// ignore case or not
  #[arg(short, long)]
  ignore_case: bool,

  /// partern is a regex
  #[arg(short, long)]
  regex: bool,
}

impl Config {
  pub fn build() -> Result<Config, &'static str> {
    let cli = Cli::parse();
    
    let query = cli.query.clone();
    let files = cli.files.clone();
    let ignore_case = cli.ignore_case.clone();
    let reg = cli.regex.clone();
    
    Ok(Config{query, 
              files, 
              ignore_case, 
              reg})
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  for file in config.files.iter() {
    let contents = fs::read_to_string(file)?;
    
    let results = if config.reg {
      search_regex(&config.query, &contents)
    } else if config.ignore_case {
      search_insentitve(&config.query, &contents)
    } else {
      search(&config.query, &contents)
    };

    println!("{:?}", fs::canonicalize(PathBuf::from(file)).unwrap());

    for (line_no, line) in results.iter().sorted_by_key(|x| x.0) {
      println!("Line {line_no} : {line}");
    }

    print!("\n");
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

pub fn search_regex<'a>(query: &str, contents: &'a str) -> HashMap<i32, &'a str> {
  let mut results = HashMap::new();
  let mut line_no = 0;
  let re = Regex::new(query).unwrap();

  for line in contents.lines() {
    line_no += 1;
    if re.captures(line).is_some() {
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
  fn case_without_parameter() {
    let query = "";
    let contents = "";

    assert_eq!(hashmap!(), search(query, contents));
  }

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

  #[test]
  fn case_regex() {
    let query = r"[A-Z][a-z]{1,3}";
    let contents = "\
Rust:
safe, fast, productive.
Trust me.";

    assert_eq!( hashmap![1 => "Rust:", 3 => "Trust me."], search_regex(query, contents));
  }

}