use std::{sync::mpsc, thread};

pub fn run() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Hello from a spawn thread! {}", i);
        }
    });

    for i in 1..10 {
        println!("Hello from a main thread! {}", i);
    }
    
    // channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("Got: {}", received)


}