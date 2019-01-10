mod crypto;

#[cfg(test)]
mod tests {
  use crate::crypto::{hex_to_base64, xor_hex};

  #[test]
  fn test_xor_hex() {
    assert_eq!(
      xor_hex(
        "1c0111001f010100061a024b53535009181c",
        "686974207468652062756c6c277320657965"
      ),
      "746865206b696420646f6e277420706c6179"
    );
  }

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
