use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Using Message Passing to Transfer Data Between Threads
// One increasingly popular approach to ensuring safe concurrency is message
// passing, where threads or actors communicate by sending each other messages
// containing data. Here’s the idea in a slogan from the Go language
// documentation: “Do not communicate by sharing memory; instead, share memory
// by communicating.”

// To accomplish message-sending concurrency, Rust's standard library provides
// an implementation of channels. A channel is a general programming concept by
// which data is sent from one thread to another.

// You can imagine a channel in programming as being like a directional channel
// of water, such as a stream or a river. If you put something like a rubber
// duck into a river, it will travel downstream to the end of the waterway.

// A channel has two halves: a transmitter and a receiver. The transmitter half
// is the upstream location where you put rubber ducks into the river, and the
// receiver half is where the rubber duck ends up downstream. One part of your
// code calls methods on the transmitter with the data you want to send, and
// another part checks the receiving end for arriving messages. A channel is
// said to be closed if either the transmitter or receiver half is dropped.

// Here, we’ll work up to a program that has one thread to generate values and
// send them down a channel, and another thread that will receive the values and
// print them out. We’ll be sending simple values between threads using a
// channel to illustrate the feature. Once you’re familiar with the technique,
// you could use channels for any threads that needs to communicate between each
// other, such as a chat system or a system where many threads perform parts of
// a calculation and send the parts to one thread that aggregates the results.

fn main() {
    // We create a new channel using the mpsc::channel function; mpsc stands for
    // multiple producer, single consumer. In short, the way Rust’s standard
    // library implements channels means a channel can have multiple sending
    // ends that produce values but only one receiving end that consumes those
    // values. Imagine multiple streams flowing together into one big river:
    // everything sent down any of the streams will end up in one river at the
    // end. We’ll start with a single producer for now, but we’ll add multiple
    // producers when we get this example working.

    // The mpsc::channel function returns a tuple, the first element of which is
    // the sending end--the transmitter--and the second element is the receiving
    // end--the receiver. The abbreviations tx and rx are traditionally used in
    // many fields for transmitter and receiver respectively, so we name our
    // variables as such to indicate each end.
    let (tx, rx) = mpsc::channel();

    // Let’s move the transmitting end into a spawned thread and have it send
    // one string so the spawned thread is communicating with the main thread.

    // Again, we’re using thread::spawn to create a new thread and then using
    // move to move tx into the closure so the spawned thread owns tx. The
    // spawned thread needs to own the transmitter to be able to send messages
    // through the channel. The transmitter has a send method that takes the
    // value we want to send. The send method returns a Result<T, E> type, so if
    // the receiver has already been dropped and there’s nowhere to send a
    // value, the send operation will return an error.
    thread::spawn(move || {
        let val = String::from("hi");
        // Channels and Ownership Transference
        // The ownership rules play a vital role in message sending because they
        // help you write safe, concurrent code. Preventing errors in concurrent
        // programming is the advantage of thinking about ownership throughout
        // your Rust programs. Let’s do an experiment to show how channels and
        // ownership work together to prevent problems: we’ll try to use a val
        // value in the spawned thread after we’ve sent it down the channel.
        tx.send(val).unwrap();
    });

    // The receiver has two useful methods: recv and try_recv. We’re using recv,
    // short for receive, which will block the main thread’s execution and wait
    // until a value is sent down the channel. Once a value is sent, recv will
    // return it in a Result<T, E>. When the transmitter closes, recv will
    // return an error to signal that no more values will be coming.

    // The try_recv method doesn’t block, but will instead return a Result<T, E>
    // immediately: an Ok value holding a message if one is available and an Err
    // value if there aren’t any messages this time. Using try_recv is useful if
    // this thread has other work to do while waiting for messages: we could
    // write a loop that calls try_recv every so often, handles a message if one
    // is available, and otherwise does other work for a little while until
    // checking again.

    // We’ve used recv in this example for simplicity; we don’t have any other
    // work for the main thread to do other than wait for messages, so blocking
    // the main thread is appropriate.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Sending Multiple Values and Seeing the Receiver Waiting
    // This time, the spawned thread has a vector of strings that we want to
    // send to the main thread. We iterate over them, sending each individually,
    // and pause between each by calling the thread::sleep function with a
    // Duration value of 1 second.
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
            thread::sleep(Duration::from_secs(1));
        }
    });

    // In the main thread, we’re not calling the recv function explicitly
    // anymore: instead, we’re treating rx as an iterator. For each value
    // received, we’re printing it. When the channel is closed, iteration will
    // end.
    for received in rx {
        println!("Got: {}", received);
    }

    // Creating Multiple Producers by Cloning the Transmitter
    // This time, before we create the first spawned thread, we call clone on
    // the transmitter. This will give us a new transmitter we can pass to the
    // first spawned thread. We pass the original transmitter to a second
    // spawned thread. This gives us two threads, each sending different
    // messages to the one receiver.
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
            thread::sleep(Duration::from_secs(1));
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
            thread::sleep(Duration::from_secs(1));
        }
    });

    // You might see the values in another order, depending on your system. This
    // is what makes concurrency interesting as well as difficult. If you
    // experiment with thread::sleep, giving it various values in the different
    // threads, each run will be more nondeterministic and create different
    // output each time.
    for received in rx {
        println!("Got: {}", received);
    }
}
