mod hex_to_base64;
mod xor_hex;
pub use self::hex_to_base64::hex_to_base64;
pub use self::xor_hex::xor_hex;

mod convert {
  pub fn hex_to_decimal(hex: &str) -> i32 {
    i32::from_str_radix(hex, 16).expect("Failed to parse")
  }

  fn decimal_to_bits(d: i32) -> Vec<bool> {
    let bits = format!("{:08b}", d);
    bits.chars().map(|c| c == '1').collect()
  }

  pub fn hex_to_bits(hex: &str) -> Vec<bool> {
    let mut result = vec![];
    for idx in (0..hex.len()).step_by(2) {
      let decimal = hex_to_decimal(&hex[idx..idx + 2]);
      let bits = decimal_to_bits(decimal);
      result.extend_from_slice(&bits)
    }

    result
  }

  pub fn bits_to_hex(bits: Vec<bool>) -> String {
    let mut result = String::from("");
    for i in (0..bits.len()).step_by(8) {
      let d = bits_to_decimal(bits[i..i + 8].to_vec());
      result.push_str(&format!("{:x}", d));
    }
    result
  }

  pub fn bits_to_decimal(bits: Vec<bool>) -> i32 {
    let base: i32 = 2;
    let max_exp: u32 = bits.len() as u32 - 1;
    let mut val: i32 = 0;
    for i in 0..bits.len() {
      let pow = max_exp - (i as u32);
      val += if bits[i] { base.pow(pow) } else { 0 }
    }
    val
  }

}

#[cfg(test)]
mod tests {
  use super::convert::{bits_to_decimal, hex_to_decimal};

  #[test]
  fn test_hex_to_decimal() {
    assert_eq!(hex_to_decimal("4D"), 77);
    assert_eq!(hex_to_decimal("01"), 1);
  }

  #[test]
  fn test_bits_to_decimal_8() {
    assert_eq!(
      bits_to_decimal(vec![false, false, false, false, false, false, false, false]),
      0
    );
    assert_eq!(
      bits_to_decimal(vec![false, false, false, false, false, false, false, true]),
      1
    );
    assert_eq!(
      bits_to_decimal(vec![false, false, false, false, false, false, true, false]),
      2
    );
    assert_eq!(
      bits_to_decimal(vec![false, false, false, false, false, false, true, true]),
      3
    );
    assert_eq!(
      bits_to_decimal(vec![true, false, false, false, false, false, false, false]),
      128
    );
  }

  #[test]
  fn test_bits_to_decimal_6() {
    assert_eq!(
      bits_to_decimal(vec![false, false, false, false, false, false]),
      0
    );
    assert_eq!(
      bits_to_decimal(vec![false, false, false, false, false, true]),
      1
    );
    assert_eq!(
      bits_to_decimal(vec![false, false, false, false, true, false]),
      2
    );
    assert_eq!(
      bits_to_decimal(vec![false, false, false, false, true, true]),
      3
    );
    assert_eq!(
      bits_to_decimal(vec![true, false, false, false, false, false]),
      32
    );
    assert_eq!(
      bits_to_decimal(vec![true, true, true, true, true, true]),
      63
    );
  }
}
