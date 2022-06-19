fn main() {
  // "message" coming into the scope
  let message = String::from("Hello");
  // "message" is moved into the print_message function
  print_message(message);
  // "message" is no longer valid
}
// "message" is going out of the scope
// but nothing more will happen because it was moved into print_message function

// "a" variable coming into the scope
fn print_message(a: String) {
  println!("{}", a);
  // "c" is coming into the scope and "a" is moved into "c"
  let _c = a;
  // "a" is no longer valid
}
// "a" is going out of the scope
// but nothing more will happen because it was moved into "c"

// "c" is going out of the scope and "drop"
// is called which clears the memory from the heap
