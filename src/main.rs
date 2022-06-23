fn main() {
  let message: String = String::from("Hello World!");
  let new_message: &String = &message;
  // new_message is not the owner of the data
  // new_message is borrowing a reference to message

  println!("{}", message);
  println!("{}", new_message);
}
// message and new_message going out of scope
// new_message is not dropped because it does not have the ownership of what it refers to
// message is dropped
