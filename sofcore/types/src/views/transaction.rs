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

//! View onto transaction srlp
use bytes::Bytes;
use sophon_types::{H256, U256};
use hash::keccak;
use super::ViewRlp;

/// View onto transaction srlp.
pub struct TransactionView<'a> {
	srlp: ViewRlp<'a>
}

impl<'a> TransactionView<'a> {
	/// Creates new view onto valid transaction srlp.
	/// Use the `view!` macro to create this view in order to capture debugging info.
	///
	/// # Example
	///
	/// ```
	/// #[macro_use]
	/// extern crate common_types as types;
	///
	/// use types::views::{TransactionView};
	///
	/// fn main() {
	/// let bytes : &[u8] = &[];
	/// let tx_view = view!(TransactionView, bytes);
	/// }
	/// ```
	pub fn new(srlp: ViewRlp<'a>) -> TransactionView<'a> {
		TransactionView {
			srlp: srlp
		}
	}

	/// Return reference to underlaying srlp.
	pub fn srlp(&self) -> &ViewRlp<'a> {
		&self.srlp
	}

	/// Returns transaction hash.
	pub fn hash(&self) -> H256 {
		keccak(self.srlp.as_raw())
	}

	/// Get the nonce field of the transaction.
	pub fn nonce(&self) -> U256 { self.srlp.val_at(0) }

	/// Get the gas_price field of the transaction.
	pub fn gas_price(&self) -> U256 { self.srlp.val_at(1) }

	/// Get the gas field of the transaction.
	pub fn gas(&self) -> U256 { self.srlp.val_at(2) }

	/// Get the value field of the transaction.
	pub fn value(&self) -> U256 { self.srlp.val_at(4) }

	/// Get the data field of the transaction.
	pub fn data(&self) -> Bytes { self.srlp.val_at(5) }

	/// Get the v field of the transaction.
	pub fn v(&self) -> u8 { let r: u16 = self.srlp.val_at(6); r as u8 }

	/// Get the r field of the transaction.
	pub fn r(&self) -> U256 { self.srlp.val_at(7) }

	/// Get the s field of the transaction.
	pub fn s(&self) -> U256 { self.srlp.val_at(8) }
}

#[cfg(test)]
mod tests {
	use rustc_hex::FromHex;
	use super::TransactionView;

	#[test]
	fn test_transaction_view() {
		let srlp = "f87c80018261a894095e7baea6a6c7c4c2dfeb977efac326af552d870a9d00000000000000000000000000000000000000000000000000000000001ba048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a0efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804".from_hex().unwrap();

		let view = view!(TransactionView, &srlp);
		assert_eq!(view.nonce(), 0.into());
		assert_eq!(view.gas_price(), 1.into());
		assert_eq!(view.gas(), 0x61a8.into());
		assert_eq!(view.value(), 0xa.into());
		assert_eq!(view.data(), "0000000000000000000000000000000000000000000000000000000000".from_hex().unwrap());
		assert_eq!(view.r(), "48b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353".into());
		assert_eq!(view.s(), "efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804".into());
		assert_eq!(view.v(), 0x1b);
	}
}
