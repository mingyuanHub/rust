fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn larget_t<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let mut number_list2 = vec![102, 34, 600, 89, 54, 2, 43, 8];
    number_list2.push(888);

    let result = largest(&number_list2);
    println!("The largest number is {result}");

    let number_list3 = [1,2,34,5,6];
    let result = largest(&number_list3);
    println!("The largest number is {result}");

    let char_list = ['a', 'b', 'c', 'r'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    test();

}


#[derive(Debug)]
struct Point<T> {
    x: i32,
    y: T
}

impl<T: Into<i32>> Point<T> {

    fn get(&self) -> &i32 {
        &self.x
    }

    fn move1(self) -> i32 {
        self.x + self.y.into()
    }
}
fn test() {
    let p1 = Point { x: 5, y: 10 };
    println!("p1:{:?}, {}, {}, {}", p1, p1.x, p1.y, p1.get( ));
    println!("{}", p1.move1());

}