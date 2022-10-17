use std::{fs::File, error::Error, io::Read};

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
  const BUFFER_LEN: usize = 16;
  let mut buffer = [0u8; BUFFER_LEN];
  let mut file = File::open(config.file_path)?;

  
  loop {
    let read_count = file.read(&mut buffer)?;
    let bytes = &buffer[..read_count];

    print_data_as_hex(bytes, read_count);
    print!("  ");
    print_data_as_chars(bytes, read_count);

    if read_count != BUFFER_LEN {
      break;
    }
  }

  Ok(())
}

fn print_data_as_hex(bytes: &[u8], read_count: usize) {
  for byte in bytes.iter() {
    print!("{:x} ", byte)
  }
}

fn print_data_as_chars(bytes: &[u8], read_count: usize) {
  for byte in bytes.iter() {
    print!("{:x} ", byte)
  }
}