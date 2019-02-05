use serde::{Deserialize,Serialize};
use reqwest::{Client,Response,Result,Url};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    name: String,
    id:i32,
    level: i32,
    #[serde(rename = "maxLevel")]
    max_level: i32,
    count: i32,
    #[serde(rename = "iconUrls")]
    icon_urls: IconUrl,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FavouriteCard {
    name:String,
    id:i32,
    #[serde(rename = "maxLevel")]
    max_level:i32,
    #[serde(rename = "iconUrls")]
    icon_urls: IconUrl,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IconUrl {
    medium: String,
}

pub struct CardApi<'a> {
   client: &'a mut Client,
}


impl <'a> CardApi<'a> {
    pub fn new(client:&'a mut Client) -> Self  {
        Self {
            client
        }
    }
    pub fn cards(&mut self) -> reqwest::Result<Vec<FavouriteCard>> {
        let url = format!("{}/cards ", crate::APIROOT);
        
        self.client.get(Url::parse(&url).unwrap()).send()?.json()
    }
}