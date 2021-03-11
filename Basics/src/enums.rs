// Enums are types which have a few definite values

enum Moviment {
  //variants
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Moviment) {
  // Perform action depending on info

  match m {
    Moviment::Up => println!("Moving up"),
    Moviment::Down => println!("Moving down"),
    Moviment::Left => println!("Moving left"),
    Moviment::Right => println!("Moving right"),
  }
}

pub fn run() {
  let avatar1 = Moviment::Left;
  let avatar2 = Moviment::Up;
  let avatar3 = Moviment::Right;
  let avatar4 = Moviment::Down;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}
