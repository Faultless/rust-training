fn main() {
  let mut new_user = User {
    username: String::from("serge"),
    email: String::from("kamel.serge@hotmail.fr"),
    sign_in_count: 0,
    active: false,
  };

  new_user.username = String::from("paratoner");

  print!(
    "{}\n{}\n{}\n{}",
    new_user.username,
    new_user.email,
    new_user.sign_in_count,
    new_user.active
  );
}

#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
