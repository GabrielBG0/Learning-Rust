// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
  // Data types AND length have to be exact
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{:?}", numbers);

  // Reassing values
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Get sigle val
  println!("Single Value: {}", numbers[0]);

  // Get array length
  println!("Array length: {}", numbers.len());

  //Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..4];
  println!("Slice: {:?}", slice);
}
