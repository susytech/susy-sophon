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

//! RPC implementations for the light client.
//!
//! This doesn't re-implement all of the RPC APIs, just those which aren't
//! significantly generic to be reused.

pub mod sof;
pub mod susy;
pub mod susy_set;
pub mod trace;
pub mod net;

pub use self::sof::SofClient;
pub use self::susy::SusyClient;
pub use self::susy_set::SusySetClient;
pub use self::net::NetClient;
pub use self::trace::TracesClient;
