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

//! Susy-specific rpc interface.

use std::collections::BTreeMap;

use sophon_types::{H64, H160, H256, H512, U64, U256};
use susy_jsonrpc_core::{BoxFuture, Result};
use susy_jsonrpc_derive::rpc;
use v1::types::{
	Bytes, CallRequest,
	Peers, Transaction, RpcSettings, Histogram, RecoveredAccount,
	TransactionStats, LocalTransactionStatus,
	BlockNumber, ConsensusCapability, VersionInfo,
	OperationsInfo, ChainStatus, Log, Filter,
	RichHeader, Receipt,
};

/// Susy-specific rpc interface.
#[rpc]
pub trait Susy {
	/// RPC Metadata
	type Metadata;

	/// Returns current transactions limit.
	#[rpc(name = "susy_transactionsLimit")]
	fn transactions_limit(&self) -> Result<usize>;

	/// Returns mining extra data.
	#[rpc(name = "susy_extraData")]
	fn extra_data(&self) -> Result<Bytes>;

	/// Returns mining gas floor target.
	#[rpc(name = "susy_gasFloorTarget")]
	fn gas_floor_target(&self) -> Result<U256>;

	/// Returns mining gas floor cap.
	#[rpc(name = "susy_gasCeilTarget")]
	fn gas_ceil_target(&self) -> Result<U256>;

	/// Returns minimal gas price for transaction to be included in queue.
	#[rpc(name = "susy_minGasPrice")]
	fn min_gas_price(&self) -> Result<U256>;

	/// Returns latest logs
	#[rpc(name = "susy_devLogs")]
	fn dev_logs(&self) -> Result<Vec<String>>;

	/// Returns logs levels
	#[rpc(name = "susy_devLogsLevels")]
	fn dev_logs_levels(&self) -> Result<String>;

	/// Returns chain name - DEPRECATED. Use `susy_chainName` instead.
	#[rpc(name = "susy_netChain")]
	fn net_chain(&self) -> Result<String>;

	/// Returns peers details
	#[rpc(name = "susy_netPeers")]
	fn net_peers(&self) -> Result<Peers>;

	/// Returns network port
	#[rpc(name = "susy_netPort")]
	fn net_port(&self) -> Result<u16>;

	/// Returns rpc settings
	#[rpc(name = "susy_rpcSettings")]
	fn rpc_settings(&self) -> Result<RpcSettings>;

	/// Returns node name
	#[rpc(name = "susy_nodeName")]
	fn node_name(&self) -> Result<String>;

	/// Returns default extra data
	#[rpc(name = "susy_defaultExtraData")]
	fn default_extra_data(&self) -> Result<Bytes>;

	/// Returns distribution of gas price in latest blocks.
	#[rpc(name = "susy_gasPriceHistogram")]
	fn gas_price_histogram(&self) -> BoxFuture<Histogram>;

	/// Returns number of unsigned transactions waiting in the signer queue (if signer enabled)
	/// Returns error when signer is disabled
	#[rpc(name = "susy_unsignedTransactionsCount")]
	fn unsigned_transactions_count(&self) -> Result<usize>;

	/// Returns a cryptographically random phrase sufficient for securely seeding a secret key.
	#[rpc(name = "susy_generateSecretPhrase")]
	fn generate_secret_phrase(&self) -> Result<String>;

	/// Returns whatever address would be derived from the given phrase if it were to seed a brainwallet.
	#[rpc(name = "susy_phraseToAddress")]
	fn phrase_to_address(&self, String) -> Result<H160>;

	/// Returns the value of the registrar for this network.
	#[rpc(name = "susy_registryAddress")]
	fn registry_address(&self) -> Result<Option<H160>>;

	/// Returns all addresses if Fat DB is enabled (`--fat-db`), or null if not.
	#[rpc(name = "susy_listAccounts")]
	fn list_accounts(&self, u64, Option<H160>, Option<BlockNumber>) -> Result<Option<Vec<H160>>>;

	/// Returns all storage keys of the given address (first parameter) if Fat DB is enabled (`--fat-db`),
	/// or null if not.
	#[rpc(name = "susy_listStorageKeys")]
	fn list_storage_keys(&self, H160, u64, Option<H256>, Option<BlockNumber>) -> Result<Option<Vec<H256>>>;

	/// Encrypt some data with a public key under ECIES.
	/// First parameter is the 512-byte destination public key, second is the message.
	#[rpc(name = "susy_encryptMessage")]
	fn encrypt_message(&self, H512, Bytes) -> Result<Bytes>;

	/// Returns all pending transactions from transaction queue.
	#[rpc(name = "susy_pendingTransactions")]
	fn pending_transactions(&self, Option<usize>) -> Result<Vec<Transaction>>;

	/// Returns all transactions from transaction queue.
	///
	/// Some of them might not be ready to be included in a block yet.
	#[rpc(name = "susy_allTransactions")]
	fn all_transactions(&self) -> Result<Vec<Transaction>>;

	/// Same as susy_allTransactions, but return only transactions hashes.
	#[rpc(name = "susy_allTransactionHashes")]
	fn all_transaction_hashes(&self) -> Result<Vec<H256>>;

	/// Returns all future transactions from transaction queue (deprecated)
	#[rpc(name = "susy_futureTransactions")]
	fn future_transactions(&self) -> Result<Vec<Transaction>>;

	/// Returns propagation statistics on transactions pending in the queue.
	#[rpc(name = "susy_pendingTransactionsStats")]
	fn pending_transactions_stats(&self) -> Result<BTreeMap<H256, TransactionStats>>;

	/// Returns a list of current and past local transactions with status details.
	#[rpc(name = "susy_localTransactions")]
	fn local_transactions(&self) -> Result<BTreeMap<H256, LocalTransactionStatus>>;

	/// Returns current WS Server interface and port or an error if ws server is disabled.
	#[rpc(name = "susy_wsUrl")]
	fn ws_url(&self) -> Result<String>;

	/// Returns next nonce for particular sender. Should include all transactions in the queue.
	#[rpc(name = "susy_nextNonce")]
	fn next_nonce(&self, H160) -> BoxFuture<U256>;

	/// Get the mode. Returns one of: "active", "passive", "dark", "offline".
	#[rpc(name = "susy_mode")]
	fn mode(&self) -> Result<String>;

	/// Get the chain name. Returns one of the pre-configured chain names or a filename.
	#[rpc(name = "susy_chain")]
	fn chain(&self) -> Result<String>;

	/// Get the enode of this node.
	#[rpc(name = "susy_enode")]
	fn enode(&self) -> Result<String>;

	/// Returns information on current consensus capability.
	#[rpc(name = "susy_consensusCapability")]
	fn consensus_capability(&self) -> Result<ConsensusCapability>;

	/// Get our version information in a nice object.
	#[rpc(name = "susy_versionInfo")]
	fn version_info(&self) -> Result<VersionInfo>;

	/// Get information concerning the latest releases if available.
	#[rpc(name = "susy_releasesInfo")]
	fn releases_info(&self) -> Result<Option<OperationsInfo>>;

	/// Get the current chain status.
	#[rpc(name = "susy_chainStatus")]
	fn chain_status(&self) -> Result<ChainStatus>;

	/// Get node kind info.
	#[rpc(name = "susy_nodeKind")]
	fn node_kind(&self) -> Result<::v1::types::NodeKind>;

	/// Get block header.
	/// Same as `sof_getBlockByNumber` but without uncles and transactions.
	#[rpc(name = "susy_getBlockHeaderByNumber")]
	fn block_header(&self, Option<BlockNumber>) -> BoxFuture<RichHeader>;

	/// Get block receipts.
	/// Allows you to fetch receipts from the entire block at once.
	/// If no parameter is provided defaults to `latest`.
	#[rpc(name = "susy_getBlockReceipts")]
	fn block_receipts(&self, Option<BlockNumber>) -> BoxFuture<Vec<Receipt>>;

	/// Get IPFS CIDv0 given protobuf encoded bytes.
	#[rpc(name = "susy_cidV0")]
	fn ipfs_cid(&self, Bytes) -> Result<String>;

	/// Call contract, returning the output data.
	#[rpc(name = "susy_call")]
	fn call(&self, Vec<CallRequest>, Option<BlockNumber>) -> Result<Vec<Bytes>>;

	/// Used for submitting a proof-of-work solution (similar to `sof_submitWork`,
	/// but returns block hash on success, and returns an explicit error message on failure).
	#[rpc(name = "susy_submitWorkDetail")]
	fn submit_work_detail(&self, H64, H256, H256) -> Result<H256>;

	/// Returns the status of the node. Used as the health endpoint.
	///
	/// The RPC returns successful response if:
	/// - The node have a peer (unless running a dev chain)
	/// - The node is not syncing.
	///
	/// Otherwise the RPC returns error.
	#[rpc(name = "susy_nodeStatus")]
	fn status(&self) -> Result<()>;

	/// Extracts Address and public key from signature using the r, s and v params. Equivalent to Polynomial erecover
	/// as well as checks the signature for chain replay protection
	#[rpc(name = "susy_verifySignature")]
	fn verify_signature(&self, bool, Bytes, H256, H256, U64) -> Result<RecoveredAccount>;

	/// Returns logs matching given filter object.
	/// Is allowed to skip filling transaction hash for faster query.
	#[rpc(name = "susy_getLogsNoTransactionHash")]
	fn logs_no_tx_hash(&self, Filter) -> BoxFuture<Vec<Log>>;
}
