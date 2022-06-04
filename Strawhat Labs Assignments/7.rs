//TODO: Send mutable reference to function and print string
fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", Solana");
}
