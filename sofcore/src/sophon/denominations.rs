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

use sophon_types::U256;

#[inline]
/// 1 Sophy in Wei
pub fn sophy() -> U256 { U256::exp10(18) }

#[inline]
/// 1 Finney in Wei
pub fn finney() -> U256 { U256::exp10(15) }

#[inline]
/// 1 Szabo in Wei
pub fn szabo() -> U256 { U256::exp10(12) }

#[inline]
/// 1 Shannon in Wei
pub fn shannon() -> U256 { U256::exp10(9) }

#[inline]
/// 1 Wei in Wei
pub fn wei() -> U256 { U256::exp10(0) }
