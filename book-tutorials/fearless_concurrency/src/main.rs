use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {

    //
    //
    // example of basic thread usage
    //
    println!("basic thread usage:");

    // calling spawn immediately prepares another thread executing the code in the closure, however it needs to
    // get an opportunity to run (this is done by causing the main thread to sleep, as demonstrated in the loop)
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // // join() blocks the calling thread until the thread referred to by the handle completes
    // // uncomment the following line to cause the main thread and the spawned thread to no longer be interleaved
    // // (this is another way to give other threads an opportunity to run, apart from causing the main thread to
    // // sleep)
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);

        // causing the main thread to sleep gives other threads an opportunity to run
        // if the following line is commented out then the main loop will keep running without giving the spawned
        // thread any time to run (until some other time that the main thread sleeps or waits for other threads)
        thread::sleep(Duration::from_millis(1));
    }

    // calling join() here will ensure that the spawned thread runs to completion - if the main thread did not call
    // this and instead just exited it would stop execution of all other threads as well.
    // commenting out the following line and uncommenting the return statement will demonstrate this
    handle.join().unwrap();
    // return;


    //
    //
    // example of capturing environment from the calling thread:
    println!("\ncapturing environment from calling thread");

    // create a vector
    let v = vec![1, 2, 3];

    // spawn a thread to show the vector while the main thread continues
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        drop(v);
    });

    // wait for the spawned thread to complete
    handle.join().unwrap();

    //
    //
    // an example of message passing between threads
    // from go: don't communicate by sharing memory, share memory by communicating!
    println!("\nthread communication!");

    // create a MPSC (multiple producer, single consumer) channel and unpack both
    // the transmitter and the receiver
    let (tx, rx) = mpsc::channel();

    // spawn a thread which owns the transmitting side and will send a message
    // the value sent is not available in the scope of the main thread but we will receive it from the channel
    // also, the rust compiler is able to infer the type of the channel from the type of the value sent through
    // it -- trying to later send a different type would be a compile error
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // // the following line is a compile error because the channel has already inferred its message type to be '&str'
        // tx.send(3).unwrap();
    });

    // the recv method blocks the calling thread until a message is received, or the channel closes
    // another option would be try_recv, which does not block. it immediately returns a Result type indicating whether or
    // not a message was received, so that the calling thread could keep doing other work.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    //
    //
    // an example of how tx.send() actually transfers ownership!
    println!("\ntx.send transfers ownership between tx / rx!");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));

            // // we cannot use 'val' after it is sent - ownership now belongs to the receiving thread!
            // // the receiving thread could have already dropped the value making this line invalid - the memory
            // // would not be guaranteed to be what was expected within 'val'
            // // uncomment this block to see the error
            // println!("val is {}", val);
        }
    });

    // treat rx as an iterator, repeatedly calling recv on it
    // when the channel closes the iterator will end
    for received in rx {
        println!("Got: {}", received);
    }

    //
    //
    // example of using multiple producers (by cloning the transmitter!)
    println!("\nusing multiple producers!");
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // note, it seems that cloning tx keeps channel open (and rx receiving) until *both* threads end!
    // this looks like a usage of a reference counting smart pointer behind the scenes! super cool
    for received in rx {
        println!("Got: {}", received);
    }

    //
    //
    // an example of mutexes!
    println!("\nsimple mutexes!");

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    //
    //
    // sharing state using a mutex in multiple threads!
    println!("\nmultiple thread mutex!");

    // // the invalid method shown below uses a single mutex named counter
    // let counter = Mutex::new(0);

    // the proper solution uses an Atomic Reference Counter (Arc)
    // a normal Refernce Counter (Rc) is not thread-safe, and using it here would result in a compiler error 
    // informing us of this fact. instead we use an atomic reference counter 
    // why are all Rcs not Arcs? Because an Arc comes with an additional performance penalty to make it
    // thread-safe. When only single-threaded reference counting is needed use Rc!
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // // this method is invalid, after the first outer loop ownership of the counter mutex has been moved
        // // into the previous definition of the closure (they are actually separate instances, which is required
        // // to spawn multiple threads!)
        // let executor = move || {
        //     let mut num = counter.lock().unwrap();
        //     *num += 1;
        // };


        // instead we can clone references to the Mutex with a reference counter, and move the copied references
        // into the executor closure
        let counter = Arc::clone(&counter);
        let executor = move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        };

        // spawn a thread and add the handle to the vector of handles
        let handle = thread::spawn(executor);
        handles.push(handle);
    }

    // ensure all threads run to completion
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
