mod clans;
mod royale;

const APIROOT: &'static str = "https://api.clashroyale.com/v1";

pub use crate::clans::{ClanSearch,SearchResponse,Clan,ClanMember,Location,Arena};
pub use crate::royale::Royale;
