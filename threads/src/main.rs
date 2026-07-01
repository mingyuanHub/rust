use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1))
    }

   let _ = handle1.join().unwrap();

    
    // --------------
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let v3 = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
       println!("thread1 {:?}", v1);
       println!("thread2 {:?}", v2)
    });

    handle2.join().unwrap();

    println!("main thread {:?}", v3);
}
