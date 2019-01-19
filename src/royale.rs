use crate::clans::ClanTag;

#[derive(Clone)]
pub struct Royale<'a> {
    key: &'a str,
}

impl<'a> Royale<'a> {
    pub fn new(key: &'a str) -> Self {
        Royale { key }
    }

    pub fn clans() -> () {
        unimplemented!()
    }


    pub fn clan(tag:ClanTag) {
        unimplemented!()
    }

}
