// Copyleft 2015-2019 Superstring.Community
// This file is part of Susy Sophon.

// Susy Sophon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Susy Sophon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MSRCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Susy Sophon.  If not, see <http://www.gnu.org/licenses/>.

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate sophon_types;
extern crate sofjson;
extern crate wasm;
extern crate vm;
extern crate clap;
extern crate rustc_hex;
extern crate env_logger;

mod fixture;
mod runner;

use fixture::Fixture;
use clap::{App, Arg};
use std::fs;

fn main() {
	::env_logger::init();

	let matches = App::new("swasm-run-test")
		.arg(Arg::with_name("target")
			.index(1)
			.required(true)
			.multiple(true)
			.help("JSON fixture"))
		.get_matches();

	let mut exit_code = 0;

	for target in matches.values_of("target").expect("No target parameter") {
		let mut f = fs::File::open(target).expect("Failed to open file");
		let fixtures: Vec<Fixture> = serde_json::from_reader(&mut f).expect("Failed to deserialize json");

		for fixture in fixtures.into_iter() {
			let fails = runner::run_fixture(&fixture);
			for fail in fails.iter() {
				exit_code = 1;
				println!("Failed assert in test \"{}\" ('{}'): {}", fixture.caption.as_ref(), target, fail);
			}
		}
	}

	std::process::exit(exit_code);
}
