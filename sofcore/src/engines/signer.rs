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

//! A signer used by Engines which need to sign messages.

use sophon_types::{H256, Address};
use sofkey::{self, Signature};

/// Everything that an Engine needs to sign messages.
pub trait EngineSigner: Send + Sync {
	/// Sign a consensus message hash.
	fn sign(&self, hash: H256) -> Result<Signature, sofkey::Error>;

	/// Signing address
	fn address(&self) -> Address;
}

/// Creates a new `EngineSigner` from given key pair.
pub fn from_keypair(keypair: sofkey::KeyPair) -> Box<EngineSigner> {
	Box::new(Signer(keypair))
}

struct Signer(sofkey::KeyPair);

impl EngineSigner for Signer {
	fn sign(&self, hash: H256) -> Result<Signature, sofkey::Error> {
		sofkey::sign(self.0.secret(), &hash)
	}

	fn address(&self) -> Address {
		self.0.address()
	}
}

#[cfg(test)]
mod test_signer {
	use std::sync::Arc;

	use sofkey::Password;
	use accounts::{self, AccountProvider, SignError};

	use super::*;

	impl EngineSigner for (Arc<AccountProvider>, Address, Password) {
		fn sign(&self, hash: H256) -> Result<Signature, sofkey::Error> {
			match self.0.sign(self.1, Some(self.2.clone()), hash) {
				Err(SignError::NotUnlocked) => unreachable!(),
				Err(SignError::NotFound) => Err(sofkey::Error::InvalidAddress),
				Err(SignError::Hardware(err)) => {
					warn!("Error using hardware wallet for engine: {:?}", err);
					Err(sofkey::Error::InvalidSecret)
				},
				Err(SignError::SStore(accounts::Error::SofKey(err))) => Err(err),
				Err(SignError::SStore(accounts::Error::SofKeyCrypto(err))) => {
					warn!("Low level crypto error: {:?}", err);
					Err(sofkey::Error::InvalidSecret)
				},
				Err(SignError::SStore(err)) => {
					warn!("Error signing for engine: {:?}", err);
					Err(sofkey::Error::InvalidSignature)
				},
				Ok(ok) => Ok(ok),
			}
		}

		fn address(&self) -> Address {
			self.1
		}
	}
}
