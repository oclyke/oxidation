use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closures can capture values from their environment in three ways, which
    // directly map to the three ways a function can take a parameter:
    // - borrowing immutably,
    // - borrowing mutably,
    // - taking ownership,

    //
    //
    // An example of when a closure only borrows a reference to the environment variable
    // The closure captures an immutable reference to the vector named list because it
    // only needs an immutable reference to print the value:
    // Because the closure captures an immutable reference that we can continue using other
    // immutable references to the list

    // Print header for this test
    println!("\nOnly borrowing closure test:");

    // Define immutable list
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Create closure which only requires immutable reference to print the value
    let only_borrows = || println!("From closure: {:?}", list);

    // Use the list before and after calling the closure
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // With this closure we can continue to call the closure and use immutable
    // references to the list over and over again...
    only_borrows();
    println!("After calling closure again: {:?}", list);
    // only_borrows();
    // println!("After calling closure: {:?}", list);
    // only_borrows();
    // println!("After calling closure: {:?}", list);

    // Use a mutable reference to the list to change the contents
    // (note: this can only be done when the only_borrows closure is not called
    //  again - the only_borrows closure lifetime has concluded?)
    list.push(9);
    println!("After mutating the list: {:?}", list);

    // // We CANNOT use the closure again once the mutable reference is taken in the
    // // above code. Try uncommenting this line to see what happens.
    // only_borrows();

    //
    //
    // An example where the closure uses a mutable reference
    // You can no longer use immutable references to the variable enclosed while the
    // closure still has its immutable borrow. Remeber - it is possible to have
    // Simultaneous immutable reference lifetimes, but while a mutable reference exists
    // there can be *no* other simultaneous references.

    // Print header for this test
    println!("\nMutably borrowing closure test:");

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Define a closure which captures a mutable reference to list
    // - When borrows_mutably is defined, it captures a mutable reference to list
    let mut borrows_mutably = || list.push(7);

    // // Note: we can no longer use an immutable reference to list here in order to
    // // print its value as was done in the previous example - this is the main
    // // illustrative difference between the immutable and immutable borrow closures
    // // Try uncommenting this code to see why
    // println!("Before calling closure: {:?}", list);

    // Call the closure
    // - The mutable borrow for the closure is still active here
    borrows_mutably();
    // - On this line, because the closure is not called again, the mutable borrow ends

    // So at this point we may use an immutable reference to print the new value of the list
    println!("After calling closure: {:?}", list);

    // // However, another call to the borrows_mutably closure would extend the lifetime of the
    // // mutable reference that is captured by this closure, and would be a compile error.
    // // Try uncommenting this code to see for yourself...
    // borrows_mutably();

    //
    //
    // An example of why the 'move' specifier is required for (some?) closures
    // which are passed to the thread::spawn function.
    // Effectively, if the new thread was given a reference (rather than being
    // given ownership "moving ownership into closure") then the main thread could
    // potentially end before the new thread and the data that the reference points
    // to would be invalid - a dangling reference!

    // Print header for this test
    println!("\nMove borrowing closure test with thread:");

    // Create the vector and print its value using an immutable reference
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Create a thread using the 'move' specifier for the closure
    // This will cause the closure to copy the list value
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // // The following code is not valid because the closure has been
    // // given ownership of the variable list. Uncomment to see!
    // println!("Attempt to print list from main thread: {:?}", list);

    //
    //
    // An example of moving captured values out of closures and the fn traits
    // Closures capture values from the context in which they are defined (moving ownership *into* the closure)
    // The body of the closure determines what references or values, if any, are moved out of the closure.
    // A closure can do any of the following:
    // - move a captured value out of the closure
    // - mutate the captured value
    // - neither move nor mutate the value
    // - capture nothing from the environment to begin with

    // Traits:
    // - The way a closure captures and handles values from the environment affects which traits the closure implements
    // - Traits are how functions and structs can specify what kinds of closures they can use
    // - Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:

    // 'FnOnce' applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
    // 'FnMut' applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
    // 'Fn' applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

    // Functions can implement all three of the Fn traits too. If what we want to do doesn’t require capturing
    // a value from the environment, we can use the name of a function rather than a closure where we need
    // something that implements one of the Fn traits

    println!("\nExample of moving values out of closures");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // sort_by_key requires the FnMut trait because it calls the closure several times
    // (why does calling it multiple times equate to a FnMut trait? I think because it is a guarantee to the
    // user that the values won't change over the multiple calls - in other words a concurrency guarantee.)
    // (I'm still not quite sure about the naming convention - why call it Mut if there's no mutation? Wouldn't
    // the rust requirement that 'if a mutable reference exists it must be the only mutable reference' be enough
    // to cause a compile error if a mutable reference was taken while the closure still had an immutable ref to
    // the variable?)
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // now: a demonstration of a closure which would not be usable with sort_by_key
    // * this will demonstrate what it means to move values out of the environment of the closure *
    // the idea here is that someone tried to see how many times the closure was called by pushing a string into
    // a vector that they would presumably count the elements in later (eww... I know).
    // - 'value' and 'sort_operations' are both captured in the closure's environment at definition
    // - when the closure first runs the 'sort_operations' push method transfers ownership of the 'value' value
    //   into the 'sort_operations' vector. (what's importand to remember here is that ownership of 'value' really
    //   moves from the outer scope into the closure, and then from the closure into the 'sort_operations' vector.
    // - when the closure is called a second time it will no longer "have" (meaning "own") the 'value' value in its
    //   environment, so it cannot push it into the vector again.
    //
    // This means that the closure gets the FnOnce trait and no more, and therefore cannot be used in 'sort_by_key'
    //
    // I suspect there would be a way to make this work by capturing a reference and then dereferencing that ref
    // when pushing into the vector, but I haven't proven it and I'm not positive and I'm pretty sure that would
    // be a bad code smell anyway.

    // // Uncomment the following to see the error generated by trying to count operations with this method
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    // A proper way to count code like this would be just to increment a number in the closure
    let mut count = 0;
    list.sort_by_key(|r| {
        count += 1;
        r.width
    });
    println!("the sort closure was called {count} times");
    println!("\
        surprise!
        did you think sort would only call the closure once for each element in the list? \
        if that were the case then Rust would have to allocate some memory to store the keys and then \
        repeatedly compare them in the sorting algorithm! instead it just calls the closure each time \
        that it needs to determine the sorting key for a particular rectangle! \
        "
    );

    //
    //
    // This little example just shows how closures are sort of like more general functions, and
    // arguments which take certain kinds of closures (i.e. those with the 'Fn' trait) can also
    // take functions as their arguments
    let option_some = Some("SOME value!");
    let option_none: Option<&str> = None;

    // use anonymous closures for each unwrap_or_else
    // - these closures neither capture any references nor mutate any state, so they are assigned the 'Fn' trait
    println!("\nusing anonymous closures");
    let some = option_some.unwrap_or_else(|| "anonymous closure 0");
    let none = option_none.unwrap_or_else(|| "anonymous closure 1");
    println!("{:?}", [some, none]);

    // use a single named closure for each unwrap_or_else
    // the named closure does not capture any references or mutate any state, so it is assigned the 'Fn' trait
    println!("\nusing a function-like closure");
    let closure_like_a_fn = || "function-like closure [has Fn trait]";
    let some = option_some.unwrap_or_else(closure_like_a_fn);
    let none = option_none.unwrap_or_else(closure_like_a_fn);
    println!("{:?}", [some, none]);

    // use a true function each unwrap_or_else
    // by definition a function cannot capture references the way that a closure can (it can take them as
    // arguments, however the signature for 'unwrap_or_else' dictates that in this case the function will
    // take no arguments. a function cannot capture references in a 'side channel' like a closure can, even
    // though a function can be created "dynamically")
    //
    // it could mutate state if it took an argument with a
    // the named closure does not capture any references or mutate any state, so it is assigned the 'Fn' trait
    println!("\nusing a true function");
    let some = option_some.unwrap_or_else(unwrap_handler_fn);
    let none = option_none.unwrap_or_else(unwrap_handler_fn);
    println!("{:?}", [some, none]);

    // use a nested function (to illustrate the difference from a closure)
    // (https://stackoverflow.com/a/26685687)
    fn nested_unwrap_handler_fn() -> &'static str {
        // this is a nested function. it is well behaved like a normal function and cannot capture items from
        // scope like a closure could.

        // // Uncomment the following line to generate a compile error indicating that this function cannot capture
        // // any references - it must only take arguments through its parameters!
        // // error[E0434]: can't capture dynamic environment in a fn item
        // println!("this is an illegal use of outer scope within a function! {:?}", option_none);

        println!("\tthe nested unwrap handler has been called");
        "a true function (nested)"
    }
}

fn unwrap_handler_fn() -> &'static str {
    // as a function and not a closure, it has no context surrounding the call
    // all it can do is hand back the 'or_else' value, in this case 0
    // (p.s. I think that seeing the static lifetime in the signature is a bad code smell - suggesting something about why do you need to keep a reference to this string around at all times?)
    println!("\tthe unwrap handler function has been called!");
    "a true function"
}
