use serde::Serialize;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use reqwest::{Request,RequestBuilder,Method,IntoUrl};

#[derive(Serialize)]
pub struct Clan {
    tag: String,
    name: String,
    #[serde(rename = "badgeId")]
    badge_id: i32,
    #[serde(rename = "type")]
    kind: String,
    #[serde(rename = "clanScore")]
    clan_score: i32,
    #[serde(rename = "requiredTrophies")]
    required_trophies: i32,
    #[serde(rename = "donationsPerWeek")]
    donations_per_week: i32,
    #[serde(rename = "clanChestLevel")]
    clan_chest_level: i32,
    #[serde(rename = "clanChestMaxLevel")]
    clan_chest_max_level: i32,
    members: i32,
    location: Location,
    description: String,
    #[serde(rename = "clanChestStatus")]
    clan_chest_status: String,
    #[serde(rename = "clanChestMaxPoints")]
    clan_chest_points: i32,
    #[serde(rename = "membersList")]
    members_list: Vec<ClanMember>,
}

#[derive(Serialize)]
pub struct Location {
    id: i32,
    name: String,
    #[serde(rename = "isCountry")]
    is_country: bool,
    #[serde(rename = "countryCode")]
    country_code: String,
}

#[derive(Serialize)]
pub struct ClanMember {
    tag: String,
    name: String,
    #[serde(rename = "expLevel")]
    exp_level: i32,
    trophies: i32,
    arena: Arena,
    role: String,
    #[serde(rename = "clanRank")]
    clan_rank: i32,
    #[serde(rename = "reviousClanRank")]
    previous_clan_rank: i32,
    donations: i32,
    #[serde(rename = "donationsReceived")]
    donations_received: i32,
    #[serde(rename = "clanChestPoints")]
    clan_chest_points: i32,
}

#[derive(Serialize)]
pub struct Arena {
    id: i32,
    name: String,
}


pub struct ClanTag(String);

impl ClanTag {
    pub fn new(tag:&str) -> Result<ClanTag,()> {

        if tag.is_empty() {
            return Err(())
        }

        if &tag[0..1] != "#" {
            return Err(())
        }




        Ok(ClanTag(utf8_percent_encode(tag,DEFAULT_ENCODE_SET).collect() ))
    }
}


#[derive(Clone,Default)]
pub struct ClanRequest<'a> {
    name:&'a str,
    location_id:Option<i32>,
    min_members:Option<i32>,
    max_members:Option<i32>,
    min_score:Option<i32>,
    limit:Option<i32>,
    after:Option<i32>,
    before:Option<i32>,
}

impl <'a> ClanRequest <'a> {
    pub fn new(name:&'a str) -> Self {
        ClanRequest {
            name,
            ..Default::default()
        }
    }

    pub fn location(mut self,id:i32) -> Self {
        self.location_id = Some(id);
        self
    }

    pub fn min_members(mut self,n:i32) -> Self {
        self.min_members = Some(n);
        self
    }


    pub fn max_members(mut self,n:i32) -> Self {
        self.max_members = Some(n);
        self
    }

    pub fn min_score(mut self,n:i32) -> Self {
        self.min_score = Some(n);
        self
    }

    pub fn limit(mut self,n:i32) -> Self {
        self.limit = Some(n);
        self
    }


    pub fn after(mut self,n:i32) -> Self {
        self.after = Some(n);
        self
    }

    pub fn before(mut self,n:i32) -> Self {
        self.before = Some(n);
        self
    }

    pub fn build(self) -> Request {


        let url = Url::parse(&format!("{}/clans",crate::APIROOT)).unwrap();


        // match self {

        // }


        // let url = 
        // let builder = Request::new(Method::get,);
        unimplemented!()
    }
}