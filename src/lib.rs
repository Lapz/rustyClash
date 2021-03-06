pub mod clans;
pub mod players;
mod royale;
pub mod cards;
pub mod util;
pub mod tournaments;

const APIROOT: &str = "https://api.clashroyale.com/v1";

pub use crate::clans::{Arena, Clan, ClanMember, ClanSearch, Location};
pub use crate::royale::Royale;
pub use util::Tag;
pub use players::{Player};
