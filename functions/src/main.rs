extern crate easy_strings;

use easy_strings::{ez, EZString};

fn main() {
  let eto_gg = ez("eto guegue");

  let eto_gg = make_uppercase(eto_gg);

  println!("str is now: {}", eto_gg.to_string());
}

fn make_uppercase(the_str: EZString) -> EZString {
  the_str.to_uppercase()
}
