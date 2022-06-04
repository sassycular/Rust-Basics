// ToDO throw a custom error if the user enters a negative number
#[macro_use] extern crate text_io;

struct num {
  x: i32
}

pub trait Number {
  fn ntype(&self);
}

impl Number for num {
    fn ntype(&self) {
    if self.x < 0 {
      println!("You have entered a negative number");
    }
  }
}

fn main() {
  println!("Enter an integer: ");
  let y = num {
    x: read!()
  };

  y.ntype();
  
  
}
