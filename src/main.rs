use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // get the command line parameter
    let args: Vec<String> = env::args().collect();

    // parse the cmd parameter
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments: {err}");
        process::exit(1);
    });

    // run grep's script
    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
