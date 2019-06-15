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

//! Spec seal.

use srlp::RlpStream;
use sophon_types::{H64, H256, H520};
use sofjson;

/// Classic sophon seal.
pub struct Sophon {
	/// Seal nonce.
	pub nonce: H64,
	/// Seal mix hash.
	pub mix_hash: H256,
}

impl Into<Generic> for Sophon {
	fn into(self) -> Generic {
		let mut s = RlpStream::new_list(2);
		s.append(&self.mix_hash).append(&self.nonce);
		Generic(s.out())
	}
}

/// AuthorityRound seal.
pub struct AuthorityRound {
	/// Seal step.
	pub step: usize,
	/// Seal signature.
	pub signature: H520,
}

/// Tendermint seal.
pub struct Tendermint {
	/// Seal round.
	pub round: usize,
	/// Proposal seal signature.
	pub proposal: H520,
	/// Precommit seal signatures.
	pub precommits: Vec<H520>,
}

impl Into<Generic> for AuthorityRound {
	fn into(self) -> Generic {
		let mut s = RlpStream::new_list(2);
		s.append(&self.step).append(&self.signature);
		Generic(s.out())
	}
}

impl Into<Generic> for Tendermint {
	fn into(self) -> Generic {
		let mut stream = RlpStream::new_list(3);
		stream
			.append(&self.round)
			.append(&self.proposal)
			.append_list(&self.precommits);
		Generic(stream.out())
	}
}

pub struct Generic(pub Vec<u8>);

/// Genesis seal type.
pub enum Seal {
	/// Classic sophon seal.
	Sophon(Sophon),
	/// AuthorityRound seal.
	AuthorityRound(AuthorityRound),
	/// Tendermint seal.
	Tendermint(Tendermint),
	/// Generic SRLP seal.
	Generic(Generic),
}

impl From<sofjson::spec::Seal> for Seal {
	fn from(s: sofjson::spec::Seal) -> Self {
		match s {
			sofjson::spec::Seal::Sophon(sof) => Seal::Sophon(Sophon {
				nonce: sof.nonce.into(),
				mix_hash: sof.mix_hash.into()
			}),
			sofjson::spec::Seal::AuthorityRound(ar) => Seal::AuthorityRound(AuthorityRound {
				step: ar.step.into(),
				signature: ar.signature.into()
			}),
			sofjson::spec::Seal::Tendermint(tender) => Seal::Tendermint(Tendermint {
				round: tender.round.into(),
				proposal: tender.proposal.into(),
				precommits: tender.precommits.into_iter().map(Into::into).collect()
			}),
			sofjson::spec::Seal::Generic(g) => Seal::Generic(Generic(g.into())),
		}
	}
}

impl Into<Generic> for Seal {
	fn into(self) -> Generic {
		match self {
			Seal::Generic(generic) => generic,
			Seal::Sophon(sof) => sof.into(),
			Seal::AuthorityRound(ar) => ar.into(),
			Seal::Tendermint(tender) => tender.into(),
		}
	}
}
