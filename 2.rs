// TODO Read two numbers from Command Line Arguments and print the sum of the numbers 
// install the crate text_io - needed for this code 

#[macro_use] extern crate text_io;
fn main(){
  println!("Enter the first number: "); 
  let p: i32 = read!();
  println!("Enter the second number: ");
  let q: i32 = read!();
  let sum = p + q;
  println!("The sum of the numbers is: {}", sum);
}

