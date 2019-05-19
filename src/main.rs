use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use rustfm_scrobble::{Scrobbler, Scrobble};

use std::thread;

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

fn scrobble(artist: &str, track: &str, album: &str) {
    let track = scrobble!(artist, track, album);
    unsafe {
    	match &SCROBBLER {
    		Some(scrobbler) => 
    			match scrobbler.scrobble(track) {
			        Ok(_) => { println!("Scrobbled {}", artist); }
			        Err(e) => { println!("{}", e); }
			    },
			None => {},
    		}
    }
}

fn now_playing(artist: &str, track: &str, album: &str) {
    let track = scrobble!(artist, track, album);
    unsafe {
    	match &SCROBBLER {
    		Some(scrobbler) => 
    			match scrobbler.now_playing(track) {
			        Ok(_) => { println!("Now playing {}", artist); }
			        Err(e) => { println!("{}", e); }
			    },
			None => {},
    		}
    }
}

fn main() {
    auth_scrobbler();
    let client = Client::connect("localhost", 27017).expect("Failed to start Mongo");
    let coll = client.db("rust").collection("scrobbles");

    let doc = doc!{"scrobbled": { "$ne": false }};
    let cursor = coll.find(Some(doc.clone()), None).ok().expect("Failed to execute find");

    for result in cursor {
        if let Ok(item) = result {
        	if let Some(&Bson::String(ref artist)) = item.get("artist") {
        		if let Some(&Bson::String(ref album)) = item.get("album") {
        			if let Some(&Bson::String(ref track)) = item.get("title") {
        				if let Some(&Bson::FloatingPoint(ref duration)) = item.get("length_ms") {
	        				now_playing(artist, track, album);
        					thread::sleep_ms(*duration as u32);
	            			scrobble(artist, track, album);
     					}		
    				}
    			}
			}
		}
	}
}