use snake_game::testing_lib::{Log, Person, PersonId};

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
