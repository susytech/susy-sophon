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

//! RPC interface.

use std::collections::BTreeMap;

use susy_jsonrpc_core::Result;
use susy_jsonrpc_derive::rpc;

/// RPC Interface.
#[rpc]
pub trait Rpc {
	/// Returns supported modules for Graviton 1.3.6
	/// @ignore
	#[rpc(name = "modules")]
	fn modules(&self) -> Result<BTreeMap<String, String>>;

	/// Returns supported modules for Graviton 1.4.0
	/// @ignore
	#[rpc(name = "rpc_modules")]
	fn rpc_modules(&self) -> Result<BTreeMap<String, String>>;
}
