fn main() {
  let s1 = String::from("a string");
  let length = calculate_length(&s1);

  println!("The string \"{}\" is of length {}", s1, length);

  let sub = first_word(&s1);
  println!("First word of \"{}\" is \"{}\"", s1, sub);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }
  &s[..]
}
