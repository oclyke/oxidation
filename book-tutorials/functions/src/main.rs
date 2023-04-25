fn main() {
    println!("Hello, world!");

    another_function(3);

    let mut x = 3;
    let r1 = &mut x;
    println!("r1: {r1} {}", *r1)
}

fn another_function(x: i32) {
    println!("Another function. {x}");
}
