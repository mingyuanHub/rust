
struct My {
    name: String,
}

fn main() {
    println!("Hello, world!");

    let my = My {
        name: "my".to_string(),
    };

    println!("my created, name={}", my.name);

    drop(my);

    println!("my dropped");

    // println!("my created, name={}", my.name);

}
