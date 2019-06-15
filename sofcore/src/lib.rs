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

#![warn(missing_docs, unused_extern_crates)]
#![cfg_attr(feature = "time_checked_add", feature(time_checked_add))]

//! Sofcore library
//!
//! ### Rust version:
//! - nightly
//!
//! ### Supported platforms:
//! - OSX
//! - Linux
//!
//! ### Building:
//!
//! - Ubuntu 14.04 and later:
//!
//!   ```bash
//!
//!   # install rustup
//!   curl https://sh.rustup.rs -sSf | sh
//!
//!   # download and build susy
//!   git clone https://octonion.institute/susytech/susy-sophon
//!   cd susy
//!   cargo build --release
//!   ```
//!
//! - OSX:
//!
//!   ```bash
//!   # install rocksdb && rustup
//!   brew update
//!   curl https://sh.rustup.rs -sSf | sh
//!
//!   # download and build susy
//!   git clone https://octonion.institute/susytech/susy-sophon
//!   cd susy
//!   cargo build --release
//!   ```

// Recursion limit required because of
// error_chain foreign_links.
#![recursion_limit="128"]

extern crate ansi_term;
extern crate bn;
extern crate byteorder;
extern crate common_types as types;
extern crate crossbeam;
extern crate sofabi;
extern crate sofash;
extern crate sofcore_blockchain as blockchain;
extern crate sofcore_bloom_journal as bloom_journal;
extern crate sofcore_call_contract as call_contract;
extern crate sofcore_db as db;
extern crate sofcore_io as io;
extern crate sofcore_miner;
extern crate sophon_types;
extern crate sofjson;
extern crate sofkey;
extern crate hash_db;
extern crate heapsize;
extern crate itertools;
extern crate journaldb;
extern crate keccak_hash as hash;
extern crate keccak_hasher;
extern crate susykv;
extern crate susykv_memorydb;
extern crate len_caching_lock;
extern crate lru_cache;
extern crate memory_cache;
extern crate memory_db;
extern crate num;
extern crate num_cpus;
extern crate susy_bytes as bytes;
extern crate susy_crypto;
extern crate susy_machine;
extern crate susy_snappy as snappy;
extern crate parking_lot;
extern crate trie_db as trie;
extern crate susy_patricia_trie_sophon as softrie;
extern crate rand;
extern crate rayon;
extern crate srlp;
extern crate rustc_hex;
extern crate serde;
extern crate stats;
extern crate triehash_sophon as triehash;
extern crate unexpected;
extern crate using_queue;
extern crate vm;
extern crate wasm;

#[cfg(test)]
extern crate sofcore_accounts as accounts;
#[cfg(feature = "stratum")]
extern crate sofcore_stratum;
#[cfg(any(test, feature = "tempdir"))]
extern crate tempdir;
#[cfg(any(test, feature = "susykv-rocksdb"))]
extern crate susykv_rocksdb;
#[cfg(any(test, feature = "blooms-db"))]
extern crate blooms_db;
#[cfg(any(test, feature = "env_logger"))]
extern crate env_logger;
#[cfg(test)]
extern crate srlp_compress;

#[macro_use]
extern crate sofabi_derive;
#[macro_use]
extern crate sofabi_contract;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate macros;
#[macro_use]
extern crate srlp_derive;
#[macro_use]
extern crate trace_time;
#[macro_use]
extern crate serde_derive;

#[cfg_attr(test, macro_use)]
extern crate svm;

#[cfg(all(test, feature = "price-info"))]
extern crate fetch;

#[cfg(all(test, feature = "price-info"))]
extern crate susy_runtime;

#[cfg(not(time_checked_add))]
extern crate time_utils;

pub mod block;
pub mod builtin;
pub mod client;
pub mod engines;
pub mod error;
pub mod sophon;
pub mod executed;
pub mod executive;
pub mod machine;
pub mod miner;
pub mod pod_state;
pub mod pod_account;
pub mod snapshot;
pub mod spec;
pub mod state;
pub mod state_db;
pub mod trace;
pub mod transaction_ext;
pub mod verification;

mod account_db;
mod externalities;
mod factory;
mod tx_filter;

#[cfg(test)]
mod tests;
#[cfg(feature = "json-tests")]
pub mod json_tests;
#[cfg(any(test, feature = "test-helpers"))]
pub mod test_helpers;

pub use executive::contract_address;
pub use svm::CreateContractAddress;
pub use trie::TrieSpec;
