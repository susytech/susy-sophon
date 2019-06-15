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

//! Traces config.

/// Traces config.
#[derive(Debug, PartialEq, Clone)]
pub struct Config {
	/// Indicates if tracing should be enabled or not.
	/// If it's None, it will be automatically configured.
	pub enabled: bool,
	/// Preferef cache-size.
	pub pref_cache_size: usize,
	/// Max cache-size.
	pub max_cache_size: usize,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			enabled: false,
			pref_cache_size: 15 * 1024 * 1024,
			max_cache_size: 20 * 1024 * 1024,
		}
	}
}
