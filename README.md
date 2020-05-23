# gmusic-to-spotify

Transfer Google Music playlists to Spotify.

> This is not a fully featured command line app to transfer Google Music playlist to Spotify. There are manual steps required to get this working.

## Usage

```
cargo run <gmusic_playlist_json> <spotify_playlist_id>
```

## Google Music playlist

There doesn't seem to be a good programatic way to get Google Music playlists from their API. I tried [gmusic-rs](https://github.com/maxjoehnk/gmusic-rs) which seems to be using undocumented skyjam API which I couldn't get it to work, plus getting the client ID and secret is not very straighforward.

### Getting playlist JSON

Getting playlist in a JSON format is easy but it requires some manualy work. Navigate to [music.google.com](https://play.google.com/music/listen#/wmp) and use your browser developer tools to grab the playlist JSON payload.

Before you click on the playlist you want to save, open developer tools and navigate to Network tab. Click on the playlist you want to save and then look for the request URL that starts with this `https://play.google.com/music/services/loaduserplaylist`. Right click on it and navigate to Copy -> Copy Response. This will store the response JSON to your clipboard.

Create a new file naming it after your playlist e.g. `summer-2015.json` and save it. You can do the same for the automatic Thumbs up playlist and save it as `favorites.json`.

## Spotify credentials

In order to use Spotify API you need to fill out values inside `spotify.env` file. Navigate to [Spotify Dashboard](https://developer.spotify.com/dashboard) where you need to create a new Client ID. Give it a name, description, check "I don't know" and agree to finish the process.

The next page should give you Client ID and Client Secret. Copy those two values to `spotify.env`. Click Edit Settings and make sure you enter `http://localhost` under Redirect URIs.

Last thing you need is your User ID. You can find this in the URL if you go to your Profile in Spotify web app. If you're using your desktop app, clicking on your name at the top and then clicking three dots: Share -> Copy Spotify URI will give you your full User ID. Copy it to `spotify.env` as well.

## Transfer favorite tracks

To transfer Google Play music "Thumbs up" playlist to Spotify "Liked Songs"

```
cargo run favorites.json
```

You will be take to Spotify website where you need to allow access and then redirected back to `localhost`. Copy the entire localhost URL and paste it back into the terminal prompt.

## Transfer other playlists

To transfer any other playlist you'll need to create new Spotify playlist and get the playlist ID and pass it as the last argument. You can get the playlist ID the same way as you got your user ID, either URL from the website or via Share menu.

```
cargo run summer-2015.json spotify:playlist:xxxxxxxxxxxxxxxxxxxxxx
```
