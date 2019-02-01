use chrono::*;
use chttp::{Client, http::Request};
use json;
use std::env;
use std::io::prelude::*;

const BASE_URI: &'static str = "https://mercury.postlight.com/parser";


pub struct Mercury {
    client: Client,
    api_key: String,
}

impl Default for Mercury {
    fn default() -> Mercury {
        let api_key = env::var("MERCURY_API_KEY").expect("MERCURY_API_KEY not set");
        Mercury::new(api_key)
    }
}

impl Mercury {
    /// Create a new Mercury client.
    pub fn new<S: Into<String>>(api_key: S) -> Mercury {
        Mercury {
            client: Client::new().unwrap(),
            api_key: api_key.into(),
        }
    }

    /// Invoke the Mercury parser.
    pub fn parse(&self, url: &str) -> Option<Article> {
        let request = Request::get(format!("{}?url={}", BASE_URI, url))
            .header("x-api-key", self.api_key.clone())
            .body(())
            .unwrap();

        // Send the API request.
        let mut response = self.client.send(request).unwrap();

        // Read the response body.
        let mut response_body = String::new();
        if response.body_mut().read_to_string(&mut response_body).is_err() {
            return None;
        }

        // Read the HTTP response as JSON.
        let json = json::parse(&response_body).unwrap();

        Self::article_from_json(json)
    }

    fn article_from_json(mut json: json::JsonValue) -> Option<Article> {
        Some(Article {
            author: json["author"].take_string(),
            content: match json["content"].take_string() {
                Some(s) => s,
                None => return None,
            },
            date_published: json["date_published"].take_string().and_then(|s| {
                s.parse::<DateTime<UTC>>().ok()
            }),
            dek: json["dek"].take_string(),
            direction: match json["direction"].take_string() {
                Some(s) => s,
                None => return None,
            },
            domain: match json["domain"].take_string() {
                Some(s) => s,
                None => return None,
            },
            excerpt: match json["excerpt"].take_string() {
                Some(s) => s,
                None => return None,
            },
            lead_image_url: json["lead_image_url"].take_string(),
            next_page_url: json["next_page_url"].take_string(),
            rendered_pages: match json["rendered_pages"].as_u32() {
                Some(i) => i,
                None => return None,
            },
            title: match json["title"].take_string() {
                Some(s) => s,
                None => return None,
            },
            total_pages: match json["total_pages"].as_u32() {
                Some(i) => i,
                None => return None,
            },
            url: json["url"].take_string().unwrap(),
            word_count: match json["word_count"].as_u32() {
                Some(i) => i,
                None => return None,
            },
        })
    }
}


#[derive(Clone, Debug)]
pub struct Article {
    pub author: Option<String>,
    pub content: String,
    pub date_published: Option<DateTime<UTC>>,
    pub dek: Option<String>,
    pub direction: String,
    pub domain: String,
    pub excerpt: String,
    pub lead_image_url: Option<String>,
    pub next_page_url: Option<String>,
    pub rendered_pages: u32,
    pub title: String,
    pub total_pages: u32,
    pub url: String,
    pub word_count: u32,
}


mod test {
    use super::Mercury;


    #[test]
    fn test_parse() {
        let mercury = Mercury::default();
        let article = mercury.parse("https://trackchanges.postlight.com/building-awesome-cms-f034344d8ed");
        println!("{:?}", article);
    }
}
