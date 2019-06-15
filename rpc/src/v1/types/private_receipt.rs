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

use sofcore_private_tx::{Receipt as SofPrivateReceipt};
use sophon_types::{H160, H256};
use v1::types::TransactionRequest;

/// Receipt
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateTransactionReceipt {
	/// Transaction Hash
	pub transaction_hash: H256,
	/// Private contract address
	pub contract_address: H160,
	/// Status code
	#[serde(rename = "status")]
	pub status_code: u8,
}

impl From<SofPrivateReceipt> for PrivateTransactionReceipt {
	fn from(r: SofPrivateReceipt) -> Self {
		PrivateTransactionReceipt {
			transaction_hash: r.hash.into(),
			contract_address: r.contract_address.into(),
			status_code: r.status_code.into(),
		}
	}
}

/// Receipt and Transaction
#[derive(Debug, Serialize)]
pub struct PrivateTransactionReceiptAndTransaction {
	/// Receipt
	pub receipt: PrivateTransactionReceipt,
	/// Transaction
	pub transaction: TransactionRequest,
}
