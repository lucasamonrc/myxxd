use std::{ env, process };

use myxxd::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Parsing error. {err}");
        process::exit(1);
    });

    if let Err(e) = myxxd::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
