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

#![warn(missing_docs)]

//! Miner module
//! Keeps track of transactions and mined block.

extern crate ansi_term;
extern crate common_types as types;
extern crate sofabi;
extern crate sofcore_call_contract as call_contract;
extern crate sophon_types;
extern crate futures;
extern crate heapsize;
extern crate keccak_hash as hash;
extern crate linked_hash_map;
extern crate susy_runtime;
extern crate parking_lot;
#[cfg(feature = "price-info")]
extern crate price_info;
extern crate srlp;
extern crate transaction_pool as txpool;

#[macro_use]
extern crate sofabi_contract;
#[macro_use]
extern crate sofabi_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate trace_time;

#[cfg(test)]
extern crate rustc_hex;
#[cfg(test)]
extern crate sofkey;
#[cfg(test)]
extern crate env_logger;

pub mod external;
#[cfg(feature = "price-info")]
pub mod gas_price_calibrator;
pub mod gas_pricer;
pub mod local_accounts;
pub mod pool;
pub mod service_transaction_checker;
#[cfg(feature = "work-notify")]
pub mod work_notify;
