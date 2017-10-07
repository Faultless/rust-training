fn main() {
  const MAX_POINTS: u32 = 100_000;
  println!("max points are: {}", MAX_POINTS);

  let x: char = '5';
  println!("x is: {}", x);

  let x: i32 = 6;
  println!("x is: {}", x);

  let spaces = "    ";
  let spaces = spaces.len();
  println!("spaces is: {}", spaces);

  let heart_eyed_cat = '😻';
  println!("meow: {}", heart_eyed_cat);

  let tup = (500, 1.3, '5');
  let (x, y, z): (i32, f64, char) = tup;
  println!("y is: {}", y);
}
