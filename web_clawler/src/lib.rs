use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use url::Url;
use log::info;
use serde::Serialize;
use thiserror::Error;

// We can use the macro on thiserror crate
#[derive(Error, Debug)]
pub enum GetLinksError {
    // automatically implentation of Display trait
    #[error("Failed to send a request")]
    SendRequest(#[source] reqwest::Error),
    #[error("Failed to read the response body")]
    ResponseBody(#[source] reqwest::Error),
    #[error("Failed to make the link URL absolute")]
    AbsolutizeUrl(#[source] url::ParseError),
}

pub struct LinkExtractor {
    client: Client,  // client is made by ClientBuilder
}

impl LinkExtractor {
    pub fn from_client(client: Client) -> Self {
        Self {
            client: client,
        }
    }

    /// Url list
    pub fn get_links(&self, url:Url)  -> Result<Vec<Url>, eyre::Report> {
        info!("GET \"{}\"", url);
        let response  = self.client.get(url).send().map_err(|e| GetLinksError::SendRequest(e))?;
        let base_url  = response.url().clone(); // clone object 
        let status    = response.status();
        let body      = response.text()?;
        let doc       = Document::from(body.as_str()); // create HTML Document Object
        let mut links = Vec::new();
        info!("Retrieved {} \"{}\"", status, base_url);

        for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
            match Url::parse(href) {
                Ok(url) => { 
                    links.push(url); 
                },
                Err(url::ParseError::RelativeUrlWithoutBase) => {
                    let url = base_url.join(href)?;
                    links.push(url);
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Ok(links)
    }
}
