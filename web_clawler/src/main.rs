use std::env;
use url::Url;
use web_clawler::LinkExtractor;
use reqwest::blocking::ClientBuilder;

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

    let links = extractor.get_links(url)?;

    for link in links.iter() {
        println!("{}", link);
    }

    Ok(())
}
