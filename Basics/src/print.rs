pub fn run() {
  // print to console

  println!("hello from the print.rs file");

  //basic formating
  println!("number: {}", 1);

  //positional Arguments
  println!(
    "esse {0} o jeito de {1} qual vai, {0} assim q faz",
    "é", "escolher"
  );

  //placeholder traits
  println!("Binary {:b}, Hex {:x}, Octal {:o}", 10, 10, 10);

  // placeholder for debugging traits
  println!("{:?}", (10, true, "alpha"));

  //basic math
  println!("10 + 10 = {}", 10 + 10);
}
