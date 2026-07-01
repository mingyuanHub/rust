fn main() {
    println!("Hello, world!");

    let smart_pointer = Box::new(5);

    println!("{}", smart_pointer);

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("{:?}", list);


    let x = String::from("xxx");
    let y = Box::new(&x);
    let z = &&x;

    assert_eq!("xxx", *y);   // *y 是 &String，和 "xxx" 比较 ✓（自动解引用）
    assert_eq!("xxx", **y);  // **y 是 String，也可以
    assert_eq!("xxx", **z);  // **y 是 String，也可以

}

// use crate::List::{Cons, Nil};

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }