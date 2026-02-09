use std::env::args;
use std::sync::mpsc;
use std::time::Duration;
use trpl::{block_on, Either, Html};

// async fn page_title(url: &str) -> (&str,Option<String>) {
//     //asynchronously reads a pages title
//     let response = trpl::get(url).await;
//     let title_string = response.text().await;
//     let title = Html::parse(&title_string)
//         .select_first("title")
//         .map(|title| title.inner_html());
//     (url,title)
// }

fn main() {
    // let args: Vec<String> = args().collect();
    // trpl::block_on(async {
    //     let title_one = page_title(&args[1]);
    //     let title_two = page_title(&args[2]);
    //
    //     let (url,maybe_title) = match trpl::select(title_one,title_two).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };
    //
    //     println!("{} url returned first",url);
    //     match maybe_title {
    //         Some(title) => println!("Its page title was: '{title}'"),
    //         None => println!("It had no title."),
    //     }
    // })

    // trpl::block_on(async {
    //     let run_one = async {
    //         for i in 1..10  {
    //             println!("Printing {} from run_one",i);
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };
    //
    //     let handle_one = run_one.await;
    //
    //     let run_two = async {
    //         for i in 1..10  {
    //             println!("Printing {} from run_two",i);
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };
    //     let handle_two = run_two.await;
    // })
    trpl::block_on( async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}