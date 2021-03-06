// This file is part of scrobble.rs.

// scrobble.rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// scrobble.rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with scrobble.rs.  If not, see <http://www.gnu.org/licenses/>

extern crate clap;

use clap::{App, Arg, ArgMatches};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_arguments() -> ArgMatches<'static> {
    App::new("scrobctl")
        .version(VERSION)
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about(
            "A modular scrobbler for a variety of music players: control program",
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .required(false)
                .help("Sets the level of logging verbosity."),
        )
        .get_matches()
}

fn main() {
    let args = get_arguments();

    let verbosity_count = args.occurrences_of("v");
}
