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

//! Blockchain sync module
//! Implements sophon protocol version 63 as specified here:
//! https://octonion.institute/susy-go/wiki/wiki/Sophon-Wire-Protocol
//!

extern crate common_types as types;
extern crate sofcore;
extern crate sofcore_io as io;
extern crate sofcore_network as network;
extern crate sofcore_network_devp2p as devp2p;
extern crate sophon_types;
extern crate sofkey;
extern crate sofstore;
extern crate fastmap;
extern crate keccak_hash as hash;
extern crate susy_bytes as bytes;
extern crate parking_lot;
extern crate rand;
extern crate srlp;
extern crate triehash_sophon;

extern crate sofcore_light as light;

#[cfg(test)] extern crate env_logger;
#[cfg(test)] extern crate sofcore_private_tx;
#[cfg(test)] extern crate susykv_memorydb;
#[cfg(test)] extern crate rustc_hex;

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate macros;
#[macro_use]
extern crate log;
#[macro_use]
extern crate heapsize;
#[macro_use]
extern crate trace_time;

mod chain;
mod blocks;
mod block_sync;
mod sync_io;
mod private_tx;
mod snapshot;
mod transactions_stats;

pub mod light_sync;

#[cfg(test)]
mod tests;

mod api;

pub use api::*;
pub use chain::{SyncStatus, SyncState};
pub use devp2p::validate_node_url;
pub use network::{NonReservedPeerMode, Error, ErrorKind, ConnectionFilter, ConnectionDirection};
pub use private_tx::{PrivateTxHandler, NoopPrivateTxHandler, SimplePrivateTxHandler};
