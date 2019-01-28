pub mod clans;
pub mod players;
mod royale;
pub mod util;

const APIROOT: &str = "https://api.clashroyale.com/v1";

pub use crate::clans::{Arena, Clan, ClanMember, ClanSearch, Location};
pub use crate::royale::Royale;
pub use util::Tag;
