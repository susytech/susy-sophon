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

//! Snapshot tests.

mod proof_of_work;
mod proof_of_authority;
mod state;
mod service;

pub mod helpers;

use super::ManifestData;

#[test]
fn manifest_srlp() {
	let manifest = ManifestData {
		version: 2,
		block_hashes: Vec::new(),
		state_hashes: Vec::new(),
		block_number: 1234567,
		state_root: Default::default(),
		block_hash: Default::default(),
	};
	let raw = manifest.clone().into_srlp();
	assert_eq!(ManifestData::from_srlp(&raw).unwrap(), manifest);
}
