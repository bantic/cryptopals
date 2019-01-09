use std::collections::HashMap;

fn main() {
  println!("Hello, world!");
}

fn hex_to_decimal(hex: &str) -> i32 {
  i32::from_str_radix(hex, 16).expect("Failed to parse")
}

fn decimal_to_bits(d: i32) -> Vec<bool> {
  let bits = format!("{:08b}", d);
  println!("d2b {}->{:?}", d, bits);
  bits.chars().map(|c| c == '1').collect()
}

fn hex_to_bits(hex: &str) -> Vec<bool> {
  let mut result = vec![];
  for idx in (0..hex.len()).step_by(2) {
    let decimal = hex_to_decimal(&hex[idx..idx + 2]);
    let bits = decimal_to_bits(decimal);
    println!("{}->{}->{:?}", &hex[idx..idx + 2], decimal, bits);
    result.extend_from_slice(&bits)
  }

  result
}

fn pad_bits(bits: Vec<bool>) -> Vec<bool> {
  let mut result = vec![];
  result.extend_from_slice(&bits);
  while result.len() % 6 != 0 {
    result.push(false);
  }
  result
}

fn bits_to_base64(bits: Vec<bool>) -> char {
  let decimal_to_base64_map: HashMap<i32, char> = [
    (0, 'A'),
    (16, 'Q'),
    (32, 'g'),
    (48, 'w'),
    (1, 'B'),
    (17, 'R'),
    (33, 'h'),
    (49, 'x'),
    (2, 'C'),
    (18, 'S'),
    (34, 'i'),
    (50, 'y'),
    (3, 'D'),
    (19, 'T'),
    (35, 'j'),
    (51, 'z'),
    (4, 'E'),
    (20, 'U'),
    (36, 'k'),
    (52, '0'),
    (5, 'F'),
    (21, 'V'),
    (37, 'l'),
    (53, '1'),
    (6, 'G'),
    (22, 'W'),
    (38, 'm'),
    (54, '2'),
    (7, 'H'),
    (23, 'X'),
    (39, 'n'),
    (55, '3'),
    (8, 'I'),
    (24, 'Y'),
    (40, 'o'),
    (56, '4'),
    (9, 'J'),
    (25, 'Z'),
    (41, 'p'),
    (57, '5'),
    (10, 'K'),
    (26, 'a'),
    (42, 'q'),
    (58, '6'),
    (11, 'L'),
    (27, 'b'),
    (43, 'r'),
    (59, '7'),
    (12, 'M'),
    (28, 'c'),
    (44, 's'),
    (60, '8'),
    (13, 'N'),
    (29, 'd'),
    (45, 't'),
    (61, '9'),
    (14, 'O'),
    (30, 'e'),
    (46, 'u'),
    (62, '+'),
    (15, 'P'),
    (31, 'f'),
    (47, 'v'),
    (63, '/'),
  ]
  .iter()
  .cloned()
  .collect();

  let decimal = bits_to_decimal(bits);
  *decimal_to_base64_map
    .get(&decimal)
    .expect("Failed to find the decimal->base64 equiv")
}

fn bits_to_decimal(bits: Vec<bool>) -> i32 {
  let base: i32 = 2;
  let max_exp: u32 = bits.len() as u32 - 1;
  let mut val: i32 = 0;
  for i in 0..bits.len() {
    let pow = max_exp - (i as u32);
    // println!("i {}, val {}, pow: {}", i, bits[i], pow);
    val += if bits[i] { base.pow(pow) } else { 0 }
  }
  val
}

pub fn hex_to_base64(hex: &str) -> String {
  let bits = pad_bits(hex_to_bits(hex));
  let equal_padding_needed = match bits.len() {
    l if l <= 24 => 24 - l,
    l => 24 - (24 % l),
  } / 6;
  println!(
    "padded bits len {}, padding_needed {}",
    bits.len(),
    equal_padding_needed
  );

  let mut result = String::from("");
  for i in (0..bits.len()).step_by(6) {
    let _bits = bits[i..i + 6].to_vec();
    result.push(bits_to_base64(_bits));
  }
  for _ in 0..equal_padding_needed {
    result.push('=');
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hex_to_decimal() {
    assert_eq!(hex_to_decimal("4D"), 77);
    assert_eq!(hex_to_decimal("01"), 1);
  }

  #[test]
  fn test_hex_to_bits() {
    // 0 1	0	0	1	1	0	1
    assert_eq!(
      hex_to_bits("4d"),
      [false, true, false, false, true, true, false, true]
    );

    // 0	1	1	0	0	0	0	1
    assert_eq!(
      hex_to_bits("61"),
      [false, true, true, false, false, false, false, true]
    );

    assert_eq!(
      hex_to_bits("4d61"),
      [
        false, true, false, false, true, true, false, true, false, true, true, false, false, false,
        false, true
      ]
    );
  }

  #[test]
  fn test_pad_bits() {
    assert_eq!(
      pad_bits(vec![true, true, true, true, true, true]),
      vec![true, true, true, true, true, true]
    );
    assert_eq!(
      pad_bits(vec![true, true, true, true, true]),
      vec![true, true, true, true, true, false]
    );
    assert_eq!(
      pad_bits(vec![true, true, true, true]),
      vec![true, true, true, true, false, false]
    );
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

  #[test]
  fn test_hex264_simple() {
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
