use std::env::args;
use trpl::{block_on, Either, Html};

async fn page_title(url: &str) -> (&str,Option<String>) {
    //asynchronously reads a pages title
    let response = trpl::get(url).await;
    let title_string = response.text().await;
    let title = Html::parse(&title_string)
        .select_first("title")
        .map(|title| title.inner_html());
    (url,title)
}

fn main() {
    let args: Vec<String> = args().collect();
    trpl::block_on(async {
        let title_one = page_title(&args[1]);
        let title_two = page_title(&args[2]);

        let (url,maybe_title) = match trpl::select(title_one,title_two).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{} url returned first",url);
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}