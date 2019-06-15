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

//! Stores recently seen bad blocks.

use bytes::{Bytes, ToPretty};
use sophon_types::H256;
use itertools::Itertools;
use memory_cache::MemoryLruCache;
use parking_lot::RwLock;
use verification::queue::kind::blocks::Unverified;

/// Recently seen bad blocks.
pub struct BadBlocks {
	last_blocks: RwLock<MemoryLruCache<H256, (Unverified, String)>>,
}

impl Default for BadBlocks {
	fn default() -> Self {
		BadBlocks {
			last_blocks: RwLock::new(MemoryLruCache::new(8 * 1024 * 1024)),
		}
	}
}

impl BadBlocks {
	/// Reports given SRLP as invalid block.
	pub fn report(&self, raw: Bytes, message: String) {
		match Unverified::from_srlp(raw) {
			Ok(unverified) => {
				error!(
					target: "client",
					"\nBad block detected: {}\nSRLP: {}\nHeader: {:?}\nUncles: {}\nTransactions:{}\n",
					message,
					unverified.bytes.to_hex(),
					unverified.header,
					unverified.uncles
						.iter()
						.enumerate()
						.map(|(index, uncle)| format!("[Uncle {}] {:?}", index, uncle))
						.join("\n"),
					unverified.transactions
						.iter()
						.enumerate()
						.map(|(index, tx)| format!("[Tx {}] {:?}", index, tx))
						.join("\n"),
				);
				self.last_blocks.write().insert(unverified.header.hash(), (unverified, message));
			},
			Err(err) => {
				error!(target: "client", "Bad undecodable block detected: {}\n{:?}", message, err);
			},
		}
	}

	/// Returns a list of recently detected bad blocks with error descriptions.
	pub fn bad_blocks(&self) -> Vec<(Unverified, String)> {
		self.last_blocks.read()
			.backstore()
			.iter()
			.map(|(_k, (unverified, message))| (
				Unverified::from_srlp(unverified.bytes.clone())
					.expect("Bytes coming from UnverifiedBlock so decodable; qed"),
				message.clone(),
			))
			.collect()
	}
}
