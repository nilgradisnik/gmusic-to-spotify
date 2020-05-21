use serde::Deserialize;
use serde_json::Value;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
}

pub fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader)?;

    Ok(json)
}

pub fn parse_songs(json: Value) -> Result<Vec<Song>, Box<dyn Error>> {
    let mut songs: Vec<Song> = Vec::new();

    for song in json[1][0].as_array().unwrap() {
        songs.push(Song {
            title: song[1].as_str().unwrap().to_string(),
            artist: song[3].as_str().unwrap().to_string(),
            album: song[4].as_str().unwrap().to_string(),
        });
    }

    Ok(songs)
}
