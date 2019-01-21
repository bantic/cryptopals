use super::convert::hex_to_vecu8;
use std::collections::HashMap;

pub fn xor(hexed_input: &str, c: char) -> String {
  let unhexed_input: Vec<u8> = hex_to_vecu8(hexed_input);
  let c = c as u8;
  let mut result = String::new();
  for i in unhexed_input {
    result.push((i ^ c) as char);
  }
  result
}

pub fn decode_fixed_xor(hexed_input: &str) -> String {
  let mut best_score: f32 = -1.0;
  let mut best_output: String = String::new();

  for i in 0_u8..255_u8 {
    let output = xor(hexed_input, i as char);
    let score = freq_score(&output);
    if (score > best_score) {
      println!("{} -> {} -> {}", i, score, output);
      best_score = score;
      best_output = output.clone();
    }
  }
  best_output
}

fn get_char_freq(c: char) -> f32 {
  match c.to_ascii_lowercase() {
    'a' => 8.167 / 100.0,
    'b' => 1.492 / 100.0,
    'c' => 2.782 / 100.0,
    'd' => 4.253 / 100.0,
    'e' => 12.70 / 100.0,
    'f' => 2.228 / 100.0,
    'g' => 2.015 / 100.0,
    'h' => 6.094 / 100.0,
    'i' => 6.966 / 100.0,
    'j' => 0.153 / 100.0,
    'k' => 0.772 / 100.0,
    'l' => 4.025 / 100.0,
    'm' => 2.406 / 100.0,
    'n' => 6.749 / 100.0,
    'o' => 7.507 / 100.0,
    'p' => 1.929 / 100.0,
    'q' => 0.095 / 100.0,
    'r' => 5.987 / 100.0,
    's' => 6.327 / 100.0,
    't' => 9.056 / 100.0,
    'u' => 2.758 / 100.0,
    'v' => 0.978 / 100.0,
    'w' => 2.360 / 100.0,
    'x' => 0.150 / 100.0,
    'y' => 1.974 / 100.0,
    'z' => 0.074 / 100.0,
    _ => 0.0,
  }
}

fn freq_score(str: &str) -> f32 {
  let mut score: f32 = 0.0;
  let mut letter_counts: HashMap<char, i32> = HashMap::new();
  for c in str.chars() {
    let c = c.to_ascii_lowercase();
    if !c.is_ascii() {
      score -= 1.0;
      continue;
    }

    if !letter_counts.contains_key(&c) {
      letter_counts.insert(c, 1);
    } else {
      let v = letter_counts.get(&c).unwrap();
      letter_counts.insert(c, v + 1);
    }
  }

  for c in "abcdefghijklmnopqrstuvwxyz".chars() {
    let count = match letter_counts.get(&c) {
      Some(v) => *v,
      _ => 0,
    };
    let freq: f32 = (count as f32) / (str.len() as f32);
    let expected_freq = get_char_freq(c);
    score += 2.0 - (freq - expected_freq).abs();
  }

  let expected_space_freq: f32 = (str.len() as f32) / 4.0;
  let space_count = match letter_counts.get(&' ') {
    Some(v) => *v,
    _ => 0,
  };
  let space_freq = (space_count as f32) / (str.len() as f32);
  score += 2.0 - (space_freq - expected_space_freq).abs();

  for c in str.chars() {
    if !c.is_ascii() {
      score -= 1.0;
    }
  }

  score
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_decode() {
    assert_eq!(
      decode_fixed_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"),
      "Cooking MC's like a pound of bacon"
    );
  }
}
