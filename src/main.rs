use config::reader;
use config::types::Config;

use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson};
use mongodb::{Client, ThreadedClient};

use rustfm_scrobble::{Scrobble, Scrobbler};

use std::path::Path;
use std::{process,thread};

static mut SCROBBLER: Option<rustfm_scrobble::Scrobbler> = None;

macro_rules! scrobble {
    ($artist:expr, $track:expr, $album:expr) => {
        Scrobble::new($artist.to_string(), $track.to_string(), $album.to_string())
    };
}

// TODO use authentication with session key
// TODO macro
fn auth_scrobbler(configuration: &Config) {
    let api_key = configuration.lookup_str("lastfm.api_key").unwrap();
    let api_secret = configuration.lookup_str("lastfm.api_secret").unwrap();
    let username = configuration.lookup_str("lastfm.username").unwrap();
    let password = configuration.lookup_str("lastfm.password").unwrap();

    unsafe {
        let mut scrobbler = Scrobbler::new(api_key.to_string(), api_secret.to_string());
        match scrobbler.authenticate(username.to_string(), password.to_string()) {
            Ok(_) => {
                println!("Authenticated!");
            }
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        };
        SCROBBLER = Some(scrobbler);
    }
}

fn scrobble(artist: &str, track: &str, album: &str) {
    let track = scrobble!(artist, track, album);
    unsafe {
        match &SCROBBLER {
            Some(scrobbler) => match scrobbler.scrobble(track) {
                Ok(_) => {
                    println!("Scrobbled {}", artist);
                }
                Err(e) => {
                    println!("{}", e);
                }
            },
            None => {}
        }
    }
}

fn now_playing(artist: &str, track: &str, album: &str) {
    let track = scrobble!(artist, track, album);
    unsafe {
        match &SCROBBLER {
            Some(scrobbler) => match scrobbler.now_playing(track) {
                Ok(_) => {
                    println!("Now playing {}", artist);
                }
                Err(e) => {
                    println!("{}", e);
                }
            },
            None => {}
        }
    }
}

fn main() {
    let configuration = reader::from_file(Path::new("app.conf"))
        .expect("Failed to load configs from file");

    auth_scrobbler(&configuration);

    let host = configuration.lookup_str("mongo.host").unwrap();
    let port = configuration.lookup_integer32("mongo.port").unwrap();
    let db = configuration.lookup_str("mongo.db").unwrap();
    let collection = configuration.lookup_str("mongo.collection").unwrap();

    let client = Client::connect(host, port as u16).expect("Failed to connect to Mongo");
    let coll = client.db(db).collection(collection);

    let doc = doc! {"scrobbled": { "$ne": false }};
    let cursor = coll
        .find(Some(doc.clone()), None)
        .ok()
        .expect("Failed to execute find");

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
