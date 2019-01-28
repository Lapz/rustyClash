pub mod clans;
mod royale;

const APIROOT:&str = "https://api.clashroyale.com/v1";

pub use crate::clans::{Arena, Clan, ClanMember, ClanSearch, Tag, Location};
pub use crate::royale::Royale;
use serde::{Serialize,Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResponse<T: Serialize> {
    items: Vec<T>,
    #[serde(skip_deserializing)]
    paging: Paging,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Paging {
    cursors: Vec<()>,
}