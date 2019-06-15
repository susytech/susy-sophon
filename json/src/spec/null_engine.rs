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

//! Null engine params deserialization.

use uint::Uint;

/// Authority params deserialization.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct NullEngineParams {
	/// Block reward.
	pub block_reward: Option<Uint>,
}

/// Null engine descriptor
#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NullEngine {
	/// Sofash params.
	pub params: NullEngineParams,
}

#[cfg(test)]
mod tests {
	use serde_json;
	use uint::Uint;
	use sophon_types::U256;
	use super::*;

	#[test]
	fn null_engine_deserialization() {
		let s = r#"{
			"params": {
				"blockReward": "0x0d"
			}
		}"#;

		let deserialized: NullEngine = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized.params.block_reward, Some(Uint(U256::from(0x0d))));
	}
}
