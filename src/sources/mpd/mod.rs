//! This is the module for the `MPD` source for rscribble.

// This file is part of rscribble.

// rscribble is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// rscribble is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with rscribble.  If not, see <http://www.gnu.org/licenses/>

extern crate mpd;

use mpd::idle::Idle;
use mpd::Client;
use mpd::idle::Subsystem::Player;
use std::collections::BTreeMap;

/// Return the Artist in a BTree of tags.
pub fn get_artist(tags: BTreeMap<String, String>) -> String {
    match tags.get("Artist") {
        Some(x) => x.to_owned(),
        None => "None".to_owned(),
    }
}

/// Return the Album in a BTree of tags.
pub fn get_album(tags: BTreeMap<String, String>) -> String {
    match tags.get("Album") {
        Some(x) => x.to_owned(),
        None => "None".to_owned(),
    }
}

/// Loop over MPD `Player` events, and display the song and artist.
pub fn loop_mpd_player_song_artist_disp() {
    let addr = "127.0.0.1:6600";
    let mut conn = Client::connect(addr).unwrap();

    loop {
        let _ = conn.wait(&[Player]);
        match conn.currentsong().unwrap() {
            Some(s) => {
                println!("New song detected.");
                let song_title = s.title.unwrap();
                let song_artist = get_artist(s.tags.clone());
                let song_album = get_album(s.tags.clone());

                println!("Song title: {}", song_title);
                println!("Song artist: {}", song_artist);
                println!("Song album: {}", song_album);
            }
            None => {
                println!("No song detected.");
            }
        }
    }


}
