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

use keccak::Keccak256;
use super::{KeyPair, Generator, Secret};
use susy_wordlist;

/// Simple brainwallet.
pub struct Brain(String);

impl Brain {
	pub fn new(s: String) -> Self {
		Brain(s)
	}

	pub fn validate_phrase(phrase: &str, expected_words: usize) -> Result<(), ::WordlistError> {
		susy_wordlist::validate_phrase(phrase, expected_words)
	}
}

impl Generator for Brain {
    type Error = ::Void;

	fn generate(&mut self) -> Result<KeyPair, Self::Error> {
		let seed = self.0.clone();
		let mut secret = seed.into_bytes().keccak256();

		let mut i = 0;
		loop {
			secret = secret.keccak256();

			match i > 16384 {
				false => i += 1,
				true => {
					if let Ok(pair) = Secret::from_unsafe_slice(&secret)
						.and_then(KeyPair::from_secret)
					{
						if pair.address()[0] == 0 {
							trace!("Testing: {}, got: {:?}", self.0, pair.address());
							return Ok(pair)
						}
					}
				},
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use {Brain, Generator};

	#[test]
	fn test_brain() {
		let words = "this is sparta!".to_owned();
		let first_keypair = Brain::new(words.clone()).generate().unwrap();
		let second_keypair = Brain::new(words.clone()).generate().unwrap();
		assert_eq!(first_keypair.secret(), second_keypair.secret());
	}
}
