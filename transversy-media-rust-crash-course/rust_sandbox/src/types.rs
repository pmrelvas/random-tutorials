/*
Primitive Types --
  - integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
  - floats: f32, f64
  - boolean (bool)
  - characters (char)
  - tuples
  - arrays (in rust they are fixed length)
 */

// rust is a statically types language, which means that it must know the
// types of all variables at compile time. However, the compiler can usually
// infer what type we want to use based on the value and how we use it.

pub fn run() {
  // by default is "i32"
  let x = 1;

  // by default is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 4545455454;

  // find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active: bool = true;

  // get boolean from expression
  let is_greater: bool = 10 < 5;

  let a1 = 'a';
  let face = '\u{1F600}';

  println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

}