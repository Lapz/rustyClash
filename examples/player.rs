extern crate rusty_royale;
#[macro_use]
extern crate serde_json;

use rusty_royale::clans::{Clan, ClanSearch};
use rusty_royale::{Royale, Tag,Player};

fn main() {
    let mut royale = Royale::new("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6IjI4YTMxOGY3LTAwMDAtYTFlYi03ZmExLTJjNzQzM2M2Y2NhNSJ9.eyJpc3MiOiJzdXBlcmNlbGwiLCJhdWQiOiJzdXBlcmNlbGw6Z2FtZWFwaSIsImp0aSI6IjE3MDc3NmQwLTM1ZjAtNDQyYi1iZDUzLTkwMGUzNWU0MDc4MyIsImlhdCI6MTU0OTI4MDIzNSwic3ViIjoiZGV2ZWxvcGVyL2NmZGEwNDgwLWRiNjItMTk2Yi1iNjlmLWVlNzg5MWU1Y2I3YyIsInNjb3BlcyI6WyJyb3lhbGUiXSwibGltaXRzIjpbeyJ0aWVyIjoiZGV2ZWxvcGVyL3NpbHZlciIsInR5cGUiOiJ0aHJvdHRsaW5nIn0seyJjaWRycyI6WyIxNjEuMjMuNTcuMTE1Il0sInR5cGUiOiJjbGllbnQifV19.VzRRmLfsULnrjBWU1h1DmXRL6Gc6y0TgnKcB7rvzx2gRoB7SaHTnZnQNsbFpTBHdhx1YXBdV_D91nw_-gaGQwg").unwrap();

    let tag = Tag::new("#8Q820C0").unwrap();
    
    let player = royale
        .players()
        .player(&tag)
        .unwrap();

    let battle_log = royale.players().battlelog(&tag).unwrap();

    let upcoming_chests = royale.players().upcoming_chests(&tag).unwrap();



    println!("{:#?}", player);

  println!("{:#?}",battle_log);

    println!("{:#?}",upcoming_chests);
}
