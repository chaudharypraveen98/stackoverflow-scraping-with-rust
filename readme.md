## Scraping Stackoverflow with Rust
It will extract the title, question link, answers count, view count and votes from stackoverflow depending on the tag parameter and count. This scraper is inspired from [Kadekillary Scarper](https://github.com/kadekillary/scraping-with-rust) with updated libraries and some more features added.

<img src="Rust Scraping.png">

#### Libraries Used
* [Reqwest](https://crates.io/crates/reqwest) 
    >An ergonomic, batteries-included HTTP Client for Rust.
* [Select](https://crates.io/crates/select)
    >A Rust library to extract useful data from HTML documents, suitable for web scraping
* [Clap](https://crates.io/crates/clap)
    >A simple-to-use, efficient, and full-featured library for parsing command line arguments and subcommands.

#### Features
* Simple and Fast
* Async get request
* Cli mode

#### How to run
1. Build the executable by `cargo build`
2. Run by `./target/debug/stackoverflow-scraping-with-rust -t <tag> -c <count>`<br>
    **< tag >** is the topic from which you want to scrape
    **< count >** is the number of posts/threads to be scraped. Note the maximum limit is *16*<br>
    Like this
    `./target/debug/stackoverflow-scraping-with-rust -t java -c 1`

#### Sample Output
<img src="sample_output.JPG">

