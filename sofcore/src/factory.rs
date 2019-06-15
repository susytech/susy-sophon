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

use trie::TrieFactory;
use softrie::RlpCodec;
use account_db::Factory as AccountFactory;
use svm::{Factory as SvmFactory, VMType};
use vm::{Exec, ActionParams, Schedule};
use wasm::WasmInterpreter;
use keccak_hasher::KeccakHasher;

const WASM_MAGIC_NUMBER: &'static [u8; 4] = b"\0asm";

/// Virtual machine factory
#[derive(Default, Clone)]
pub struct VmFactory {
	svm: SvmFactory,
}

impl VmFactory {
	pub fn create(&self, params: ActionParams, schedule: &Schedule, depth: usize) -> Box<Exec> {
		if schedule.wasm.is_some() && params.code.as_ref().map_or(false, |code| code.len() > 4 && &code[0..4] == WASM_MAGIC_NUMBER) {
			Box::new(WasmInterpreter::new(params))
		} else {
			self.svm.create(params, schedule, depth)
		}
	}

	pub fn new(svm: VMType, cache_size: usize) -> Self {
		VmFactory { svm: SvmFactory::new(svm, cache_size) }
	}
}

impl From<SvmFactory> for VmFactory {
	fn from(svm: SvmFactory) -> Self {
		VmFactory { svm: svm }
	}
}

/// Collection of factories.
#[derive(Default, Clone)]
pub struct Factories {
	/// factory for svm.
	pub vm: VmFactory,
	/// factory for tries.
	pub trie: TrieFactory<KeccakHasher, RlpCodec>,
	/// factory for account databases.
	pub accountdb: AccountFactory,
}
