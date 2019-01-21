mod crypto;

#[cfg(test)]
mod tests {
  use crate::crypto::convert::hex_to_vecu8;
  use crate::crypto::{hex_to_base64, xor, xor_hex};

  #[test]
  fn test_hex_to_vecu8() {
    assert_eq!(hex_to_vecu8("1b"), [27]);
    assert_eq!(
      hex_to_vecu8("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"),
      vec![
        27, 55, 55, 51, 49, 54, 63, 120, 21, 27, 127, 43, 120, 52, 49, 51, 61, 120, 57, 120, 40,
        55, 45, 54, 60, 120, 55, 62, 120, 58, 57, 59, 55, 54
      ]
    );
  }

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
