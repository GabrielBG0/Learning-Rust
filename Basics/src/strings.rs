// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  //primitive str
  let hello = "hello";

  //Growable string
  let mut world = String::from("world");

  //Get lenth
  println!("length: {}", hello.len());

  //push char
  world.push('!');

  //push string
  world.push_str(" Frist time with Rust!!");

  //capasity in bites
  println!("Capacity: {}", world.capacity());

  //Check if empity
  println!("Is empty: {}", hello.is_empty());

  //Contains
  println!("contais 'world': {}", world.contains("world"));

  //Replace
  println!("Replace: {} {}", hello.replace("hello", "hi"), world);

  //Loop through sring by space
  for word in world.split_whitespace() {
    println!("{}", word)
  }

  //Create String with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  //Assertion testig
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{}", s);
}
