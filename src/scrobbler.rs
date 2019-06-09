use config::types::Config;

use rustfm_scrobble::{Scrobble, Scrobbler};

use std::process;

macro_rules! scrobble {
    ($artist:expr, $track:expr, $album:expr) => {
        Scrobble::new($artist.to_string(), $track.to_string(), $album.to_string())
    };
}

pub struct ScrobbleClient {
    pub scrobbler: rustfm_scrobble::Scrobbler,
}

// TODO use authentication with session key
impl ScrobbleClient {
    pub fn new(configuration: &Config) -> ScrobbleClient {
        let api_key = configuration
            .lookup_str("lastfm.api_key")
            .expect("Missing api key");
        let api_secret = configuration
            .lookup_str("lastfm.api_secret")
            .expect("Missing api secret");
        let username = configuration
            .lookup_str("lastfm.username")
            .expect("Missing username");
        let password = configuration
            .lookup_str("lastfm.password")
            .expect("Missing password");

        let mut scrobbler = Scrobbler::new(api_key.to_string(), api_secret.to_string());
        match scrobbler.authenticate_with_password(username.to_string(), password.to_string()) {
            Ok(_) => {
                println!("Authenticated!");
            }
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        };

        ScrobbleClient {
            scrobbler,
        }
    }

    pub fn scrobble(&self, artist: &str, track: &str, album: &str) {
        let track = scrobble!(artist, track, album);
        match &self.scrobbler.scrobble(track) {
            Ok(_) => {
                println!("Scrobbled {}", artist);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    pub fn now_playing(&self, artist: &str, track: &str, album: &str) {
        let track = scrobble!(artist, track, album);
        match &self.scrobbler.now_playing(track) {
            Ok(_) => {
                println!("Now playing {}", artist);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
