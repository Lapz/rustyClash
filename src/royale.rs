use crate::clans::ClanApi;
use crate::players::PlayerApi;
use failure::Error;
use reqwest::{self, header, Client};

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

    pub fn players(&mut self) -> PlayerApi {
        PlayerApi::new(&mut self.client)
    }
}
