use crate::Drinks;

fn main () {

  println!("Hello, and welcome to the test-package cafe!");
  println!("The available drinks are: ");
  for name in Drinks {
    println!(name);
  }
}
