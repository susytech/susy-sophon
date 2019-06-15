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

//! Hash-addressed content resolver & fetcher.

#![warn(missing_docs)]

#[macro_use]
extern crate log;

extern crate sofabi;
extern crate susy_bytes as bytes;
extern crate sophon_types;
extern crate futures;
extern crate keccak_hash as hash;
extern crate mime;
extern crate mime_guess;
extern crate susy_runtime;
extern crate rand;
extern crate rustc_hex;
extern crate registrar;

pub extern crate fetch;

#[macro_use]
extern crate sofabi_derive;
#[macro_use]
extern crate sofabi_contract;
#[cfg(test)]
extern crate parking_lot;
#[cfg(test)]
extern crate fake_fetch;

mod client;

pub mod urlhint;

pub use client::{HashFetch, Client, Error};
pub use fetch::Abort;
