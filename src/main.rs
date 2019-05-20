use config::reader;

use mongodb::{bson, doc, Bson};

use std::path::Path;
use std::{thread, time};

mod mongo;
mod scrobbler;

fn main() {
    let configuration = reader::from_file(Path::new("app.conf")).expect("Failed to load app.conf");
    let db = mongo::MongoClient::new(&configuration);
    let scrobbler = scrobbler::ScrobbleClient::new(&configuration);

    // TODO use change streams
    let doc2 = doc! {"scrobbled": { "$ne": false }};
    let cursor = db.coll.find(Some(doc2.clone()), None).unwrap();
    for result in cursor {
        let doc = result.expect("Received network error during cursor operations");
        if let Some(&Bson::String(ref artist)) = doc.get("artist") {
            if let Some(&Bson::String(ref album)) = doc.get("album") {
                if let Some(&Bson::String(ref track)) = doc.get("title") {
                    if let Some(&Bson::FloatingPoint(ref duration)) = doc.get("length_ms") {
                        scrobbler.now_playing(artist, track, album);

                        let duration = time::Duration::from_millis(*duration as u64);
                        thread::sleep(duration);

                        scrobbler.scrobble(artist, track, album);

                        let update = doc! { "$set": { "scrobbled": true } };
                        db.coll
                            .update_one(doc.clone(), update, None)
                            .expect("Failed to update document.");
                    }
                }
            }
        }
    }
}
