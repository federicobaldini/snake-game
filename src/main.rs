// Trait
trait Log {
  fn display_info(&self);
}

#[derive(Debug)]

enum PersonId {
  IdentityCard(String),
  Passport(String),
}

struct Person {
  name: String,
  last_name: String,
  age: u8,
  id: PersonId,
}

impl Log for Person {
  fn display_info(&self) {
    println!(
      "{} {} {} {:?}",
      self.name, self.last_name, self.age, self.id
    )
  }
}

impl Person {
  // Associated function
  fn from(name: String, last_name: String, age: u8, id: PersonId) -> Person {
    Person {
      name,
      last_name,
      age,
      id,
    }
  }

  // Method
  // First parameter it's always self, which represents the instance of the struct
  // the method is being called on.
  // Within an implementation block, the type Self is an alias for the current type.
  fn set_name(&mut self, new_name: String) {
    self.name = new_name;
  }
}

fn main() {
  let mut person = Person::from(
    String::from("Federico"),
    String::from("Baldini"),
    24,
    PersonId::IdentityCard(String::from("12345678")),
  );

  person.set_name("Matteo".to_string());
  log_info_1(person);
}

// impl makes the compiler determine type at compile time
// it will create multiple versions of the function, depending on
// how many type implement the Log trait
fn log_info_1(val: impl Log) {
  val.display_info();
}

// dyn is for short for dynamic, and says that function should perform dynamic dispatch
// decision of exactly which function to call at the runtime
fn log_info_2(val: &dyn Log) {
  val.display_info();
}
