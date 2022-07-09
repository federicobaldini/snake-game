pub mod testing_lib {

  use std::fmt;

  pub trait Log {
    fn display_info(&self);
  }

  pub enum PersonId {
    IdentityCard(String),
    Passport(String),
  }

  impl fmt::Display for PersonId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        PersonId::IdentityCard(document_code) => {
          write!(f, "{}", document_code)
        }
        PersonId::Passport(document_code) => {
          write!(f, "{}", document_code)
        }
      }
    }
  }

  pub struct Person {
    name: String,
    last_name: String,
    age: u8,
    id: PersonId,
  }

  impl Log for Person {
    fn display_info(&self) {
      println!("{} {} {} {}", self.name, self.last_name, self.age, self.id)
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

    pub fn get_name(&self) -> &String {
      &self.name
    }

    pub fn set_name(&mut self, new_name: String) {
      self.name = new_name;
    }
  }
}
