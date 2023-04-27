// this is the library crate root for this package. it can define some public API(?)

#[derive(Debug)]
pub enum CoffeeFlavor {
  Mocha,
  Latte,
  Capuchino,
  Americano,
}

#[derive(Debug)]
pub enum Drinks {
  Water,
  Coffee(CoffeeFlavor),
  Soda,
  Beer,
}

#[derive(Debug)]
pub struct MyRectangle {
  width: u32,
  height: u32,
}
impl MyRectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  pub fn print_info(&self) {
    let width = self.width;
    let height = self.height;
    println!("rectangle with width: {width} and height {height}");
  }
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height,
    }
  }
}

// declare an inline module (replacing the semicolon with curly brackets)
mod felicity_farmstand {
  enum Vegetables {
    Asparagus,
    Squash,
    Beets,
    Pickle(String),
  }
}

// declare a module 'chris_cataclysm' which would be found in either of:
// src/chris_cataclysm.rs
// src/chris_cataclysm/mod.rs
mod chris_cataclysm;

// declare a module 'owen_oblivion' which would be found in either of:
// src/owen_oblivion.rs
// src/owen_oblivion/mod.rs
mod owen_oblivion;
