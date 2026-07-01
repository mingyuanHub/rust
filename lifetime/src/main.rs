fn main() {

    let a = 5;
    let b = 6;
    println!("{} + {} = {}", a, b, add(a, b));


    let str1 = "Hello";
    let str2 = "World1";

    let mut longest = get_longest(str1, str2);
    println!("The longest string is {}", longest);


    {
        let str3 = "World22";
        longest = get_longest(str1, str3);
        // println!("The longest string is {}", longest);
    }
    println!("The longest string is {}", longest);

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}