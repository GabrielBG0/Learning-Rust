// Vectors - Resizable arrays

use std::mem;

pub fn run() {
  // Data types AND length have to be exact
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
  println!("{:?}", numbers);

  // Reassing values
  numbers[2] = 20;

  //Add on to vector
  numbers.push(6);
  numbers.push(7);

  println!("{:?}", numbers);

  //Pop of the last value
  numbers.pop();
  println!("{:?}", numbers);

  // Get sigle val
  println!("Single Value: {}", numbers[0]);

  // Get vector length
  println!("Vector length: {}", numbers.len());

  //Vectors are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..4];
  println!("Slice: {:?}", slice);

  // Loop through vectors values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate vaues
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("Numbers x 2: {:?}", numbers)
}
