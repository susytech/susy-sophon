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

extern crate ansi_term;
extern crate sofcore;
extern crate sofcore_blockchain as blockchain;
extern crate sofcore_io as io;
extern crate sofcore_private_tx;
extern crate sofcore_sync as sync;
extern crate sophon_types;
extern crate susykv;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate trace_time;

#[cfg(test)]
extern crate sofcore_db;
#[cfg(test)]
extern crate tempdir;

mod error;
mod service;
mod stop_guard;

#[cfg(test)]
extern crate susykv_rocksdb;

pub use error::{Error, ErrorKind};
pub use service::{ClientService, PrivateTxService};
