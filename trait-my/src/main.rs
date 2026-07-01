fn main() {
    println!("Hello, world!");

    let car = Car {
        name: String::from("red car"),
    };

    println!("{}", car.print_str());


    let truck = Truck {
        name: String::from("blue truck"),
    };
    println!("{}", truck.print_str());


    let car1 = return_print();
    println!("{}", car1.print_str());
}

pub trait PrintStr {
    fn print_str(&self) -> String;
}

struct Car {
    name: String,
}

impl PrintStr for Car {
    fn print_str(&self) -> String {
        format!("I am a car named {}", self.name)
    }
}


struct Truck {
    name: String,
}

impl PrintStr for Truck {
    fn print_str(&self) -> String {
        format!("I am a truck named {}", self.name)
    }
}

fn return_print() -> impl PrintStr {
    Car {
        name: String::from("white car"),
    }
}
