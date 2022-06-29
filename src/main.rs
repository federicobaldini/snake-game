enum Country {
  England,
  Italy,
  France,
  Germany,
}

struct Person {
  name: String,
  last_name: String,
  age: u8,
  country: Country,
}

impl Person {
  // Associated function
  fn from(name: String, last_name: String, age: u8, country: Country) -> Person {
    Person {
      name,
      last_name,
      age,
      country,
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
    Country::Italy,
  );

  person.set_name("Matteo".to_string());

  println!("{} {} {}", person.name, person.last_name, person.age);
}
