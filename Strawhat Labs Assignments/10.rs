// TODO: Get input from user and append it to a file called input txt
use std::io::prelude::*;
use text_io::read;
use std::fs::OpenOptions;

fn main() {
  let a:String = read!();
  let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("src/file.txt")
            .unwrap();
  writeln!(file, "{}", a);
}
