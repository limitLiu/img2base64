pub trait CharAt {
  fn char_at(&self, index: usize) -> char;
}

impl CharAt for str {
  fn char_at(&self, index: usize) -> char {
    self[index..].chars().next().unwrap()
  }
}

const TABLE: &str = "\
    ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(bytes: &[u8], size: usize) -> String {
  let mut buffer = Vec::<char>::new();
  buffer.resize((size + 2) / 3 * 4, '=');
  let mut i = 0;
  let mut j = 0;
  let n = size / 3 * 3;
  while i < n {
    let x: u32 = ((bytes[i] as u32) << 16)
      | ((bytes[i + 1] as u32) << 8)
      | (bytes[i + 2] as u32);
    buffer[j] = TABLE.char_at(((x >> 18) & 63) as usize);
    buffer[j + 1] = TABLE.char_at(((x >> 12) & 63) as usize);
    buffer[j + 2] = TABLE.char_at(((x >> 6) & 63) as usize);
    buffer[j + 3] = TABLE.char_at((x & 63) as usize);
    i += 3;
    j += 4;
  }
  if i + 1 == size {
    let x = (bytes[i] as u32) << 16;
    buffer[j] = TABLE.char_at(((x >> 18) & 63) as usize);
    buffer[j + 1] = TABLE.char_at(((x >> 12) & 63) as usize);
  } else if i + 2 == size {
    let x = (bytes[i] as u32) << 16 | (bytes[i + 1] as u32) << 8;
    buffer[j] = TABLE.char_at(((x >> 18) & 63) as usize);
    buffer[j + 1] = TABLE.char_at(((x >> 12) & 63) as usize);
    buffer[j + 2] = TABLE.char_at(((x >> 6) & 63) as usize);
  }
  buffer.iter().collect()
}

#[test]
fn test_encode() {
  assert_eq!(encode("".as_bytes(), 0), "");
  assert_eq!(encode("f".as_bytes(), 1), "Zg==");
  assert_eq!(encode("fo".as_bytes(), 2), "Zm8=");
  assert_eq!(encode("foo".as_bytes(), 3), "Zm9v");
  assert_eq!(encode("foob".as_bytes(), 4), "Zm9vYg==");
}
