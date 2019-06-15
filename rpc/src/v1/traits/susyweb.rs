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

//! SusyWeb rpc interface.
use sophon_types::H256;
use susy_jsonrpc_core::Result;
use susy_jsonrpc_derive::rpc;

use v1::types::Bytes;

/// SusyWeb rpc interface.
#[rpc]
pub trait SusyWeb {
	/// Returns current client version.
	#[rpc(name = "susyweb_clientVersion")]
	fn client_version(&self) -> Result<String>;

	/// Returns sha3 of the given data
	#[rpc(name = "susyweb_sha3")]
	fn sha3(&self, Bytes) -> Result<H256>;
}
