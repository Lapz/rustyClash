use crate::clans::ClanSearch;
use crate::clans::ClanTag;
use reqwest::{self, header, Client, ClientBuilder, Response,Url};

#[derive(Clone)]
pub struct Royale<'a> {
    key: &'a str,
    client: Client,
}

impl<'a> Royale<'a> {
    pub fn new(key: &'a str) -> reqwest::Result<Self> {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", key)).unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Royale { key, client })
    }

    pub fn search_clans(&mut self, query: ClanSearch) -> reqwest::Result<Response> {
        let url = query.build();

        println!("{}",url);
        self.client.get(url).send()
    }

    pub fn clan(tag: ClanTag) {
        let url = format!("{}/clans/{:?}",crate::APIROOT,tag);
        let url = Url::parse(&url).unwrap()?;

    }
}
