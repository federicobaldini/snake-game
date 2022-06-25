fn main() {
  let mut message: String = String::from("Hello");
  let new_message: &mut String = &mut message;
  // new_message is not the owner of the data
  // new_message is borrowing a reference to message

  new_message.push_str(" World!");

  println!("{}", new_message);
  println!("{}", message);
}
// message and new_message going out of scope
// new_message is not dropped because it does not have the ownership of what it refers to
// message is dropped
