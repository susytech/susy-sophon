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
use susy_jsonrpc_core::{Result, BoxFuture};
use susy_jsonrpc_derive::rpc;
use sophon_types::{H64, H160, H256, U64, U256};

use v1::types::{RichBlock, BlockNumber, Bytes, CallRequest, Filter, FilterChanges, Index, SofAccount};
use v1::types::{Log, Receipt, SyncStatus, Transaction, Work};

/// Sof rpc interface.
#[rpc]
pub trait Sof {
	/// RPC Metadata
	type Metadata;

	/// Returns protocol version encoded as a string (quotes are necessary).
	#[rpc(name = "sof_protocolVersion")]
	fn protocol_version(&self) -> Result<String>;

	/// Returns an object with data about the sync status or false. (wtf?)
	#[rpc(name = "sof_syncing")]
	fn syncing(&self) -> Result<SyncStatus>;

	/// Returns the number of hashes per second that the node is mining with.
	#[rpc(name = "sof_hashrate")]
	fn hashrate(&self) -> Result<U256>;

	/// Returns block author.
	#[rpc(name = "sof_coinbase")]
	fn author(&self) -> Result<H160>;

	/// Returns true if client is actively mining new blocks.
	#[rpc(name = "sof_mining")]
	fn is_mining(&self) -> Result<bool>;

	/// Returns the chain ID used for transaction signing at the
	/// current best block. None is returned if not
	/// available.
	#[rpc(name = "sof_chainId")]
	fn chain_id(&self) -> Result<Option<U64>>;

	/// Returns current gas_price.
	#[rpc(name = "sof_gasPrice")]
	fn gas_price(&self) -> BoxFuture<U256>;

	/// Returns accounts list.
	#[rpc(name = "sof_accounts")]
	fn accounts(&self) -> Result<Vec<H160>>;

	/// Returns highest block number.
	#[rpc(name = "sof_blockNumber")]
	fn block_number(&self) -> Result<U256>;

	/// Returns balance of the given account.
	#[rpc(name = "sof_getBalance")]
	fn balance(&self, H160, Option<BlockNumber>) -> BoxFuture<U256>;

	/// Returns the account- and storage-values of the specified account including the Merkle-proof
	#[rpc(name = "sof_getProof")]
	fn proof(&self, H160, Vec<H256>, Option<BlockNumber>) -> BoxFuture<SofAccount>;

	/// Returns content of the storage at given address.
	#[rpc(name = "sof_getStorageAt")]
	fn storage_at(&self, H160, U256, Option<BlockNumber>) -> BoxFuture<H256>;

	/// Returns block with given hash.
	#[rpc(name = "sof_getBlockByHash")]
	fn block_by_hash(&self, H256, bool) -> BoxFuture<Option<RichBlock>>;

	/// Returns block with given number.
	#[rpc(name = "sof_getBlockByNumber")]
	fn block_by_number(&self, BlockNumber, bool) -> BoxFuture<Option<RichBlock>>;

	/// Returns the number of transactions sent from given address at given time (block number).
	#[rpc(name = "sof_getTransactionCount")]
	fn transaction_count(&self, H160, Option<BlockNumber>) -> BoxFuture<U256>;

	/// Returns the number of transactions in a block with given hash.
	#[rpc(name = "sof_getBlockTransactionCountByHash")]
	fn block_transaction_count_by_hash(&self, H256) -> BoxFuture<Option<U256>>;

	/// Returns the number of transactions in a block with given block number.
	#[rpc(name = "sof_getBlockTransactionCountByNumber")]
	fn block_transaction_count_by_number(&self, BlockNumber) -> BoxFuture<Option<U256>>;

	/// Returns the number of uncles in a block with given hash.
	#[rpc(name = "sof_getUncleCountByBlockHash")]
	fn block_uncles_count_by_hash(&self, H256) -> BoxFuture<Option<U256>>;

	/// Returns the number of uncles in a block with given block number.
	#[rpc(name = "sof_getUncleCountByBlockNumber")]
	fn block_uncles_count_by_number(&self, BlockNumber) -> BoxFuture<Option<U256>>;

	/// Returns the code at given address at given time (block number).
	#[rpc(name = "sof_getCode")]
	fn code_at(&self, H160, Option<BlockNumber>) -> BoxFuture<Bytes>;

	/// Sends signed transaction, returning its hash.
	#[rpc(name = "sof_sendRawTransaction")]
	fn send_raw_transaction(&self, Bytes) -> Result<H256>;

	/// @alias of `sof_sendRawTransaction`.
	#[rpc(name = "sof_submitTransaction")]
	fn submit_transaction(&self, Bytes) -> Result<H256>;

	/// Call contract, returning the output data.
	#[rpc(name = "sof_call")]
	fn call(&self, CallRequest, Option<BlockNumber>) -> BoxFuture<Bytes>;

	/// Estimate gas needed for execution of given contract.
	#[rpc(name = "sof_estimateGas")]
	fn estimate_gas(&self, CallRequest, Option<BlockNumber>) -> BoxFuture<U256>;

	/// Get transaction by its hash.
	#[rpc(name = "sof_getTransactionByHash")]
	fn transaction_by_hash(&self, H256) -> BoxFuture<Option<Transaction>>;

	/// Returns transaction at given block hash and index.
	#[rpc(name = "sof_getTransactionByBlockHashAndIndex")]
	fn transaction_by_block_hash_and_index(&self, H256, Index) -> BoxFuture<Option<Transaction>>;

	/// Returns transaction by given block number and index.
	#[rpc(name = "sof_getTransactionByBlockNumberAndIndex")]
	fn transaction_by_block_number_and_index(&self, BlockNumber, Index) -> BoxFuture<Option<Transaction>>;

	/// Returns transaction receipt by transaction hash.
	#[rpc(name = "sof_getTransactionReceipt")]
	fn transaction_receipt(&self, H256) -> BoxFuture<Option<Receipt>>;

	/// Returns an uncles at given block and index.
	#[rpc(name = "sof_getUncleByBlockHashAndIndex")]
	fn uncle_by_block_hash_and_index(&self, H256, Index) -> BoxFuture<Option<RichBlock>>;

	/// Returns an uncles at given block and index.
	#[rpc(name = "sof_getUncleByBlockNumberAndIndex")]
	fn uncle_by_block_number_and_index(&self, BlockNumber, Index) -> BoxFuture<Option<RichBlock>>;

	/// Returns available compilers.
	/// @deprecated
	#[rpc(name = "sof_getCompilers")]
	fn compilers(&self) -> Result<Vec<String>>;

	/// Compiles lll code.
	/// @deprecated
	#[rpc(name = "sof_compileLLL")]
	fn compile_lll(&self, String) -> Result<Bytes>;

	/// Compiles polynomial.
	/// @deprecated
	#[rpc(name = "sof_compilePolynomial")]
	fn compile_polynomial(&self, String) -> Result<Bytes>;

	/// Compiles serpent.
	/// @deprecated
	#[rpc(name = "sof_compileSerpent")]
	fn compile_serpent(&self, String) -> Result<Bytes>;

	/// Returns logs matching given filter object.
	#[rpc(name = "sof_getLogs")]
	fn logs(&self, Filter) -> BoxFuture<Vec<Log>>;

	/// Returns the hash of the current block, the seedHash, and the boundary condition to be met.
	#[rpc(name = "sof_getWork")]
	fn work(&self, Option<u64>) -> Result<Work>;

	/// Used for submitting a proof-of-work solution.
	#[rpc(name = "sof_submitWork")]
	fn submit_work(&self, H64, H256, H256) -> Result<bool>;

	/// Used for submitting mining hashrate.
	#[rpc(name = "sof_submitHashrate")]
	fn submit_hashrate(&self, U256, H256) -> Result<bool>;
}

/// Sof filters rpc api (polling).
// TODO: do filters api properly
#[rpc]
pub trait SofFilter {
	/// Returns id of new filter.
	#[rpc(name = "sof_newFilter")]
	fn new_filter(&self, Filter) -> Result<U256>;

	/// Returns id of new block filter.
	#[rpc(name = "sof_newBlockFilter")]
	fn new_block_filter(&self) -> Result<U256>;

	/// Returns id of new block filter.
	#[rpc(name = "sof_newPendingTransactionFilter")]
	fn new_pending_transaction_filter(&self) -> Result<U256>;

	/// Returns filter changes since last poll.
	#[rpc(name = "sof_getFilterChanges")]
	fn filter_changes(&self, Index) -> BoxFuture<FilterChanges>;

	/// Returns all logs matching given filter (in a range 'from' - 'to').
	#[rpc(name = "sof_getFilterLogs")]
	fn filter_logs(&self, Index) -> BoxFuture<Vec<Log>>;

	/// Uninstalls filter.
	#[rpc(name = "sof_uninstallFilter")]
	fn uninstall_filter(&self, Index) -> Result<bool>;
}
