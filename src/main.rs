use std::{thread, time};
use std::path::Path;

use config::reader;
use docopt::Docopt;
use id3::Tag;
use serde::Deserialize;

use glob::glob;

mod scrobbler;

const USAGE: &'static str = "
LastFM rust scrobbler. Scrobbles all mp3 in a folder.

Usage:
  rust-scrobbler <folder>...
  rust-scrobbler (-h | --help)
  rust-scrobbler --version

Arguments:
  folder        Folder with music to scrobble.

Options:
  -h --help     Show this message.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_folder: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let configuration = reader::from_file(Path::new("app.conf")).expect("Failed to load app.conf");
    let scrobbler = scrobbler::ScrobbleClient::new(&configuration);

    for folder in args.arg_folder {
        let expr = format!("{}/{}", folder, "**/*.mp3");
        for entry in glob(&*expr).unwrap() {
            match entry {
                Ok(path) => {
                    println!("{:?}", path.display());
                    let tag = Tag::read_from_path(path).unwrap();
                    println!("{:?}",tag);

                    let artist = tag.artist().unwrap();
                    let album = tag.album().unwrap();
                    let title = tag.title().unwrap();

                    println!("Scrobbling {} - {} - {}", artist, album, title);
                    scrobbler.now_playing(artist, title, album);

                    let length = tag.duration().unwrap_or_else(|| 10);
                    println!("Waiting for {:?} seconds", length);
                    let duration = time::Duration::from_secs(length as u64);
                    thread::sleep(duration);

                    scrobbler.scrobble(artist, title, album);

                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

}
