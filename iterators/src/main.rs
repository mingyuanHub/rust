fn main() {
    println!("Hello, world!");

    let list: Vec<i32> = vec![1, 2, 3];

    let ite = list.iter();
    for i in ite {
        println!("{}", i);
    }

    let total: i32 = list.iter().sum();
    println!("{}", total);

    let mut it2 = list.iter();
    println!("{:?}", it2.next());

    let it3= list.iter();
    let list3:Vec<_> = it3.map(|x|x+1).collect();
    println!("{:?}", list3)
}
