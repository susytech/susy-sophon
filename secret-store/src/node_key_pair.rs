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

use sofkey::crypto::ecdh::agree;
use sofkey::{KeyPair, Public, Signature, Error as SofKeyError, sign, public_to_address};
use sophon_types::{H256, Address};
use traits::NodeKeyPair;

pub struct PlainNodeKeyPair {
	key_pair: KeyPair,
}

impl PlainNodeKeyPair {
	pub fn new(key_pair: KeyPair) -> Self {
		PlainNodeKeyPair {
			key_pair: key_pair,
		}
	}

	#[cfg(test)]
	pub fn key_pair(&self) -> &KeyPair {
		&self.key_pair
	}
}

impl NodeKeyPair for PlainNodeKeyPair {
	fn public(&self) -> &Public {
		self.key_pair.public()
	}

	fn address(&self) -> Address {
		public_to_address(self.key_pair.public())
	}

	fn sign(&self, data: &H256) -> Result<Signature, SofKeyError> {
		sign(self.key_pair.secret(), data)
	}

	fn compute_shared_key(&self, peer_public: &Public) -> Result<KeyPair, SofKeyError> {
		agree(self.key_pair.secret(), peer_public)
			.map_err(|e| SofKeyError::Custom(e.to_string()))
			.and_then(KeyPair::from_secret)
	}
}

#[cfg(feature = "accounts")]
mod accounts {
	use super::*;
	use std::sync::Arc;
	use sofkey::Password;
	use accounts::AccountProvider;

	pub struct KeyStoreNodeKeyPair {
		account_provider: Arc<AccountProvider>,
		address: Address,
		public: Public,
		password: Password,
	}

	impl KeyStoreNodeKeyPair {
		pub fn new(account_provider: Arc<AccountProvider>, address: Address, password: Password) -> Result<Self, SofKeyError> {
			let public = account_provider.account_public(address.clone(), &password).map_err(|e| SofKeyError::Custom(format!("{}", e)))?;
			Ok(KeyStoreNodeKeyPair {
				account_provider: account_provider,
				address: address,
				public: public,
				password: password,
			})
		}
	}

	impl NodeKeyPair for KeyStoreNodeKeyPair {
		fn public(&self) -> &Public {
			&self.public
		}

		fn address(&self) -> Address {
			public_to_address(&self.public)
		}

		fn sign(&self, data: &H256) -> Result<Signature, SofKeyError> {
			self.account_provider.sign(self.address.clone(), Some(self.password.clone()), data.clone())
				.map_err(|e| SofKeyError::Custom(format!("{}", e)))
		}

		fn compute_shared_key(&self, peer_public: &Public) -> Result<KeyPair, SofKeyError> {
			KeyPair::from_secret(self.account_provider.agree(self.address.clone(), Some(self.password.clone()), peer_public)
				.map_err(|e| SofKeyError::Custom(format!("{}", e)))?)
		}
	}
}

#[cfg(feature = "accounts")]
pub use self::accounts::KeyStoreNodeKeyPair;
