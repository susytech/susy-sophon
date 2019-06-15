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

//! SIP-191 compliant decoding + hashing
use v1::types::{SIP191Version, Bytes, PresignedTransaction};
use sip_712::{hash_structured_data, SIP712};
use serde_json::{Value, from_value};
use v1::helpers::errors;
use susy_jsonrpc_core::Error;
use v1::helpers::dispatch::sof_data_hash;
use hash::keccak;
use std::fmt::Display;
use sophon_types::H256;

/// deserializes and hashes the message depending on the version specifier
pub fn hash_message(version: SIP191Version, message: Value) -> Result<H256, Error> {
	let data = match version {
		SIP191Version::StructuredData => {
			let typed_data = from_value::<SIP712>(message)
				.map_err(map_serde_err("StructuredData"))?;

			hash_structured_data(typed_data)
				.map_err(|err| errors::invalid_call_data(err.kind()))?
		}

		SIP191Version::PresignedTransaction => {
			let data = from_value::<PresignedTransaction>(message)
				.map_err(map_serde_err("WithValidator"))?;
			let prefix = b"\x19\x00";
			let data = [&prefix[..], &data.validator.0[..], &data.data.0[..]].concat();
			keccak(data)
		}

		SIP191Version::PersonalMessage => {
			let bytes = from_value::<Bytes>(message)
				.map_err(map_serde_err("Bytes"))?;
			sof_data_hash(bytes.0)
		}
	};

	Ok(data)
}

fn map_serde_err<T: Display>(struct_name: &'static str) -> impl Fn(T) -> Error {
	move |error: T| {
		errors::invalid_call_data(format!("Error deserializing '{}': {}", struct_name, error))
	}
}
