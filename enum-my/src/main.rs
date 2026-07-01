
#[derive(Debug)]
enum  MY {
    A,
    B,
    C,
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", MY::A);

    get_my(MY::B);

    let a = match_my(MY::A);
    println!("{}", a);

    let c = match_my(MY::C);
    println!("{}", c);
}

fn get_my(m: MY) {
    println!("{:?}", m)
}

fn match_my(my: MY) -> String {
    match my {
        MY::A => "A".to_string(),
        MY::B => "B".to_string(),
        MY::C => "C".to_string(),
    }
}