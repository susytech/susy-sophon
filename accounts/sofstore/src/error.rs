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

use std::fmt;
use std::io::Error as IoError;
use sofkey::{self, Error as SofKeyError};
use crypto::{self, Error as SofCryptoError};
use sofkey::DerivationError;

/// Account-related errors.
#[derive(Debug)]
pub enum Error {
	/// IO error
	Io(IoError),
	/// Invalid Password
	InvalidPassword,
	/// Account's secret is invalid.
	InvalidSecret,
	/// Invalid Vault Crypto meta.
	InvalidCryptoMeta,
	/// Invalid Account.
	InvalidAccount,
	/// Invalid Message.
	InvalidMessage,
	/// Invalid Key File
	InvalidKeyFile(String),
	/// Vaults are not supported.
	VaultsAreNotSupported,
	/// Unsupported vault
	UnsupportedVault,
	/// Invalid vault name
	InvalidVaultName,
	/// Vault not found
	VaultNotFound,
	/// Account creation failed.
	CreationFailed,
	/// `SofKey` error
	SofKey(SofKeyError),
	/// `sofkey::crypto::Error`
	SofKeyCrypto(sofkey::crypto::Error),
	/// `SofCrypto` error
	SofCrypto(SofCryptoError),
	/// Derivation error
	Derivation(DerivationError),
	/// Custom error
	Custom(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let s = match *self {
			Error::Io(ref err) => err.to_string(),
			Error::InvalidPassword => "Invalid password".into(),
			Error::InvalidSecret => "Invalid secret".into(),
			Error::InvalidCryptoMeta => "Invalid crypted metadata".into(),
			Error::InvalidAccount => "Invalid account".into(),
			Error::InvalidMessage => "Invalid message".into(),
			Error::InvalidKeyFile(ref reason) => format!("Invalid key file: {}", reason),
			Error::VaultsAreNotSupported => "Vaults are not supported".into(),
			Error::UnsupportedVault => "Vault is not supported for this operation".into(),
			Error::InvalidVaultName => "Invalid vault name".into(),
			Error::VaultNotFound => "Vault not found".into(),
			Error::CreationFailed => "Account creation failed".into(),
			Error::SofKey(ref err) => err.to_string(),
			Error::SofKeyCrypto(ref err) => err.to_string(),
			Error::SofCrypto(ref err) => err.to_string(),
			Error::Derivation(ref err) => format!("Derivation error: {:?}", err),
			Error::Custom(ref s) => s.clone(),
		};

		write!(f, "{}", s)
	}
}

impl From<IoError> for Error {
	fn from(err: IoError) -> Self {
		Error::Io(err)
	}
}

impl From<SofKeyError> for Error {
	fn from(err: SofKeyError) -> Self {
		Error::SofKey(err)
	}
}

impl From<sofkey::crypto::Error> for Error {
	fn from(err: sofkey::crypto::Error) -> Self {
		Error::SofKeyCrypto(err)
	}
}

impl From<SofCryptoError> for Error {
	fn from(err: SofCryptoError) -> Self {
		Error::SofCrypto(err)
	}
}

impl From<crypto::error::ScryptError> for Error {
	fn from(err: crypto::error::ScryptError) -> Self {
		Error::SofCrypto(err.into())
	}
}

impl From<crypto::error::SymmError> for Error {
	fn from(err: crypto::error::SymmError) -> Self {
		Error::SofCrypto(err.into())
	}
}

impl From<DerivationError> for Error {
	fn from(err: DerivationError) -> Self {
		Error::Derivation(err)
	}
}
