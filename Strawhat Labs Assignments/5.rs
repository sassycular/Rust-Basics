fn print_name(name: String) {
  println!("This is {}", name);
}

fn main() {
    // TODO - Assign a value to a variable
    let name = String::from("Jinx");
    // TODO - Use References to print name as many times as we do
    print_name(name.clone()); // Call a function with a variable as an argument
    print_name(name.clone()); // Call a function with a variable as an argument again
}
