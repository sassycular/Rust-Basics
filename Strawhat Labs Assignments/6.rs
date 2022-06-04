// ToDo - Create a strut Shape with two variables length and width and implement Area and Perimeter methods
// Area = length * width 
// Perimeter = 2 * (length + width)

struct Groot {
  length: i32,
  width: i32
}

impl Groot {
  fn Area(&self)  -> i32 {
    self.length * self.width
  }
  fn Peri(&self) -> i32 {
    2 * (self.length + self.width)
  }
}

fn main() {
  let PixieDust = Groot{
    length: 5,
    width: 7
  };
  println!("The area of Groot is: {:?}", PixieDust.Area());
  println!("The perimeter of Groot is: {:?}", PixieDust.Peri());
  
}
