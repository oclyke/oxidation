fn main() {
    let rect1 = Rectangle {
        width: 420,
        height: dbg!(17),
    };

    println!(
        "The area of the rectangle is {} square units.",
        rect1.area()
    );

    println!("the height was {} units", rect1.height);
    dbg!(&rect1);
    println!("the full struct was: {:#?}", rect1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
