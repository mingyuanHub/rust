fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    const AB_C: i32 = 1;

    println!("The value of AB_C is: {AB_C}");

    let t = 1;
    let t = t +1;
    println!("The value of t is: {t}");

    let space = "     ";
    let space = space.len();
    println!("The value of space is: {space}");

    let space2 = "   ";
    let space2 = space2.len();
    println!("The value of space2 is: {space2}");


    let ga: i32 = "42".parse().expect("Not a number!");
    println!("The value of ga is: {ga}");


    println!("The value of space2 is: {space2}");


    let _b = 2.3;
    // println!("The value of b is: {b}");
    let b:f32 = 2.2;
    println!("The value of b is: {b}");

    let c = true;
    println!("The value of c is: {c}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let d = tup.0;

    println!("The value of d is: {d}");

    let s :[i32; 5] = [0,1,2,3,4];
    println!("The value of s is: {s:?}");
    println!("The value of s is: {}", s[4]);

    for item in s {
        println!("{item}");
    }
    println!("The value of s is: {s:?}");

    for (index, item) in s.iter().enumerate() {
        println!("{index}: {item}");
    }

    let _a = "a".to_string() ;

    let y: [String;2] = ["a".to_string(), "b".to_string()];

    println!("The value of y is: {y:?}");

    for s in y.iter() {
        println!("{s}");
    }

    println!("The value of y is: {y:?}")
}
