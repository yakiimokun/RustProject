pub mod crawler;

use std::env;
use url::Url;
use web_crawler::LinkExtractor;
use web_crawler::crawler::Crawler;
use reqwest::blocking::ClientBuilder;
use std::time::Duration;

// eyre::Result<T> means Box<dyn std::error::Error>
// Box <dyn Trait> is generalized std::error::Error 
fn main() -> eyre::Result<()> {
    env_logger::init(); // Event Log initialization

    let url = env::args()
                .nth(1) // first argument
                .unwrap_or("https://www.rust-lang.org".to_owned()); // covert &str to String
    let url = Url::parse(&url)?;
    
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);

    let crawler = Crawler::new(&extractor, url);
    let wait = Duration::from_millis(100);

    for url in crawler.take(10) {
        println!("{}", url);
        std::thread::sleep(wait.clone());
    }

    Ok(())
}
