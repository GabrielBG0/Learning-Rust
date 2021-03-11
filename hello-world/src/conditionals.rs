// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
  let age: u8 = 18;
  let check_id = false;
  let knows_person_of_age = true;

  // If/else
  if age >= 21 && check_id || knows_person_of_age {
    println!("Bartender: what would u like to drink");
  } else if age < 21 && check_id {
    println!("Bartender: sorry u have to leave");
  } else {
    println!("Bartender: I'll need to see your id");
  }

  // Shorthand if
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is of age: {}", is_of_age)
}
