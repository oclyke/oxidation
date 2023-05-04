fn main() {
    println!("Hello, world!");


    // so, patterns are cool!
    // another great example of an axiom in Rust being reliably repeated!

    // my current understanding of patterns is that.... they match stuff!
    // turns out even the 'let' statement uses a pattern...
    // let PATTERN = EXPRESSION;
    // and usually a very simple pattern is used -- the name of a variable
    // in this case Rust is saying "bind this entire expression to the name '_var'"
    let _var = "simple let expression";

    // but it can actually be more complicated!
    // in general a let statement will compare the pattern to the expression and
    // bind values to names according to things like the shape or some other rules!

    // for example, we can use a pattern to destructure a tuple
    let (a, b) = (97, 98);
    println!("({a}, {b})");

    // we could even get fancier!
    let (c, (d, e)) = (99, (100, 101));
    println!("{c}, ({d}, {e})");

    // // there are some limits, though!
    // // for instance this is not valid syntax (but you can wrap it in parentheses to 
    // // "tuple-ize" it and make it valid like above)
    // let f, (g, h) = 102, (103, 104);

    // // let statements can also do some conditionals using patterns, like this:
    // // (actually, doing this precise statement causes a panic, so I am commenting it
    // // out. but it works! this shows you that Rust will see that None does not match
    // // the pattern 'Some(x)' and so it jumps into the 'else' block!)
    // // (this is called a "refutable" pattern - it can be refuted if it fails to match
    // // on the other hand a pattern like 'let var = 5' is irrefutable, because the 'var'
    // // will always bind to the entire expression)
    // let Some(x): Option<i32> = None else { todo!() };
    // println!("x is {x}");

    // a more useful way of writing the code above (so that it does not panic) is like this
    if let Some(x) = None::<i32> {
        println!("x is some number! x is {x}");
    } else {
        println!("x was not some number :(");
    }
    
    // patterns are at work in function parameter definitions too!
    // here we see that we can unpack the tuple in the parameter pattern for concise code :)
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    
    let point = (3, 5);
    print_coordinates(&point);


    // patterns can be literals!
    // here the refutable pattern branches in the match statement are all literal values
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // patterns can introduce variable shadowing
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y} (here the y in the match arm shadows the y in the outer scope)"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    // you can OR together multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // you can match a pattern which expresses a range of values!
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // destructuring syntax allows concise renaming of variables
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // you can also destructure to exactly the named fields:
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // you can use a match statement with destructuring for some really cool stuff!
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // you can destructure enums, and do so within a match statement if you like!
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // patterns can ignore values in a variety of ways. 
    // the _ is used to completely ignore a variable without binding at all
    // prefixing a var name with _ like _var will bind to _var but the compiler will ignore _var if it is unused
    // .. can be used to avoid having to use _ for the rest of a destructurable item

    // using .. with a struct
    struct ThreeDimensionalPoint {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = ThreeDimensionalPoint { x: 0, y: 0, z: 0 };
    match origin {
        ThreeDimensionalPoint { x, .. } => println!("x is {}", x),
    }

    // using .. with a tuple
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // // btw this won't work because it is ambiguous and Rust cannot decide what value to destructure:
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }

    // you can use things called "match guards"
    // here a match guard checks whether the number is even
    // one issue with match guards is that they cause the compiler not to check for exhaustiveness of the match guard
    // (it does seem to still check for "outer" exhaustiveness - in this case it makes sure that both Some and None 
    // option variants are covered)
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // match guards apply to the entire match statement arm, even if the pattern OR is used
    // you can always write another match arm to use a different match guard on another pattern which could overlap
    // just remember to use your fall-through properly!
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ Bindings
    // The at operator @ lets us create a variable that holds a value at the same time as we’re 
    // testing that value for a pattern match. In Listing 18-29, we want to test that a 
    // Message::Hello id field is within the range 3..=7. We also want to bind the value to the 
    // variable id_variable so we can use it in the code associated with the arm. We could name 
    // this variable id, the same as the field, but for this example we’ll use a different name.
    enum AtMessage {
        Hello { id: i32 },
    }
    let msg = AtMessage::Hello { id: 5 };
    match msg {
        AtMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        AtMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        AtMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}
