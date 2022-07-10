use std::thread;
use std::time::Duration;

// Using Threads to Run Code Simultaneously
// In most current operating systems, an executed program’s code is run in a
// process, and the operating system will manage multiple processes at once.
// Within a program, you can also have independent parts that run
// simultaneously. The features that run these independent parts are called
// threads. For example, a web server could have multiple threads so that it
// could respond to more than one request at the same time.

// Splitting the computation in your program into multiple threads to run
// multiple tasks at the same time can improve performance, but it also adds
// complexity. Because threads can run simultaneously, there’s no inherent
// guarantee about the order in which parts of your code on different threads
// will run. This can lead to problems, such as:

// Race conditions, where threads are accessing data or resources in an
//  inconsistent order
// Deadlocks, where two threads are waiting for each other, preventing both
//  threads from continuing
// Bugs that happen only in certain situations and are hard to reproduce and fix
//  reliably

// Rust attempts to mitigate the negative effects of using threads, but
// programming in a multithreaded context still takes careful thought and
// requires a code structure that is different from that in programs running in
// a single thread.

// Programming languages implement threads in a few different ways, and many
// operating systems provide an API the language can call for creating new
// threads. The Rust standard library uses a 1:1 model of thread implementation,
// whereby a program uses one operating system thread per one language thread.
// There are crates that implement other models of threading that make different
// tradeoffs to the 1:1 model.

fn main() {
    //threads();
    //join();
    //join2();
    move_closures();
}

fn threads() {
    // Creating a New Thread with spawn
    // To create a new thread, we call the thread::spawn function and pass it a
    // closure (we talked about closures in Chapter 13) containing the code we
    // want to run in the new thread.
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Note that when the main thread of a Rust program completes, all spawned
    // threads are shut down, whether or not they have finished running.
    for i in 1..5 {
        // The calls to thread::sleep force a thread to stop its execution for a
        // short duration, allowing a different thread to run. The threads will
        // probably take turns, but that isn’t guaranteed: it depends on how
        // your operating system schedules the threads. In this run, the main
        // thread printed first, even though the print statement from the
        // spawned thread appears first in the code. And even though we told the
        // spawned thread to print until i is 9, it only got to 5 before the
        // main thread shut down.
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn join() {
    // Waiting for All Threads to Finish Using join Handles
    // The code in Listing 16-1 not only stops the spawned thread prematurely
    // most of the time due to the main thread ending, but because there is no
    // guarantee on the order in which threads run, we also can’t guarantee that
    // the spawned thread will get to run at all!

    // We can fix the problem of the spawned thread not running or ending
    // prematurely by saving the return value of thread::spawn in a variable.
    // The return type of thread::spawn is JoinHandle. A JoinHandle is an owned
    // value that, when we call the join method on it, will wait for its thread
    // to finish.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Calling join on the handle blocks the thread currently running until the
    // thread represented by the handle terminates. Blocking a thread means that
    // thread is prevented from performing work or exiting.
    // The two threads continue alternating, but the main thread waits because
    // of the call to handle.join() and does not end until the spawned thread is
    // finished.
    handle.join().unwrap();
}

fn join2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn move_closures() {
    // We'll often use the move keyword with closures passed to thread::spawn
    // because the closure will then take ownership of the values it uses from
    // the environment, thus transferring ownership of those values from one
    // thread to another.

    // To use data from the main thread in the spawned thread, the spawned
    // thread’s closure must capture the values it needs.
    let v = vec![1, 2, 3];

    // y adding the move keyword before the closure, we force the closure to
    // take ownership of the values it’s using rather than allowing Rust to
    // infer that it should borrow the values.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
