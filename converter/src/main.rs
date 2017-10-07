use std::io;

fn main() {
  loop {
    println!("Enter a temperature in F: ");
    let mut temp: String = String::new();
    io::stdin()
      .read_line(&mut temp)
      .expect("Failed to read line.");

    let temp: i32 = match temp.trim().parse() {
      Ok(val) => val,
      Err(_) => continue,
    };

    let temp = f_to_c(temp);
    println!("In Celcius, the temperature is: {}", temp);
  }
}

fn f_to_c(temperature: i32) -> i32 {
  temperature - 32
}
