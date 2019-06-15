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

//! State diff module.

use std::fmt;
use std::ops::*;
use std::collections::BTreeMap;
use sophon_types::Address;
use account_diff::*;

/// Expression for the delta between two system states. Encoded the
/// delta of every altered account.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StateDiff {
	/// Raw diff key-value
	pub raw: BTreeMap<Address, AccountDiff>
}

impl StateDiff {
	/// Get the actual data.
	pub fn get(&self) -> &BTreeMap<Address, AccountDiff> {
		&self.raw
	}
}

impl fmt::Display for StateDiff {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for (add, acc) in &self.raw {
			write!(f, "{} {}: {}", acc.existance(), add, acc)?;
		}
		Ok(())
	}
}

impl Deref for StateDiff {
	type Target = BTreeMap<Address, AccountDiff>;

	fn deref(&self) -> &Self::Target {
		&self.raw
	}
}
