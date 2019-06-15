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

//! Svm factory.
//!
use std::sync::Arc;
use vm::{Exec, Schedule};
use sophon_types::U256;
use super::vm::ActionParams;
use super::interpreter::SharedCache;
use super::vmtype::VMType;

/// Svm factory. Creates appropriate Svm.
#[derive(Clone)]
pub struct Factory {
	svm: VMType,
	svm_cache: Arc<SharedCache>,
}

impl Factory {
	/// Create fresh instance of VM
	/// Might choose implementation depending on supplied gas.
	pub fn create(&self, params: ActionParams, schedule: &Schedule, depth: usize) -> Box<Exec> {
		match self.svm {
			VMType::Interpreter => if Self::can_fit_in_usize(&params.gas) {
				Box::new(super::interpreter::Interpreter::<usize>::new(params, self.svm_cache.clone(), schedule, depth))
			} else {
				Box::new(super::interpreter::Interpreter::<U256>::new(params, self.svm_cache.clone(), schedule, depth))
			}
		}
	}

	/// Create new instance of specific `VMType` factory, with a size in bytes
	/// for caching jump destinations.
	pub fn new(svm: VMType, cache_size: usize) -> Self {
		Factory {
			svm: svm,
			svm_cache: Arc::new(SharedCache::new(cache_size)),
		}
	}

	fn can_fit_in_usize(gas: &U256) -> bool {
		gas == &U256::from(gas.low_u64() as usize)
	}
}

impl Default for Factory {
	/// Returns native rust svm factory
	fn default() -> Factory {
		Factory {
			svm: VMType::Interpreter,
			svm_cache: Arc::new(SharedCache::default()),
		}
	}
}

#[test]
fn test_create_vm() {
	use vm::Ext;
	use vm::tests::FakeExt;
	use bytes::Bytes;

	let mut params = ActionParams::default();
	params.code = Some(Arc::new(Bytes::default()));
	let ext = FakeExt::new();
	let _vm = Factory::default().create(params, ext.schedule(), ext.depth());
}

/// Create tests by injecting different VM factories
#[macro_export]
macro_rules! svm_test(
	($name_test: ident: $name_int: ident) => {
		#[test]
		fn $name_int() {
			$name_test(Factory::new(VMType::Interpreter, 1024 * 32));
		}
	}
);

/// Create ignored tests by injecting different VM factories
#[macro_export]
macro_rules! svm_test_ignore(
	($name_test: ident: $name_int: ident) => {
		#[test]
		#[ignore]
		#[cfg(feature = "ignored-tests")]
		fn $name_int() {
			$name_test(Factory::new(VMType::Interpreter, 1024 * 32));
		}
	}
);
