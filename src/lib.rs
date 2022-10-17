use std::{fs::File, error::Error, io::Read};

const BUFFER_LEN: usize = 16;

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
  let mut buffer = [0u8; BUFFER_LEN];
  let mut file = File::open(config.file_path)?;

  let mut offset: u32 = 0;
  loop {
    let read_count = file.read(&mut buffer)?;
    let bytes = &buffer[..read_count];
    
    print!("{:08x}:", offset);
    offset += read_count as u32;

    print_data_as_hex(bytes);
    print!("  ");
    print_data_as_chars(bytes);
    println!();

    if read_count != BUFFER_LEN {
      break;
    }
  }

  Ok(())
}

fn print_data_as_hex(bytes: &[u8]) {
  for (i, byte) in bytes.iter().enumerate() {
    if i < BUFFER_LEN - 1 && i % 2 == 0 {
      print!(" ");
    }

    if i < bytes.len() {
      print!("{:02x}", byte);
    } else {
      print!("  ");
    }
  }
}

fn print_data_as_chars(bytes: &[u8]) {
  for byte in bytes.iter() {
    let mut current = byte.clone();
    
    if current < 32 || current > 126 {
      current = '.' as u8;
    }

    print!("{}", current as char)
  }
}