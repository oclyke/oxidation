use test_package::MyRectangle;

fn main() {
    println!("Hello, world from the default binary crate of this package!");

    // try som functionality from the default library crate
    let rect1 = MyRectangle::new(3, 4);
    rect1.print_info();

    println!("Hello, and welcome to the test-package cafe!");
    println!("The available drinks are: ");
    let drink = test_package::Drinks::Coffee(test_package::CoffeeFlavor::Latte);
    // for name in  {
    //   println!("{}", name);
    // }
    println!("{:?}", drink);
}
