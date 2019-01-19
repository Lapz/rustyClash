extern crate rusty_royale;
#[macro_use]
extern crate serde_json;

use rusty_royale::{ClanSearch,SearchResponse,Clan};
use rusty_royale::Royale;

fn main() {
    let query = ClanSearch::new("boom baby");

    let mut royale = Royale::new("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6IjI4YTMxOGY3LTAwMDAtYTFlYi03ZmExLTJjNzQzM2M2Y2NhNSJ9.eyJpc3MiOiJzdXBlcmNlbGwiLCJhdWQiOiJzdXBlcmNlbGw6Z2FtZWFwaSIsImp0aSI6ImMyNzVkODY2LWVjNjgtNGI0Mi04YTNlLTEzYzBiNjI2YTBiZiIsImlhdCI6MTU0NzkxNzgxNSwic3ViIjoiZGV2ZWxvcGVyL2NmZGEwNDgwLWRiNjItMTk2Yi1iNjlmLWVlNzg5MWU1Y2I3YyIsInNjb3BlcyI6WyJyb3lhbGUiXSwibGltaXRzIjpbeyJ0aWVyIjoiZGV2ZWxvcGVyL3NpbHZlciIsInR5cGUiOiJ0aHJvdHRsaW5nIn0seyJjaWRycyI6WyI1LjcxLjE3OS41NSJdLCJ0eXBlIjoiY2xpZW50In1dfQ.yKnA2uCsFSCKxPEyMBNXj6VWrm86iaP6hMi0o11YwfwCHJvOtC41DcJR9c1vrrayN65GUMMNuu0Buc4NxV030A").unwrap();

    let mut resp = royale.search_clans(query).unwrap();

    println!("{:#?}", resp.json::<SearchResponse<Clan>>().unwrap())
}
