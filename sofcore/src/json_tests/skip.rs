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

//! State tests to skip.

use sofjson;

#[cfg(feature="ci-skip-tests")]
lazy_static!{
	pub static ref SKIP_TEST_STATE: sofjson::test::SkipStates = {
		let skip_data = include_bytes!("../../res/sophon/tests-issues/currents.json");
		sofjson::test::SkipStates::load(&skip_data[..]).expect("No invalid json allowed")
	};
}

#[cfg(not(feature="ci-skip-tests"))]
lazy_static!{
	pub static ref SKIP_TEST_STATE: sofjson::test::SkipStates = {
		sofjson::test::SkipStates::empty()
	};
}
