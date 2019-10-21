use crate::util::SearchResponse;
use crate::util::Tag;
use crate::util::format_param;

use reqwest::{self, Client, Url};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug,Deserialize, Serialize)]
pub struct Tournaments {
    items:Vec<Tournament>, // to do change
    startedTime:String,
    members_list:Vec<Member>
}

#[derive(Debug,Deserialize, Serialize)]
pub struct Tournament {
    tag:String,
    #[serde(rename="type")]
    kind:String,
    status:String,
    #[serde(rename="creatorTag")]
    creator_tag:String,
    name:String,
    description:String,
    capacity:i32,
    #[serde(rename="maxCapacity")]
    max_capacity:i32,
    #[serde(rename="preparationDuration")]
    preparation_duration:i32,
    #[serde(rename="createdTime")]
    created_time:i32
} 

#[derive(Debug,Deserialize, Serialize)]
pub struct Member {
    tag:String,
    name:String,
    score:i32,
    rank:i32,
    clan:crate::clans::ClanBase

}


pub struct TournamentApi<'a> {
   client: &'a mut Client,
}


impl <'a> TournamentApi<'a> {
    pub fn new(client:&'a mut Client) -> Self  {
        Self {
            client
        }
    }

    pub fn tournaments(&mut self, tag:&Tag) -> reqwest::Result<Tournament>{
        let url = format!("{}/tournaments/{}", crate::APIROOT,tag);
        unimplemented!()
    }

}