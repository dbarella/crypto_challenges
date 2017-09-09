// Libraries for encoding base64 strings.

mod base64 {
    use super::hex::Hex;

    pub type Base64 = Vec<u8>;

    fn from(hex: Hex) -> Base64 {
        vec!()
    }
}

mod hex {
   pub type Hex = Vec<u8>;

   fn from(input: &str) -> Hex {
      vec!()
   }
}

#[cfg(test)]
mod tests;
