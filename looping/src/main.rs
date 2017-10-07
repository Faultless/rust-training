fn main() {
  let array = [0, 1, 2, 3];
  let mut index = 5;

  for element in array.iter() {
    println!("the value is: {}", element);
  }

  while index > 0 {
    println!("{}", index);
    index = index - 1;
  }

  println!("LIFT OFF!!");
}
