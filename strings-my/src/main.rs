fn main() {
    println!("Hello, world!");


    let a = String::from("ABC");

    let b = String::from("DEF");

    let c = a + &b;

    println!("{}", c);

    let d = format!("{}{}", b, c);

    println!("{}", d);


    panic!("777777777");

}
