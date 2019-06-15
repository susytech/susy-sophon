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

extern crate rustc_hex;
extern crate serde;
extern crate serde_json;
extern crate sophon_types;
#[macro_use] extern crate serde_derive;

pub mod hash;
pub mod uint;
pub mod bytes;
pub mod blockchain;
pub mod spec;
pub mod trie;
pub mod vm;
pub mod maybe;
pub mod state;
pub mod transaction;
pub mod test;
