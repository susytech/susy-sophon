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

//! SusyWeb rpc implementation.
use sophon_types::H256;
use hash::keccak;
use susy_jsonrpc_core::Result;
use version::version;
use v1::traits::SusyWeb;
use v1::types::Bytes;

/// SusyWeb rpc implementation.
pub struct SusyWebClient;

impl SusyWebClient {
	/// Creates new SusyWebClient.
	pub fn new() -> Self { SusyWebClient }
}

impl SusyWeb for SusyWebClient {
	fn client_version(&self) -> Result<String> {
		Ok(version().to_owned().replacen("/", "//", 1))
	}

	fn sha3(&self, data: Bytes) -> Result<H256> {
		Ok(keccak(&data.0).into())
	}
}
