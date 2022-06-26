fn main() {
  let mut message = String::from("Hello");
  let new_message = &mut message;

  (*new_message).push_str(" World!");

  println!("{}", new_message);

  let a = 10;
  let b = &a;
  let c = &b;

  println!("{}", a == **c);
}
