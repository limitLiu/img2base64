use std::{
  fs::File,
  io::{self, Read},
};

use format::ImageFormat;

mod base64;
mod format;

fn main() -> Result<(), io::Error> {
  let mut buf = Vec::new();
  let input = std::env::args().nth(1).expect("please input file path");
  let size = parse(&input)?.read_to_end(&mut buf)?;

  let fmt = ImageFormat::from_ext_str(input).map(|fmt| match fmt {
    ImageFormat::Svg => "data:image/svg+xml;base64,",
    ImageFormat::Jpeg => "data:image/jpg;base64,",
    ImageFormat::Png => "data:image/png;base64,",
  });

  match fmt {
    Some(f) => print!("{}{}", f, base64::encode(&buf, size)),
    None => print!("{}", base64::encode(&buf, size)),
  }

  Ok(())
}

fn parse(path: &str) -> io::Result<Box<dyn Read>> {
  let file = File::open(path)?;
  Ok(Box::new(file))
}
