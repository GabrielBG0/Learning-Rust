//Variables hold primitive date or references to data
//Variable are immutable by defoult
//Rust is a block-scoped language

pub fn run() {
  let name = "Gabe";
  let mut age = 20;

  println!("My name is {}, and I am {}", name, age);
  age = 21;
  println!("My name is {}, and I am {}", name, age);

  //Define constant
  const ID: i32 = 1;
  println!("Id: {}", ID);

  //Assing multiple variables
  let (my_name, my_age) = ("Gabe", 20);
  println!("{} is {}", my_name, my_age);
}
