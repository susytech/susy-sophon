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

//! Light client logic and implementation.
//!
//! A "light" client stores very little chain-related data locally
//! unlike a full node, which stores all blocks, headers, receipts, and more.
//!
//! This enables the client to have a much lower resource footprint in
//! exchange for the cost of having to ask the network for state data
//! while responding to queries. This makes a light client unsuitable for
//! low-latency applications, but perfectly suitable for simple everyday
//! use-cases like sending transactions from a personal account.
//!
//! The light client performs a header-only sync, doing verification and pruning
//! historical blocks. Upon pruning, batches of 2048 blocks have a number => (hash, TD)
//! mapping sealed into "canonical hash tries" which can later be used to verify
//! historical block queries from peers.

#![deny(missing_docs)]

pub mod client;
pub mod cht;
pub mod net;
pub mod on_demand;
pub mod transaction_queue;
pub mod cache;
pub mod provider;

mod types;

pub use self::cache::Cache;
pub use self::provider::{Provider, MAX_HEADERS_PER_REQUEST};
pub use self::transaction_queue::TransactionQueue;
pub use types::request as request;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate bincode;
extern crate common_types;
extern crate sofcore_blockchain;
extern crate sofcore_db;
extern crate sofcore_io as io;
extern crate sofcore_network as network;
extern crate susy_bytes as bytes;
extern crate sophon_types;
extern crate sofcore;
extern crate hash_db;
extern crate heapsize;
extern crate failsafe;
extern crate futures;
extern crate itertools;
extern crate keccak_hasher;
extern crate memory_db;
extern crate trie_db as trie;
extern crate susy_patricia_trie_sophon as softrie;
extern crate fastmap;
extern crate rand;
extern crate srlp;
extern crate parking_lot;
#[macro_use]
extern crate srlp_derive;
extern crate serde;
extern crate smallvec;
extern crate stats;
extern crate vm;
extern crate keccak_hash as hash;
extern crate triehash_sophon as triehash;
extern crate susykv;
extern crate memory_cache;
#[macro_use]
extern crate error_chain;

#[cfg(test)]
extern crate susykv_memorydb;
#[cfg(test)]
extern crate tempdir;
extern crate journaldb;
