fn main() {
  // Stack string
  // str is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory.
  let message: &str = "Hello World!";

  // Heap string
  // String is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.
  let message_heap: String = String::from("Hello World 2!");

  let message_new = message_heap;

  // Can not use message_heap because is was moved to message_new
  // println!("{}", message_heap);
  println!("{}", message_new);

  // You can place on stack only values with static size
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
