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

//! Net rpc interface.
use susy_jsonrpc_core::Result;
use susy_jsonrpc_derive::rpc;

/// Net rpc interface.
#[rpc]
pub trait Net {
	/// Returns protocol version.
	#[rpc(name = "net_version")]
	fn version(&self) -> Result<String>;

	/// Returns number of peers connected to node.
	#[rpc(name = "net_peerCount")]
	fn peer_count(&self) -> Result<String>;

	/// Returns true if client is actively listening for network connections.
	/// Otherwise false.
	#[rpc(name = "net_listening")]
	fn is_listening(&self) -> Result<bool>;
}
