fn main() {
    println!("Hello, world!");

    let mut s = "hello";
    println!("{}", s);
    s = "aaa";
    println!("{}", s);


    let mut t = String::from("hello");
    println!("{}", t);
    t = "aaa".to_string();
    t.push_str("hahaha");
    println!("{}", t);

    let t1 = t;
    println!("{}", t1);
    // println!("{}", t) //t 已经失效

    let t2 = &t1;
    println!("{}", t2);

    let mut t3 = t1.clone();
    t3.push_str(" t3333");
    println!("{}", t3);

    let t4 = String::from("t4444");
    takest4(t4);
    // println!("{}", t4); 报错 t4 已经失效

    let mut x = String::from("x111");
    changex(&mut x);
    println!("{}", x);

    // let reference_to_nothing = dangle();
}

fn takest4(t4 : String) {
    println!("{}", t4);
}

fn changex(x : &mut String) {
    x.push_str(" xxxxxx");
}

// fn str1() -> &str{
//     "a"
// }

fn str2() -> String {
    "b".to_string()
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

