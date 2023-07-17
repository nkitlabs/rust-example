use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn simple_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("[example_thread] hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // uncomment next line to see a new thread end before completing its task.
    handle.join().unwrap();

    for i in 1..5 {
        println!("[example_thread] hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn thread_with_move_ownership() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn passing_message_between_threads() {
    // multiple producers.
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
            println!("val is \"{}\"", val);
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
            println!("val is \"{}\"", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn share_state_over_threads() {
    // use atomic ref counter instead of normal rc.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn example_thread() {
    // example thread
    simple_thread();

    thread_with_move_ownership();

    passing_message_between_threads();

    share_state_over_threads();
}
