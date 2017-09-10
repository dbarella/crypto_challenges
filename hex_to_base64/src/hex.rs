// Hex Encoding.
//
// We assume that hex input strings must consist of characters in the set
// {0123456789abcdef}; anything else will fail to parse.

pub struct Hex<'a> {
  bytes: &'a [u8]
}

impl<'a> Hex<'a> {
  fn new(input: &str) -> Hex {
    Hex{ bytes: input.as_bytes() }
  }
}
