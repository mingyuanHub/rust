fn main() {
    println!("Hello, world!");

    new_func();

    let s_a = 3;
    println!("{s_a}");

    func2(5);
    func3(5, "ssss");

    let a = {
        let a = 5;
        a+ 1
    };

    println!("{a}");

    println!("{}", func4(5));

    let mut c = if 1==1 {
        3
    } else {
        4
    };

    println!("{c}");

    c +=1;
    println!("{c}")
}

fn new_func() {
    println!("This is a new function!")
}

fn func2(a:i32) {
    println!("value = {}", a*5)
}

fn func3(a:i32, b: &str) {
    println!("value = {}, b={b}", a*5)
}

fn func4(a:i32) -> i32 {
    // return a + 5;
    if true {
        a + 4
    } else {
        a + 3
    }
    // a+5
}