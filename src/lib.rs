use std::error::Error;
use std::{ fs, env };

pub struct Config {
  pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 2 {
        return Err("not enough arguments. Usage: myxxd <file_path>");
      }

      let file_path = args[1].clone();

      Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  Ok(())
}