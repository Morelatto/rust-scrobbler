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
  rust-scrobbler <folder>... [--loop] [--config=FILE]
  rust-scrobbler (-h | --help)
  rust-scrobbler --version

Arguments:
  folders       Folders without trailing slash with mp3 files to scrobble.

Options:
  --loop            Keep looping on folders after scrobbling everything once.
  --config=FILE     Configuration file path [default: app.conf].
  -h --help         Show this message.
  --version         Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_folder: Vec<String>,
    flag_loop: bool,
    flag_config: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some("0.2.0".to_string())).deserialize())
        .unwrap_or_else(|e| e.exit());

    let configuration = reader::from_file(Path::new(&args.flag_config.unwrap())).unwrap();
    let scrobbler = scrobbler::ScrobbleClient::new(&configuration);

    loop {
        for folder in &args.arg_folder {
            // TODO pass file type on arguments
            let expr = format!("{}/{}", folder, "**/*.mp3");
            for entry in glob(&*expr).unwrap() {
                match entry {
                    Ok(path) => {
                        println!("{:?}", path.display());
                        let tag = Tag::read_from_path(path).unwrap();
                        let artist = tag.artist().unwrap();
                        let album = tag.album().unwrap();
                        let title = tag.title().unwrap();

                        scrobbler.now_playing(artist, title, album);
                        // TODO get real duration of track and add cli option to use it instead
                        wait(tag.duration().unwrap_or_else(|| 30));
                        scrobbler.scrobble(artist, title, album);
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        if args.flag_loop == false {
            break;
        }
    }
}

fn wait(length: u32) {
    println!("Waiting for {:?} seconds", length);
    let duration = time::Duration::from_secs(length as u64);
    thread::sleep(duration);
}
