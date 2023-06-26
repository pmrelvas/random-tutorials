use std::mem;

// Vectors - resizable arrays
pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  // re-assign value
  numbers[2] = 20;

  // add on to vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  // get single val
  println!("Single value: {}", numbers[0]);

  // get vector length
  println!("Vector Length: {}", numbers.len());

  // Vectors are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..3];
  println!("slice: {:?}", slice);
}