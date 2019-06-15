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

use std::path::Path;
use super::test_common::*;
use client::SvmTestClient;
use sofjson;
use srlp::Rlp;
use types::header::Header;
use types::transaction::UnverifiedTransaction;
use transaction_ext::Transaction;

/// Run transaction jsontests on a given folder.
pub fn run_test_path<H: FnMut(&str, HookType)>(p: &Path, skip: &[&'static str], h: &mut H) {
	::json_tests::test_common::run_test_path(p, skip, do_json_test, h)
}

/// Run transaction jsontests on a given file.
pub fn run_test_file<H: FnMut(&str, HookType)>(p: &Path, h: &mut H) {
	::json_tests::test_common::run_test_file(p, do_json_test, h)
}

// Block number used to run the tests.
// Make sure that all the specified features are activated.
const BLOCK_NUMBER: u64 = 0x6ffffffffffffe;

fn do_json_test<H: FnMut(&str, HookType)>(json_data: &[u8], start_stop_hook: &mut H) -> Vec<String> {
	let tests = sofjson::transaction::Test::load(json_data).unwrap();
	let mut failed = Vec::new();
	for (name, test) in tests.into_iter() {
		start_stop_hook(&name, HookType::OnStart);

		for (spec_name, result) in test.post_state {
			let spec = match SvmTestClient::spec_from_json(&spec_name) {
				Some(spec) => spec,
				None => {
					println!("   - {} | {:?} Ignoring tests because of missing spec", name, spec_name);
					continue;
				}
			};

			let mut fail_unless = |cond: bool, title: &str| if !cond {
				failed.push(format!("{}-{:?}", name, spec_name));
				println!("Transaction failed: {:?}-{:?}: {:?}", name, spec_name, title);
			};

			let srlp: Vec<u8> = test.srlp.clone().into();
			let res = Rlp::new(&srlp)
				.as_val()
				.map_err(::error::Error::from)
				.and_then(|t: UnverifiedTransaction| {
					let mut header: Header = Default::default();
					// Use high enough number to activate all required features.
					header.set_number(BLOCK_NUMBER);

					let minimal = t.gas_required(&spec.engine.schedule(header.number())).into();
					if t.gas < minimal {
						return Err(::types::transaction::Error::InsufficientGas {
							minimal, got: t.gas,
						}.into());
					}
					spec.engine.verify_transaction_basic(&t, &header)?;
					Ok(spec.engine.verify_transaction_unordered(t, &header)?)
				});

			match (res, result.hash, result.sender) {
				(Ok(t), Some(hash), Some(sender)) => {
					fail_unless(t.sender() == sender.into(), "sender mismatch");
					fail_unless(t.hash() == hash.into(), "hash mismatch");
				},
				(Err(_), None, None) => {},
				data => {
					fail_unless(
						false,
						&format!("Validity different: {:?}", data)
					);
				}
			}
		}

		start_stop_hook(&name, HookType::OnStop);
	}

	for f in &failed {
		println!("FAILED: {:?}", f);
	}
	failed
}

declare_test!{TransactionTests_ttAddress, "TransactionTests/ttAddress"}
declare_test!{TransactionTests_ttData, "TransactionTests/ttData"}
declare_test!{TransactionTests_ttGasLimit, "TransactionTests/ttGasLimit"}
declare_test!{TransactionTests_ttGasPrice, "TransactionTests/ttGasPrice"}
declare_test!{TransactionTests_ttNonce, "TransactionTests/ttNonce"}
declare_test!{TransactionTests_ttRSValue, "TransactionTests/ttRSValue"}
declare_test!{TransactionTests_ttSignature, "TransactionTests/ttSignature"}
declare_test!{TransactionTests_ttValue, "TransactionTests/ttValue"}
declare_test!{TransactionTests_ttVValue, "TransactionTests/ttVValue"}
declare_test!{TransactionTests_ttWrongSRLP, "TransactionTests/ttWrongSRLP"}
