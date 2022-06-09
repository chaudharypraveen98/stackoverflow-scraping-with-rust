extern crate clap;
extern crate reqwest;
extern crate select;
extern crate tokio;

use clap::{App, Arg};
use rand::seq::SliceRandom;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

fn main() {
    let matches = App::new("StackOverflow Scraper")
        .version("1.0")
        .author("Praveen Chaudhary <chaudharypraveen98@gmail.com>")
        .about("It will scrape questions from stackoverflow depending on the tag.")
        .arg(
            Arg::with_name("tag")
                .short("t")
                .long("tag")
                .takes_value(true)
                .help("takes tag and scrape according to this"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .takes_value(true)
                .help("gives n count of posts"),
        )
        .get_matches();

    if matches.is_present("tag") && matches.is_present("count") {
        let url = format!(
            "https://stackoverflow.com/questions/tagged/{}?tab=Votes",
            matches.value_of("tag").unwrap()
        );
        let count: i32 = matches.value_of("count").unwrap().parse::<i32>().unwrap();
        hacker_news(&url, count as usize);
    } else if matches.is_present("tag") {
        let url = format!(
            "https://stackoverflow.com/questions/tagged/{}?tab=Votes",
            matches.value_of("tag").unwrap()
        );
        hacker_news(&url, 16);
    } else if matches.is_present("count") {
        let url = get_random_url();
        let count: i32 = matches.value_of("count").unwrap().parse::<i32>().unwrap();
        hacker_news(&url, count as usize);
    } else {
        let url = get_random_url();
        hacker_news(&url, 16);
    }
}

#[tokio::main]
async fn hacker_news(url: &str, count: usize) -> Result<(), reqwest::Error> {
    let resp = reqwest::get(url).await?;
    // println!("body = {:?}", resp.text().await?);
    // assert!(resp.status().is_success());
    let document = Document::from(&*resp.text().await?);

    for node in document.select(Class("s-post-summary")).take(count) {
        let question = node
            .select(Class("s-post-summary--content-excerpt"))
            .next()
            .unwrap()
            .text();
        let title_element = node
            .select(Class("s-post-summary--content-title").child(Name("a")))
            .next()
            .unwrap();
        let title = title_element.text();
        let question_link = title_element.attr("href").unwrap();
        let stats = node
            .select(Class("s-post-summary--stats-item-number"))
            .map(|stat| stat.text())
            .collect::<Vec<_>>();
        let votes = &stats[0];
        let answer = &stats[1];
        let views = &stats[2];
        let tags = node
            .select(Class("post-tag"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();
        println!("Question       => {}", question);
        println!(
            "Question-link  => https://stackoverflow.com{}",
            question_link
        );
        println!("Question-title => {}", title);
        println!("Votes          => {}", votes);
        println!("Views          => {}", views);
        println!("Tags           => {}", tags.join(" ,"));
        println!("Answers        => {}", answer);
        println!("-------------------------------------------------------------\n");
    }
    Ok(())
}

// Getting random tag
fn get_random_url() -> String {
    let default_tags = vec!["python", "rust", "c#", "android", "html", "javascript"];
    let random_tag = default_tags.choose(&mut rand::thread_rng()).unwrap();
    let url = format!(
        "https://stackoverflow.com/questions/tagged/{}?tab=Votes",
        random_tag
    );
    url.to_string()
}
