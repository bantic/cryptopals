// hex,hex -> hex
use super::convert;

pub fn xor_hex(a: &str, b: &str) -> String {
  let a = convert::hex_to_bits(a);
  let b = convert::hex_to_bits(b);

  let mut result: Vec<bool> = vec![];
  for i in 0..a.len() {
    result.push(&a[i] ^ &b[i]);
  }

  convert::bits_to_hex(result)
}
