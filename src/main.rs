extern crate reqwest;
extern crate select;
extern crate tokio;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

fn main() {
    hacker_news("https://stackoverflow.com/questions/tagged/rust");
}
#[tokio::main]
async fn hacker_news(url: &str) -> Result<(), reqwest::Error> {
    let resp = reqwest::get(url)
        .await?;
    // println!("body = {:?}", resp.text().await?);
    // assert!(resp.status().is_success());
    let document = Document::from(&*resp.text().await?);

    for node in document.find(Class("mln24")) {
        let question = node.find(Class("excerpt")).next().unwrap().text();
        let title_element = node.find(Class("question-hyperlink")).next().unwrap();
        let title = title_element.text();
        let question_link = title_element.attr("href").unwrap();

        println!("Question       => {}\nQuestion-link  => https://stackoverflow.com{}\nQuestion-title => {}",question,question_link,title);
        println!("\n-------------------------------------------------------------\n");
    }
    Ok(())
}