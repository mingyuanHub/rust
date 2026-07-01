
use std::{env, fs};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::new(&args);

    let context = fs::read_to_string("files/".to_string() + &config.file_name).expect("fail to open file");

    dbg!(context);
    dbg!(&config.others);

    let config = Config::build(&args).unwrap_or_else( |err| {
        println!("error: {}", err);
        panic!("{}", err)
    });
    dbg!(&config);
}
 #[derive(Debug)]
struct Config {
    file_name: String,
    others: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let arg0 = &args[0];
        let arg1 = args[1].clone();
        let arg2 = &args[2];

        dbg!(&arg0);
        dbg!(&arg1);
        dbg!(&arg2);

        // let (filename, _) = {
        //     (arg1, arg2)
        // };

        Config{
            file_name: arg1,
            others: arg2.to_string(),
        }
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 || args.len() > 3 {
            return Err("error arguments number");
        }

        Ok(Config{
            file_name: args[1].clone(),
            others: args[2].clone(),
        })
    }

}
