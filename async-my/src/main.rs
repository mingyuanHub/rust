use std::thread;
use trpl::{Either, Html};

fn main() {
    // learn1();
    // learn2();
    // learn3();
    // learn4();
    // learn5();
    learn6();
}

fn learn6() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send( i).unwrap();
            thread::sleep(std::time::Duration::from_secs(1))
        };
    });

    trpl::block_on(async {
        while let Some(value) = rx.recv().await {
            println!("learn6: {value}")
        }
    })
}

fn learn5() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("learn5 name1"),
            String::from("learn5 name2"),
            String::from("learn5 name3"),
            String::from("learn5 name4"),
        ];

        for val in vals {
            tx.send(val).unwrap();
        }

        while let Some(value) = rx.recv().await {
            println!("learn5: {value}")
        }
    })
}

fn learn4() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("learn4 name");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("{received}")
    });
}

fn learn3() {
    trpl::block_on(async{
        let async1 = async {
            for i in 21..30 {
                println!("{}", i);
                trpl::sleep(std::time::Duration::from_millis(1000)).await;
            }
        };

        let async2 = async {
            for i in 1..10 {
                println!("{}", i);
                trpl::sleep(std::time::Duration::from_millis(1000)).await;
            }
        };

        trpl::join(async1, async2).await;
    });
}
fn learn2() {
    trpl::block_on(async{
        trpl::spawn_task(async {
            for i in 21..30 {
                println!("{}", i);
                trpl::sleep(std::time::Duration::from_millis(1000)).await;
            }
        });

        for i in 1..10 {
            println!("{}", i);
            trpl::sleep(std::time::Duration::from_millis(1000)).await;
        }
    });
}

fn learn1() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}