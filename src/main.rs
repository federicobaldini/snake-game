fn main() {
  let message: &str = "Hello World!";
  let result: bool = print_message(message);

  if result {
    println!("{}", "Message print: successful");
  } else {
    println!("{}", "Message print: failed");
  }
}

// This function receive a message and print it
fn print_message(message: &str) -> bool {
  println!("{}", message);
  true
}
