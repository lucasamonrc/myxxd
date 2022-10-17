use std::{fs, error::Error};

// const MAX_BYTES_HEX: usize = 16;

pub struct Config {
  pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 2 {
        return Err("Usage: myxxd <file_path>");
      }

      let file_path = args[1].clone();

      Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let bytes = fs::read(config.file_path)?;

  for byte in bytes.iter() {
    print!("{:x} ", byte);
  }

  Ok(())
}