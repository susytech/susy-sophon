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

//! Susy-specific rpc interface for operations altering the settings.

use sophon_types::{H160, H256, U256};
use susy_jsonrpc_core::{BoxFuture, Result};
use susy_jsonrpc_derive::rpc;

use v1::types::{Bytes, ReleaseInfo, Transaction};

/// Susy-specific rpc interface for operations altering the account-related settings.
#[rpc]
pub trait SusySetAccounts {
	/// Sets account for signing consensus messages.
	#[rpc(name = "susy_setEngineSigner")]
	fn set_engine_signer(&self, H160, String) -> Result<bool>;
}

/// Susy-specific rpc interface for operations altering the settings.
#[rpc]
pub trait SusySet {
	/// Sets new minimal gas price for mined blocks.
	#[rpc(name = "susy_setMinGasPrice")]
	fn set_min_gas_price(&self, U256) -> Result<bool>;

	/// Sets new gas floor target for mined blocks.
	#[rpc(name = "susy_setGasFloorTarget")]
	fn set_gas_floor_target(&self, U256) -> Result<bool>;

	/// Sets new gas ceiling target for mined blocks.
	#[rpc(name = "susy_setGasCeilTarget")]
	fn set_gas_ceil_target(&self, U256) -> Result<bool>;

	/// Sets new extra data for mined blocks.
	#[rpc(name = "susy_setExtraData")]
	fn set_extra_data(&self, Bytes) -> Result<bool>;

	/// Sets new author for mined block.
	#[rpc(name = "susy_setAuthor")]
	fn set_author(&self, H160) -> Result<bool>;

	/// Sets the secret of engine signer account.
	#[rpc(name = "susy_setEngineSignerSecret")]
	fn set_engine_signer_secret(&self, H256) -> Result<bool>;

	/// Sets the limits for transaction queue.
	#[rpc(name = "susy_setTransactionsLimit")]
	fn set_transactions_limit(&self, usize) -> Result<bool>;

	/// Sets the maximum amount of gas a single transaction may consume.
	#[rpc(name = "susy_setMaxTransactionGas")]
	fn set_tx_gas_limit(&self, U256) -> Result<bool>;

	/// Add a reserved peer.
	#[rpc(name = "susy_addReservedPeer")]
	fn add_reserved_peer(&self, String) -> Result<bool>;

	/// Remove a reserved peer.
	#[rpc(name = "susy_removeReservedPeer")]
	fn remove_reserved_peer(&self, String) -> Result<bool>;

	/// Drop all non-reserved peers.
	#[rpc(name = "susy_dropNonReservedPeers")]
	fn drop_non_reserved_peers(&self) -> Result<bool>;

	/// Accept non-reserved peers (default behavior)
	#[rpc(name = "susy_acceptNonReservedPeers")]
	fn accept_non_reserved_peers(&self) -> Result<bool>;

	/// Start the network.
	///
	/// @deprecated - Use `set_mode("active")` instead.
	#[rpc(name = "susy_startNetwork")]
	fn start_network(&self) -> Result<bool>;

	/// Stop the network.
	///
	/// @deprecated - Use `set_mode("offline")` instead.
	#[rpc(name = "susy_stopNetwork")]
	fn stop_network(&self) -> Result<bool>;

	/// Set the mode. Argument must be one of: "active", "passive", "dark", "offline".
	#[rpc(name = "susy_setMode")]
	fn set_mode(&self, String) -> Result<bool>;

	/// Set the network spec. Argument must be one of pre-configured chains or a filename.
	#[rpc(name = "susy_setChain")]
	fn set_spec_name(&self, String) -> Result<bool>;

	/// Hash a file content under given URL.
	#[rpc(name = "susy_hashContent")]
	fn hash_content(&self, String) -> BoxFuture<H256>;

	/// Is there a release ready for install?
	#[rpc(name = "susy_upgradeReady")]
	fn upgrade_ready(&self) -> Result<Option<ReleaseInfo>>;

	/// Execute a release which is ready according to upgrade_ready().
	#[rpc(name = "susy_executeUpgrade")]
	fn execute_upgrade(&self) -> Result<bool>;

	/// Removes transaction from transaction queue.
	/// Makes sense only for transactions that were not propagated to other peers yet
	/// like scheduled transactions or transactions in future.
	/// It might also work for some local transactions with to low gas price
	/// or excessive gas limit that are not accepted by other peers whp.
	/// Returns `true` when transaction was removed, `false` if it was not found.
	#[rpc(name = "susy_removeTransaction")]
	fn remove_transaction(&self, H256) -> Result<Option<Transaction>>;
}
