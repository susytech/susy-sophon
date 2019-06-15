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

//! Block SRLP compression.

use bytes::Bytes;
use sophon_types::H256;
use hash::keccak;
use srlp::{DecoderError, RlpStream, Rlp};
use triehash::ordered_trie_root;
use types::block::Block;
use types::header::Header;
use types::views::BlockView;

const HEADER_FIELDS: usize = 8;
const BLOCK_FIELDS: usize = 2;

pub struct AbridgedBlock {
	srlp: Bytes,
}

impl AbridgedBlock {
	/// Create from srlp-compressed bytes. Does no verification.
	pub fn from_raw(compressed: Bytes) -> Self {
		AbridgedBlock {
			srlp: compressed,
		}
	}

	/// Return the inner bytes.
	pub fn into_inner(self) -> Bytes {
		self.srlp
	}

	/// Given a full block view, trim out the parent hash and block number,
	/// producing new srlp.
	pub fn from_block_view(block_view: &BlockView) -> Self {
		let header = block_view.header_view();
		let seal_fields = header.seal();

		// 10 header fields, unknown number of seal fields, and 2 block fields.
		let mut stream = RlpStream::new_list(
			HEADER_FIELDS +
			seal_fields.len() +
			BLOCK_FIELDS
		);

		// write header values.
		stream
			.append(&header.author())
			.append(&header.state_root())
			.append(&header.log_bloom())
			.append(&header.difficulty())
			.append(&header.gas_limit())
			.append(&header.gas_used())
			.append(&header.timestamp())
			.append(&header.extra_data());

		// write block values.
		stream
			.append_list(&block_view.transactions())
			.append_list(&block_view.uncles());

		// write seal fields.
		for field in seal_fields {
			stream.append_raw(&field, 1);
		}

		AbridgedBlock {
			srlp: stream.out(),
		}
	}

	/// Flesh out an abridged block view with the provided parent hash and block number.
	///
	/// Will fail if contains invalid srlp.
	pub fn to_block(&self, parent_hash: H256, number: u64, receipts_root: H256) -> Result<Block, DecoderError> {
		let srlp = Rlp::new(&self.srlp);

		let mut header: Header = Default::default();
		header.set_parent_hash(parent_hash);
		header.set_author(srlp.val_at(0)?);
		header.set_state_root(srlp.val_at(1)?);
		header.set_log_bloom(srlp.val_at(2)?);
		header.set_difficulty(srlp.val_at(3)?);
		header.set_number(number);
		header.set_gas_limit(srlp.val_at(4)?);
		header.set_gas_used(srlp.val_at(5)?);
		header.set_timestamp(srlp.val_at(6)?);
		header.set_extra_data(srlp.val_at(7)?);

		let transactions = srlp.list_at(8)?;
		let uncles: Vec<Header> = srlp.list_at(9)?;

		header.set_transactions_root(ordered_trie_root(
			srlp.at(8)?.iter().map(|r| r.as_raw())
		));
		header.set_receipts_root(receipts_root);

		let mut uncles_srlp = RlpStream::new();
		uncles_srlp.append_list(&uncles);
		header.set_uncles_hash(keccak(uncles_srlp.as_raw()));

		let mut seal_fields = Vec::new();
		for i in (HEADER_FIELDS + BLOCK_FIELDS)..srlp.item_count()? {
			let seal_srlp = srlp.at(i)?;
			seal_fields.push(seal_srlp.as_raw().to_owned());
		}

		header.set_seal(seal_fields);

		Ok(Block {
			header: header,
			transactions: transactions,
			uncles: uncles,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::AbridgedBlock;

	use bytes::Bytes;
	use sophon_types::{H256, U256, Address};
	use types::transaction::{Action, Transaction};
	use types::block::Block;
	use types::view;
	use types::views::BlockView;

	fn encode_block(b: &Block) -> Bytes {
		b.srlp_bytes()
	}

	#[test]
	fn empty_block_abridging() {
		let b = Block::default();
		let receipts_root = b.header.receipts_root().clone();
		let encoded = encode_block(&b);

		let abridged = AbridgedBlock::from_block_view(&view!(BlockView, &encoded));
		assert_eq!(abridged.to_block(H256::new(), 0, receipts_root).unwrap(), b);
	}

	#[test]
	#[should_panic]
	fn wrong_number() {
		let b = Block::default();
		let receipts_root = b.header.receipts_root().clone();
		let encoded = encode_block(&b);

		let abridged = AbridgedBlock::from_block_view(&view!(BlockView, &encoded));
		assert_eq!(abridged.to_block(H256::new(), 2, receipts_root).unwrap(), b);
	}

	#[test]
	fn with_transactions() {
		let mut b = Block::default();

		let t1 = Transaction {
			action: Action::Create,
			nonce: U256::from(42),
			gas_price: U256::from(3000),
			gas: U256::from(50_000),
			value: U256::from(1),
			data: b"Hello!".to_vec()
		}.fake_sign(Address::from(0x69));

		let t2 = Transaction {
			action: Action::Create,
			nonce: U256::from(88),
			gas_price: U256::from(12345),
			gas: U256::from(300000),
			value: U256::from(1000000000),
			data: "Eep!".into(),
		}.fake_sign(Address::from(0x55));

		b.transactions.push(t1.into());
		b.transactions.push(t2.into());

		let receipts_root = b.header.receipts_root().clone();
		b.header.set_transactions_root(::triehash::ordered_trie_root(
			b.transactions.iter().map(::srlp::encode)
		));

		let encoded = encode_block(&b);

		let abridged = AbridgedBlock::from_block_view(&view!(BlockView, &encoded[..]));
		assert_eq!(abridged.to_block(H256::new(), 0, receipts_root).unwrap(), b);
	}
}