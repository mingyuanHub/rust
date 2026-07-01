use std::thread;

fn main() {


    let closure1 = |x| x+5;
    let x = closure1(5);
    println!("{}", x);

    let mut list1 = vec![1, 2, 3];
    println!("{:?}", list1);
    let mut closure2 = || list1.push(4);
    closure2();
    println!("{:?}", list1);


    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();




    let  store = Inventory {
        shirt: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
    };

    let user1 = Some(ShirtColor::Blue);
    let color = store.giveaway(user1);
    println!("The user1 gets {:?}", color);

    let user2 = None;
    let color = store.giveaway(user2);
    println!("The user2 gets {:?}", color)
}

#[derive(Debug)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirt: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_per: Option<ShirtColor>) -> ShirtColor {
        user_per.unwrap_or_else(|| self.get_shirt())
    }

    fn get_shirt(&self) -> ShirtColor {
        let mut red_num = 0;
        let mut clue_num = 0;

       for color in &self.shirt {
           match color {
               ShirtColor::Red => red_num += 1,
               ShirtColor::Blue => clue_num += 1
           }
       }

        if red_num > clue_num {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}