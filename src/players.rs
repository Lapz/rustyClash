use crate::Tag;
use serde::{Deserialize, Serialize};
use reqwest::{Client,Url};
use crate::util::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    tag: Tag,
    name: String,
    #[serde(rename = "expLevel")]
    exp_level: i32,
    trophies: i32,
    arena: Arena,
    #[serde(rename = "bestTrophies")]
    best_trophies: i32,
    wins: i32,
    losses: i32,
    #[serde(rename = "battleCount")]
    battle_count: i32,
    #[serde(rename = "threeCrownWins")]
    three_crown_wins: i32,
    #[serde(rename = "challengeCardsWon")]
    challenge_cards_won: i32,
    #[serde(rename = "challengeMaxWins")]
    challenge_max_wins: i32,
    #[serde(rename = "tournamentCardsWon")]
    tournament_cards_won: i32,
    #[serde(rename = "tournamentBattleCount")]
    tournament_battle_count: i32,
    role: String,
    donations: i32,
    #[serde(rename = "donationsReceived")]
    donations_received: i32,
    #[serde(rename = "totalDonations")]
    total_donations: i32,
    #[serde(rename = "warDayWins")]
    war_day_wins: i32,
    #[serde(rename = "clanCardsCollected")]
    clan_cards_collected: i32,
    clan: ClanBase,
    #[serde(rename = "leagueStatistics")]
    league_statistics: LeagueStatistics,
    achievements: Vec<Acheivement>,
    cards: Vec<Card>,
    #[serde(rename = "currentFavouriteCard")]
    current_favourite_card: FavouriteCard,
    badges:Vec<Badge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arena {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClanBase {
    tag: Tag,
    name: String,
    #[serde(rename = "badgeId")]
    badge_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Badge {
    name:String,
    level:Option<i32>,
    #[serde(rename = "maxLevel")]
    max_level:Option<i32>,
    progress:Option<i32>,
    target:Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueStatistics {
    #[serde(rename = "currentSeason")]
    current_season: SeasonStatistics,
    #[serde(rename = "previousSeason")]
    previous_season: Option<SeasonStatistics>,
    #[serde(rename = "bestSeason")]
    best_season: SeasonStatistics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonStatistics {
    id: Option<String>,
    trophies: i32,
    #[serde(rename = "bestTrophies")]
    best_trophies: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acheivement {
    name: String,
    stars: i32,
    value: i32,
    target: i32,
    info: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingChests {
    items:Vec<Chest>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chest {
    index:i32,
    name:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleLog {
    #[serde(rename = "type")]
    kind:String,
    #[serde(rename = "battleTime")]
    battle_time:String,
    arena:Arena,
    #[serde(rename = "gameMode")]
    game_mode:GameMode,
    #[serde(rename = "deckSelection")]
    deck_selection:String,
    team:Vec<BattleLogTeam>,
    opponent:Vec<BattleLogTeam>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleLogTeam {
    tag:Tag,
    name:String,
    #[serde(rename = "startingTrophies")]
    starting_trophies:Option<i32>,
    #[serde(rename = "trophyChange")]
    trophy_change:Option<i32>,
    crowns:i32,
    clan:Option<ClanBase>,
    cards:Vec<FavouriteCard>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GameMode {
    id:i32,
    name:String
}

pub struct PlayerApi<'a> {
   client: &'a mut Client,
}

impl<'a> PlayerApi<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Self { client }
    }

    pub fn player(&mut self,tag:&Tag) -> reqwest::Result<Player> {
        let url = format!("{}/players/{}", crate::APIROOT, tag);

        self.client.get(Url::parse(&url).unwrap()).send()?.json()
    }

    pub fn upcoming_chests(&mut self, tag: &Tag) -> reqwest::Result<UpcomingChests> {
        let url = format!("{}/players/{}/upcomingchests ", crate::APIROOT, tag);
        
        self.client.get(Url::parse(&url).unwrap()).send()?.json()
    }

    pub fn battlelog(&mut self, tag: &Tag) -> reqwest::Result<Vec<BattleLog>> {
        let url = format!("{}/players/{}/battlelog", crate::APIROOT, tag);

        self.client.get(Url::parse(&url).unwrap()).send()?.json()
    }


}