use util::Type;

pub fn filter_by(selected_type: Type, arr: Vec<T>) {
  match selected_type {
    Name => println!("{}", Name),
    Artist => println!("{}", Artist),
    Genre => println!("{}", Genre),
  }
}
