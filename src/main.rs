use std::{
  fs::File,
  io::{self, Read},
};

mod base64;

fn main() -> Result<(), io::Error> {
  let mut buf = Vec::new();
  let input = std::env::args().nth(1).expect("please input file path");
  let size = parse(&input)?.read_to_end(&mut buf)?;
  println!("{}", base64::encode(&buf, size));
  Ok(())
}

fn parse(path: &str) -> io::Result<Box<dyn Read>> {
  let file = File::open(path)?;
  Ok(Box::new(file))
}
