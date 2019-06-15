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

//! Base data structure of this module is `Block`.
//!
//! Blocks can be produced by a local node or they may be received from the network.
//!
//! To create a block locally, we start with an `OpenBlock`. This block is mutable
//! and can be appended to with transactions and uncles.
//!
//! When ready, `OpenBlock` can be closed and turned into a `ClosedBlock`. A `ClosedBlock` can
//! be reopend again by a miner under certain circumstances. On block close, state commit is
//! performed.
//!
//! `LockedBlock` is a version of a `ClosedBlock` that cannot be reopened. It can be sealed
//! using an engine.
//!
//! `ExecutedBlock` is an underlaying data structure used by all structs above to store block
//! related info.

use bytes::Bytes;

use header::Header;
use srlp::{Rlp, RlpStream, Decodable, DecoderError};
use transaction::UnverifiedTransaction;

/// A block, encoded as it is on the block chain.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Block {
	/// The header of this block.
	pub header: Header,
	/// The transactions in this block.
	pub transactions: Vec<UnverifiedTransaction>,
	/// The uncles of this block.
	pub uncles: Vec<Header>,
}

impl Block {
	/// Get the SRLP-encoding of the block with the seal.
	pub fn srlp_bytes(&self) -> Bytes {
		let mut block_srlp = RlpStream::new_list(3);
		block_srlp.append(&self.header);
		block_srlp.append_list(&self.transactions);
		block_srlp.append_list(&self.uncles);
		block_srlp.out()
	}
}

impl Decodable for Block {
	fn decode(srlp: &Rlp) -> Result<Self, DecoderError> {
		if srlp.as_raw().len() != srlp.payload_info()?.total() {
			return Err(DecoderError::RlpIsTooBig);
		}
		if srlp.item_count()? != 3 {
			return Err(DecoderError::RlpIncorrectListLen);
		}
		Ok(Block {
			header: srlp.val_at(0)?,
			transactions: srlp.list_at(1)?,
			uncles: srlp.list_at(2)?,
		})
	}
}
