fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello world");
    let word = fist_word(&s);
    println!("{}", word);
    s.clear();
    // println!("{}", word);

    let s1 = String::from("hello world");
    let hello = &s1[0..5];
    let world = &s1[6..11];
    let a = &s1[..2];
    let b = &s1[6..];
    let c = &s1[..];
    // c = &s1[1..2];

    println!("{}, {}", hello, world);
    println!("{}, {}, {}", a, b, c);

    let d = &s1[1..];
    println!("{}",d);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}

fn fist_word(s: &String) -> &str {
    println!("{}", s);

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
