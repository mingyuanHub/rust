#[derive(Debug)]
struct User {
    active: bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

impl User {

    fn new() -> Self {
        Self {
            active: true,
            username: String::from("someusername123"),
            email: String::from("<EMAIL>"),
            sign_in_count: 1,
        }
    }
    fn pr(&self) {
        println!("00000000 {:?}", self.email);
        // 1
    }

    fn pr1(&self) {
        println!("111111111 {:?}", self.email);
        // 1
    }

    fn pr2() {
        println!("22222222")
    }
}

// #[derive(Debug)]
// struct User1 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email:String::from("<EMAIL>"),
        username:String::from("someusername123"),
        active:true,
        sign_in_count:1,
    };

    user1.email = String::from("my.com");

    println!("{:#?}", user1);
    println!("{},{},{},{}", user1.email, user1.username, user1.active, user1.sign_in_count);
    // println!("{}", user1.pr());
    user1.pr();
    user1.pr1();

    let user1 = User::new();
    user1.pr1();

    User::pr2();


    let user2 = build_user(String::from("123.com"), String::from("someusername123"));
    println!("{:#?}", user2);

    let user3 = User {
        email:String::from("456.com"),
        username:String::from("someusername456"),
        ..user2
    };
    println!("{:#?}", user3);

    // let user4 = User {
    //     active: true,
    //     username: "someusername123",
    //     email: "someone@example.com",
    //     sign_in_count: 1,
    // };
    // println!("{:#?}", user4);
}

fn build_user(email:String, username:String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
