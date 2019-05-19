use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use rustfm_scrobble::{Scrobbler, Scrobble};

static mut SCROBBLER: Option<rustfm_scrobble::Scrobbler> = None;

macro_rules! scrobble {
    ($artist:expr, $track:expr, $album:expr) => (
        Scrobble::new($artist.to_string(), $track.to_string(), $album.to_string())
    )
}

// TODO use authentication with session key
fn auth_scrobbler() {
    let api_key = "a".to_string();
    let api_secret = "b".to_string();
    let username = "ruinedmachine".to_string();
    let password = "c".to_string();

    unsafe {
        let mut scrobbler = Scrobbler::new(api_key, api_secret);
        match scrobbler.authenticate(username, password) {
            Ok(_) => { println!("Authenticated!"); }
            Err(e) => { println!("{}", e); }
        };
        SCROBBLER = Some(scrobbler);
    }
}

// fn adsfasdf(artist: String, track: String, album: String) {
//     let sc = scrobbler.ok()
//     let track_one = scrobble!(artist, track, album);
//     match sc.now_playing(track_one) {
//         Ok(_) => { println!("Sent now playing! "); }
//         Err(e) => { println!("{}", e); }
//     }

//     let track_two = scrobble!("Los Campesinos!", "The Time Before the Last", "No Blues");
//     match sc.scrobble(track_two) {
//         Ok(_) => { println!("Sent scrobble!"); }
//         Err(e) => { println!("{}", e); }
//     }

//     // let track_three = scrobble!("Los Campesinos!", "Selling Rope", "No Blues");
//     // match scrobbler.now_playing(track_three) {
//     //     Ok(_) => { println!("Sent now playing! "); }
//     //     Err(e) => { println!("{}", e); }
//     // }
// }

fn main() {
    auth_scrobbler();
    let client = Client::connect("localhost", 27017).expect("Failed to start Mongo");
    let coll = client.db("rust").collection("scrobbles");

    let doc = doc!{"scrobbled": { "$ne": false }};
    let mut cursor = coll.find(Some(doc.clone()), None).ok().expect("Failed to execute find");

    for result in cursor {
        if let Ok(item) = result {
         if let Some(&Bson::String(ref title)) = item.get("title") {
             println!("title: {}", title);
         }
     }
    }

}
