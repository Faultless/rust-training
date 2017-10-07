
#![allow(unused_variables)]
#[derive(Debug)]
enum IpAddr {
  V4(),
  V6(),
}

fn main() {
  let home = IpAddr::V4(127, 0, 0, 1);
  let loopback = IpAddr::V6(String::from("::1"));

  println!("home is: {:#?}\nloopback is: {:#?}", home, loopback);
}
