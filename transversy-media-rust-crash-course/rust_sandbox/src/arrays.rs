use std::mem;

// Arrays - fixed list where  elements are the same data types
pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  // re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);

  // get single val
  println!("Single value: {}", numbers[0]);

  // get array length
  println!("Array Length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..3];
  println!("slice: {:?}", slice);
}