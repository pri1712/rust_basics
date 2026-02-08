use std::env::args;
use trpl::{block_on, Html};

async fn page_title(url: &str) -> Option<String>{
    //asynchronously reads a pages title
    let response = trpl::get(url).await;
    let title_string = response.text().await;
    Html::parse(&title_string)
        .select_first("title")
        .map(|title| title.inner_html())

}

fn main() {
    let args: Vec<String> = args().collect();
    trpl::block_on(
        async {
            let url = &args[1];
            match page_title(url).await {
                Some(title) => println!("{}", title),
                None => println!("No title found"),
            }
        }
    );
}