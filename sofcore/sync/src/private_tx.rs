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

use parking_lot::Mutex;
use sophon_types::H256;

/// Trait which should be implemented by a private transaction handler.
pub trait PrivateTxHandler: Send + Sync + 'static {
	/// Function called on new private transaction received.
	/// Returns the hash of the imported transaction
	fn import_private_transaction(&self, srlp: &[u8]) -> Result<H256, String>;

	/// Function called on new signed private transaction received.
	/// Returns the hash of the imported transaction
	fn import_signed_private_transaction(&self, srlp: &[u8]) -> Result<H256, String>;
}

/// Nonoperative private transaction handler.
pub struct NoopPrivateTxHandler;

impl PrivateTxHandler for NoopPrivateTxHandler {
	fn import_private_transaction(&self, _srlp: &[u8]) -> Result<H256, String> {
		Ok(H256::default())
	}

	fn import_signed_private_transaction(&self, _srlp: &[u8]) -> Result<H256, String> {
		Ok(H256::default())
	}
}

/// Simple private transaction handler. Used for tests.
#[derive(Default)]
pub struct SimplePrivateTxHandler {
	/// imported private transactions
	pub txs: Mutex<Vec<Vec<u8>>>,
	/// imported signed private transactions
	pub signed_txs: Mutex<Vec<Vec<u8>>>,
}

impl PrivateTxHandler for SimplePrivateTxHandler {
	fn import_private_transaction(&self, srlp: &[u8]) -> Result<H256, String> {
		self.txs.lock().push(srlp.to_vec());
		Ok(H256::default())
	}

	fn import_signed_private_transaction(&self, srlp: &[u8]) -> Result<H256, String> {
		self.signed_txs.lock().push(srlp.to_vec());
		Ok(H256::default())
	}
}
