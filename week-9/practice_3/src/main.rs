use std::fs;

fn main() {
  fs::remove_file("datatxt").expect("could not remove file");
  println!("file is removed");
}
