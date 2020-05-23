use dotenv;
use std::env;

use clap::clap_app;

mod gmusic;
mod spotify;

use gmusic::{parse_songs, read_json_from_file, Song};
use spotify::{find_track, playlist_tracks_add, saved_tracks_add};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[tokio::main]
async fn main() {
    dotenv::from_filename("my.env").ok();

    let matches = clap_app!(gmusic_to_spotify =>
        (version: VERSION)
        (author: AUTHOR)
        (about: DESCRIPTION)
        (@arg GMUSIC_PLAYLIST_JSON: +required "Sets the Google Music playlist input JSON file to use")
        (@arg SPOTIFY_PLAYLIST_ID: "Sets the Spotify playlist ID")
    )
    .get_matches();

    let user_id = env::var("USER_ID").expect("Spotify user ID required");
    let file_name = matches.value_of("GMUSIC_PLAYLIST_JSON").unwrap();
    let playlist_id = matches.value_of("SPOTIFY_PLAYLIST_ID").or(None);

    match read_json_from_file(file_name) {
        Ok(json) => match parse_songs(json) {
            Ok(songs) => {
                let track_ids = find_tracks(songs).await;

                if playlist_id.is_some() {
                    playlist_tracks_add(&user_id, &playlist_id.unwrap(), track_ids.to_vec()).await;
                } else {
                    saved_tracks_add(track_ids.to_vec()).await;
                }
            }
            Err(error) => eprintln!("Parse: {:#?}", error),
        },
        Err(error) => eprintln!("Read: {:#?}", error),
    }
}

async fn find_tracks(songs: Vec<Song>) -> Vec<String> {
    let mut track_ids: Vec<String> = vec![];

    for song in songs {
        let query = format!("{} {}", song.title, song.artist);

        match find_track(&query).await {
            Some(track) => {
                let track_id = track.id.as_ref().unwrap();

                println!(
                    "Song [{}] {}, {}",
                    track_id, track.name, track.artists[0].name
                );

                track_ids.push(track_id.to_string());
            }
            None => println!("No track found"),
        }
    }

    track_ids
}
