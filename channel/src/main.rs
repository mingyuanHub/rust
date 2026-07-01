use std::{sync, thread};

fn main() {
    println!("Hello, world!");

    let (tx, rx) = sync::mpsc::channel();

    let tx2 = tx.clone();
    thread::spawn(move || {
        let hi = String::from("hiiiii");
        tx.send(hi).unwrap();

        let st2 = String::from("st2");
        tx.send(st2).unwrap();

        let st3 = String::from("st3");
        tx.send(st3).unwrap();

        // let v1 = vec![1,2,3];
        // tx.send(v1).unwrap();
        // println!("Sent a message!, {}", hi); //value borrowed here after move
    });

    thread::spawn(move || {
        let hi = String::from("hiiiii22222");
        tx2.send(hi).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {:?}", received);

    let received = rx.recv().unwrap();
    println!("Got: {:?}", received);

    for rec in rx {
        println!("Got: {:?}", rec)
    }
}
