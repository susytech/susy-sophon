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

//! No-op verifier.

use call_contract::CallContract;
use client::BlockInfo;
use engines::SofEngine;
use error::Error;
use types::header::Header;
use super::{verification, Verifier};

/// A no-op verifier -- this will verify everything it's given immediately.
#[allow(dead_code)]
pub struct NoopVerifier;

impl<C: BlockInfo + CallContract> Verifier<C> for NoopVerifier {
	fn verify_block_family(
		&self,
		_: &Header,
		_t: &Header,
		_: &SofEngine,
		_: Option<verification::FullFamilyParams<C>>
	) -> Result<(), Error> {
		Ok(())
	}

	fn verify_block_final(&self, _expected: &Header, _got: &Header) -> Result<(), Error> {
		Ok(())
	}

	fn verify_block_external(&self, _header: &Header, _engine: &SofEngine) -> Result<(), Error> {
		Ok(())
	}
}
