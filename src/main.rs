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
  person.display_info();
}


}
