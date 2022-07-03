pub trait Log {
  fn display_info(&self);
}

#[derive(Debug)]

pub enum PersonId {
  IdentityCard(String),
  Passport(String),
}

pub struct Person {
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
  pub fn from(name: String, last_name: String, age: u8, id: PersonId) -> Person {
    Person {
      name,
      last_name,
      age,
      id,
    }
  }

  pub fn set_name(&mut self, new_name: String) {
    self.name = new_name;
  }
}
