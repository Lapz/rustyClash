use crate::util::SearchResponse;
use crate::util::Tag;
use crate::util::format_param;

use reqwest::{self, Client, Url};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Serialize, Deserialize)]
pub struct Clan {
    tag: Tag,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    id: i32,
    name: String,
    #[serde(rename = "isCountry")]
    is_country: bool,
    #[serde(rename = "countryCode")]
    country_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(rename = "previousClanRank")]
    previous_clan_rank: i32,
    donations: i32,
    #[serde(rename = "donationsReceived")]
    donations_received: i32,
    #[serde(rename = "clanChestPoints")]
    clan_chest_points: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arena {
    id: i32,
    name: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WarLog {
    items: Vec<ClanWar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClanWar {
    #[serde(rename = "seasonID")]
    season_id: Option<i32>,
    #[serde(rename = "createdDate")]
    created_date: String,
    participants: Vec<WarParticipant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWar {
    state: String,
    #[serde(rename = "warEndTime")]
    war_end_time: Option<String>,
    clan: CurrentWarClan,
    participants: Vec<WarParticipant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWarClan {
    tag: Tag,
    name: String,
    #[serde(rename = "badgeId")]
    badge_id: i32,
    #[serde(rename = "clanScore")]
    clan_score: i32,
    participants: i32,
    #[serde(rename = "battlesPlayed")]
    battles_played: i32,
    wins: i32,
    crowns: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClanBase {
    tag: Tag,
    name: String,
    #[serde(rename = "badgeId")]
    badge_id: i32,
}

pub struct ClanApi<'a> {
    client: &'a mut Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarParticipant {
    tag: Tag,
    name: String,
    #[serde(rename = "cardsEarned")]
    cards_earned: i32,
    #[serde(rename = "battlesPlayed")]
    battles_played: i32,
    wins: i32,
}

impl<'a> ClanApi<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Self { client }
    }

    pub fn search(&mut self, query: ClanSearch) -> reqwest::Result<SearchResponse<Clan>> {
        let url = query.build();
        self.client.get(url).send()?.json()
    }

    pub fn clan(&mut self, tag: Tag) -> reqwest::Result<Clan> {
        let url = format!("{}/clans/{}", crate::APIROOT, tag);
        self.client.get(Url::parse(&url).unwrap()).send()?.json()
    }

    pub fn members(&mut self, tag: Tag) -> reqwest::Result<SearchResponse<ClanMember>> {
        let url = format!("{}/clans/{}/members", crate::APIROOT, tag);
        self.client.get(Url::parse(&url).unwrap()).send()?.json()
    }

    pub fn warlog(&mut self, tag: Tag) -> reqwest::Result<SearchResponse<ClanWar>> {
        let url = format!("{}/clans/{}/warlog", crate::APIROOT, tag);
        self.client.get(Url::parse(&url).unwrap()).send()?.json()
    }

    pub fn current_war(&mut self, tag: Tag) -> reqwest::Result<CurrentWar> {
        let url = format!("{}/clans/{}/currentwar", crate::APIROOT, tag);
        self.client.get(Url::parse(&url).unwrap()).send()?.json()
    }
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

