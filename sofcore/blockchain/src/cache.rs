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

/// Represents blockchain's in-memory cache size in bytes.
#[derive(Debug)]
pub struct CacheSize {
	/// Blocks cache size.
	pub blocks: usize,
	/// BlockDetails cache size.
	pub block_details: usize,
	/// Transaction addresses cache size.
	pub transaction_addresses: usize,
	/// Block receipts size.
	pub block_receipts: usize,
}

impl CacheSize {
	/// Total amount used by the cache.
	pub fn total(&self) -> usize {
		self.blocks + self.block_details + self.transaction_addresses + self.block_receipts
	}
}
