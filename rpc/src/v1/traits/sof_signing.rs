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

//! Sof rpc interface.

use susy_jsonrpc_core::BoxFuture;
use susy_jsonrpc_derive::rpc;

use sophon_types::{H160, H256, H520};
use v1::types::{Bytes, TransactionRequest, RichRawTransaction};

/// Signing methods implementation relying on unlocked accounts.
#[rpc]
pub trait SofSigning {
	/// RPC Metadata
	type Metadata;

	/// Signs the hash of data with given address signature.
	#[rpc(meta, name = "sof_sign")]
	fn sign(&self, Self::Metadata, H160, Bytes) -> BoxFuture<H520>;

	/// Sends transaction; will block waiting for signer to return the
	/// transaction hash.
	/// If Signer is disable it will require the account to be unlocked.
	#[rpc(meta, name = "sof_sendTransaction")]
	fn send_transaction(&self, Self::Metadata, TransactionRequest) -> BoxFuture<H256>;

	/// Signs transactions without dispatching it to the network.
	/// Returns signed transaction SRLP representation and the transaction itself.
	/// It can be later submitted using `sof_sendRawTransaction/sof_submitTransaction`.
	#[rpc(meta, name = "sof_signTransaction")]
	fn sign_transaction(&self, Self::Metadata, TransactionRequest) -> BoxFuture<RichRawTransaction>;
}
