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

//! Localized traces type definitions

use sophon_types::H256;
use super::trace::{Action, Res};
use types::BlockNumber;

/// Localized trace.
#[derive(Debug, PartialEq, Clone)]
pub struct LocalizedTrace {
	/// Type of action performed by a transaction.
	pub action: Action,
	/// Result of this action.
	pub result: Res,
	/// Number of subtraces.
	pub subtraces: usize,
	/// Exact location of trace.
	///
	/// [index in root, index in first CALL, index in second CALL, ...]
	pub trace_address: Vec<usize>,
	/// Transaction number within the block.
	pub transaction_number: Option<usize>,
	/// Signed transaction hash.
	pub transaction_hash: Option<H256>,
	/// Block number.
	pub block_number: BlockNumber,
	/// Block hash.
	pub block_hash: H256,
}
