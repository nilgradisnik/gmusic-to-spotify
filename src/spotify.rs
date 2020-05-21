extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::model::track::FullTrack;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::senum::Country;
use rspotify::util::get_token;

pub async fn find_track(query: &str) -> Option<FullTrack> {
    let spotify = authenticate("user-read-private").await;

    println!("Searching for \"{}\"", query);
    match spotify
        .search_track(query, 10, 0, Some(Country::UnitedStates))
        .await
    {
        Ok(result) => {
            if result.tracks.items.len() > 0 {
                Some(result.tracks.items[0].to_owned())
            } else {
                None
            }
        }
        Err(_) => {
            eprintln!("Error finding tracks");
            None
        }
    }
}

pub async fn saved_tracks_add(track_ids: Vec<String>) {
    let spotify = authenticate("user-library-modify").await;

    println!("Adding {} tracks...", track_ids.len());
    for ids in track_ids.chunks(50) {
        match spotify.current_user_saved_tracks_add(&ids).await {
            Ok(_) => println!("Add saved tracks successful"),
            Err(err) => eprintln!("Error saving tracks: {:?}", err),
        }
    }
}

pub async fn playlist_tracks_add(user_id: &str, playlist_id: &str, track_ids: Vec<String>) {
    let spotify = authenticate("playlist-modify-private playlist-modify-public").await;

    println!("Adding {} tracks...", track_ids.len());
    for ids in track_ids.chunks(50) {
        match spotify
            .user_playlist_add_tracks(user_id, playlist_id, &ids, None)
            .await
        {
            Ok(_) => println!("Tracks added successfully"),
            Err(err) => eprintln!("Error adding tracks: {:?}", err),
        }
    }
}

pub async fn _playlists() {
    let spotify = authenticate("playlist-read-private").await;

    match spotify.current_user_playlists(10, None).await {
        Ok(playlists) => println!("Playlists: {:?}", playlists),
        Err(_) => eprintln!("Error getting playlists"),
    }
}

pub async fn _me() {
    let spotify = authenticate("user-read-private user-read-email").await;

    match spotify.me().await {
        Ok(me) => println!("Me: {:?} {:?}", me.id, me.display_name),
        Err(_) => eprintln!("Error getting me"),
    }
}

async fn authenticate(scope: &str) -> Spotify {
    let mut oauth = SpotifyOAuth::default().scope(scope).build();

    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();

            Spotify::default()
                .client_credentials_manager(client_credential)
                .build()
        }
        None => panic!("Spotify auth failed"),
    }
}
