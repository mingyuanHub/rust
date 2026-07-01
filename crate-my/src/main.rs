mod my;

mod test;

fn main() {
    println!("Hello, world!");

    my::foo();

    test::test_name();

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(8);

    println!("Vector: {:?}", v);

    let v1 = v[2];
    println!("Vector1: {:?}, {:?}", v1, v[3]);
}
