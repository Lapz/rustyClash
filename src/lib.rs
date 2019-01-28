mod clans;
mod royale;

const APIROOT: &'static str = "https://api.clashroyale.com/v1";

pub use crate::clans::{Arena, Clan, ClanMember, ClanSearch, ClanTag, Location, SearchResponse};
pub use crate::royale::Royale;
