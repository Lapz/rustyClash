use crate::clans::*;
use failure::Error;
use reqwest::{self, header, Client, ClientBuilder, Response, Url};

pub type RoyalError<T> = Result<T, Error>;

#[derive(Clone)]
pub struct Royale<'a> {
    key: &'a str,
    client: Client,
}

impl<'a> Royale<'a> {
    pub fn new(key: &'a str) -> RoyalError<Self> {
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

    pub fn clans(&mut self) -> ClanApi {
        ClanApi::new(&mut self.client)
    }
}
