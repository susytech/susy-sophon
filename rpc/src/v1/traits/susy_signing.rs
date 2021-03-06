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

//! SusySigning rpc interface.
use susy_jsonrpc_core::{BoxFuture, Result};
use susy_jsonrpc_derive::rpc;

use sophon_types::{H160, U256};
use v1::types::{Bytes, ConfirmationResponse, TransactionRequest, Either};

/// Signing methods implementation.
#[rpc]
pub trait SusySigning {
	/// RPC Metadata
	type Metadata;

	/// Given partial transaction request produces transaction with all fields filled in.
	/// Such transaction can be then signed externally.
	#[rpc(meta, name = "susy_composeTransaction")]
	fn compose_transaction(&self, Self::Metadata, TransactionRequest) -> BoxFuture<TransactionRequest>;

	/// Posts sign request asynchronously.
	/// Will return a confirmation ID for later use with check_transaction.
	#[rpc(meta, name = "susy_postSign")]
	fn post_sign(&self, Self::Metadata, H160, Bytes) -> BoxFuture<Either<U256, ConfirmationResponse>>;

	/// Posts transaction asynchronously.
	/// Will return a transaction ID for later use with check_transaction.
	#[rpc(meta, name = "susy_postTransaction")]
	fn post_transaction(&self, Self::Metadata, TransactionRequest) -> BoxFuture<Either<U256, ConfirmationResponse>>;

	/// Checks the progress of a previously posted request (transaction/sign).
	/// Should be given a valid send_transaction ID.
	#[rpc(name = "susy_checkRequest")]
	fn check_request(&self, U256) -> Result<Option<ConfirmationResponse>>;

	/// Decrypt some ECIES-encrypted message.
	/// First parameter is the address with which it is encrypted, second is the ciphertext.
	#[rpc(meta, name = "susy_decryptMessage")]
	fn decrypt_message(&self, Self::Metadata, H160, Bytes) -> BoxFuture<Bytes>;
}
