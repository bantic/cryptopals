mod crypto;

#[cfg(test)]
mod tests {
  use crate::crypto::hex_to_base64;

  #[test]
  fn test_hex_to_base64_simple() {
    assert_eq!(hex_to_base64("4D616E"), "TWFu");
    assert_eq!(hex_to_base64("4D61"), "TWE=");
    assert_eq!(hex_to_base64("4D"), "TQ==");
  }

  #[test]
  fn test_hex_to_base64() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    assert_eq!(hex_to_base64(input), expected);
  }
}
