// TODO print array elements in reverse order

fn main() {
  let mut arr:[i32;10] = [0,1,2,3,4,5,6,7,8,9];
  println!("The array before reversing: {:?}", arr);
  arr.reverse();
  println!("The array after reversing: {:?}", arr);
}

