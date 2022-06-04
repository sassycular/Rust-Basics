// TODO Number gussing game with rust

// Reference https://www.geeksforgeeks.org/number-guessing-game-in-python/
use rand::Rng;
#[macro_use] extern crate text_io;

fn main() {
  let mut rng = rand::thread_rng();
  let n1: i32 = rng.gen_range(0..100); // TODO Generate a random number between 1 and 100
  let mut n2 = -1;
  while n1!=n2 { 
    println!("Guess a number: ");
    n2 = read!();
    if n1==n2 {
      println!("Congratulations! You guessed it right");
    }
    else if n1>n2 {
      println!("Oh no, you guessed too low");
    }
    else if n1<n2 {
      println!("Oh no, you guessed too high");
    }

  }
    // TODO: Build a loop that will continue until the user guesses the numbers
}
