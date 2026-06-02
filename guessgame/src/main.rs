use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("Guess the secret number = {}", secret_num);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input1: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered: {}, {}", input1, input.trim());

        match input1.cmp(&secret_num) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // let a = 5;
    // let b = 10;
    // println!("a=b={}", a+b);
}