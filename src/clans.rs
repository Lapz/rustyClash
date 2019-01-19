use reqwest::{self, Client, IntoUrl, Method, Request, RequestBuilder, Response, Url};
use serde::{Serialize,Deserialize};
use std::fmt::{self, Display};
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

#[derive(Debug,Serialize,Deserialize,)]
pub struct Clan {
    tag: ClanTag,
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
    description: Option<String>,
    #[serde(rename = "clanChestMaxPoints")]
    clan_chest_points: Option<i32>,
    #[serde(rename = "membersList")]
    members_list: Option<Vec<ClanMember>>,
}

#[derive(Debug,Serialize,Deserialize,)]
pub struct Location {
    id: i32,
    name: String,
    #[serde(rename = "isCountry")]
    is_country: bool,
    #[serde(rename = "countryCode")]
    country_code: Option<String>,
}

#[derive(Debug,Serialize,Deserialize,)]
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

#[derive(Debug,Serialize,Deserialize,)]
pub struct Arena {
    id: i32,
    name: String,
}

#[derive(Debug,Serialize,Deserialize,)]
pub struct ClanTag(String);

impl ClanTag {
    pub fn new(tag: &str) -> Result<ClanTag, ()> {
        if tag.is_empty() {
            return Err(());
        }

        if &tag[0..1] != "#" {
            return Err(());
        }

        Ok(ClanTag(
            utf8_percent_encode(tag, DEFAULT_ENCODE_SET).collect(),
        ))
    }
}

#[derive(Serialize, Clone, Default)]
pub struct ClanSearch<'a> {
    name: &'a str,
    #[serde(rename = "locationId")]
    location_id: Option<i32>,
    #[serde(rename = "minMembers")]
    min_members: Option<i32>,
    #[serde(rename = "maxMembers")]
    max_members: Option<i32>,
    #[serde(rename = "minScore")]
    min_score: Option<i32>,
    limit: Option<i32>,
    after: Option<i32>,
    before: Option<i32>,
}

#[derive(Debug,Serialize,Deserialize, Clone)]
pub struct SearchResponse<T:Serialize> {
    items:Vec<T>,
    #[serde(skip_deserializing)]
    paging: Paging
}

#[derive(Debug,Serialize, Clone,Default)]
pub struct Paging {
    cursors:Vec<()>
}

impl<'a> ClanSearch<'a> {
    pub fn new(name: &'a str) -> Self {
        ClanSearch {
            name,
            ..Default::default()
        }
    }

    pub fn location(mut self, id: i32) -> Self {
        self.location_id = Some(id);
        self
    }

    pub fn min_members(mut self, n: i32) -> Self {
        self.min_members = Some(n);
        self
    }

    pub fn max_members(mut self, n: i32) -> Self {
        self.max_members = Some(n);
        self
    }

    pub fn min_score(mut self, n: i32) -> Self {
        self.min_score = Some(n);
        self
    }

    pub fn limit(mut self, n: i32) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn after(mut self, n: i32) -> Self {
        self.after = Some(n);
        self
    }

    pub fn before(mut self, n: i32) -> Self {
        self.before = Some(n);
        self
    }

    #[doc(hidden)]
    pub(crate) fn build(self) -> Url {
        let mut url = format!("{}/clans?name={}", crate::APIROOT, self.name);

        if self.location_id.is_some() {
            url.push_str(&format_param("location_id", self.location_id.unwrap()))
        } else if self.min_members.is_some() {
            url.push_str(&format_param("minMembers", self.min_members.unwrap()))
        } else if self.max_members.is_some() {
            url.push_str(&format_param("maxMembers", self.max_members.unwrap()))
        } else if self.min_score.is_some() {
            url.push_str(&format_param("minScore", self.min_score.unwrap()))
        } else if self.limit.is_some() {
            url.push_str(&format_param("limit", self.limit.unwrap()))
        } else if self.after.is_some() {
            url.push_str(&format_param("after", self.after.unwrap()))
        } else if self.before.is_some() {
            url.push_str(&format_param("before", self.before.unwrap()))
        }

        Url::parse(&url).unwrap()
    }
}

fn format_param<T: Display>(key: &str, value: T) -> String {
    format!("{}={}", key, value)
}
