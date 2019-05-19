// use rustfm_scrobble::{Scrobbler, Scrobble};

// macro_rules! scrobble {
//     ($artist:expr, $track:expr, $album:expr) => (
//         Scrobble::new($artist.to_string(), $track.to_string(), $album.to_string())
//     )
// }


// fn scrobble() {
//     let api_key = "a".to_string();
//     let api_secret = "b".to_string();
//     let username = "ruinedmachine".to_string();
//     let password = "c".to_string();
// 
//     let mut scrobbler = Scrobbler::new(api_key, api_secret);

//     match scrobbler.authenticate(username, password) {
//         Ok(_) => { println!("Authenticated!"); }
//         Err(e) => { println!("{}", e); }
//     };

//     let track_one = scrobble!("Los Campesinos!", "As Lucerne / The Low", "No Blues");
//     match scrobbler.now_playing(track_one) {
//         Ok(_) => { println!("Sent now playing! "); }
//         Err(e) => { println!("{}", e); }
//     }

//     let track_two = scrobble!("Los Campesinos!", "The Time Before the Last", "No Blues");
//     match scrobbler.scrobble(track_two) {
//         Ok(_) => { println!("Sent scrobble!"); }
//         Err(e) => { println!("{}", e); }
//     }

//     // let track_three = scrobble!("Los Campesinos!", "Selling Rope", "No Blues");
//     // match scrobbler.now_playing(track_three) {
//     //     Ok(_) => { println!("Sent now playing! "); }
//     //     Err(e) => { println!("{}", e); }
//     // }
// }