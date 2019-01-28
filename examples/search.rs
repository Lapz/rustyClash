extern crate rusty_royale;
#[macro_use]
extern crate serde_json;

use rusty_royale::Royale;
use rusty_royale::{Clan, ClanSearch, ClanTag, SearchResponse};

fn main() {
    
    let mut royale = Royale::new("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6IjI4YTMxOGY3LTAwMDAtYTFlYi03ZmExLTJjNzQzM2M2Y2NhNSJ9.eyJpc3MiOiJzdXBlcmNlbGwiLCJhdWQiOiJzdXBlcmNlbGw6Z2FtZWFwaSIsImp0aSI6ImU0YzdlM2ViLTQzOTUtNDVkMi1iMmRiLTM0NGE2ODZlMzkyNSIsImlhdCI6MTU0ODY3OTA0Miwic3ViIjoiZGV2ZWxvcGVyL2NmZGEwNDgwLWRiNjItMTk2Yi1iNjlmLWVlNzg5MWU1Y2I3YyIsInNjb3BlcyI6WyJyb3lhbGUiXSwibGltaXRzIjpbeyJ0aWVyIjoiZGV2ZWxvcGVyL3NpbHZlciIsInR5cGUiOiJ0aHJvdHRsaW5nIn0seyJjaWRycyI6WyI1LjcxLjE3OS41NSIsIjE2MS4yMy41Ni4yMTciXSwidHlwZSI6ImNsaWVudCJ9XX0.rOydgmNrD8UatM5gu9BkPEyNaQYdGd-46ijlxtOjw9vkkofafJcfYnMnQatwsg2IHmeOFlCKhWYLFiKeJb6NDQ").unwrap();

    let resp = royale
        .clans()
        .current_war(ClanTag::new("#8VG2C9CJ").unwrap())
        .unwrap();

    println!("{:#?}", resp);
}
