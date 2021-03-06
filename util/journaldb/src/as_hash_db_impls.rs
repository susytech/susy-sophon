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

//! Impls of the `AsHashDB` upcast trait for all different variants of DB
use hash_db::{HashDB, AsHashDB};
use keccak_hasher::KeccakHasher;
use archivedb::ArchiveDB;
use earlymergedb::EarlyMergeDB;
use overlayrecentdb::OverlayRecentDB;
use refcounteddb::RefCountedDB;
use overlaydb::OverlayDB;
use susykv::DBValue;
use crate::{KeyedHashDB, AsKeyedHashDB};

impl AsHashDB<KeccakHasher, DBValue> for ArchiveDB {
	fn as_hash_db(&self) -> &HashDB<KeccakHasher, DBValue> { self }
	fn as_hash_db_mut(&mut self) -> &mut HashDB<KeccakHasher, DBValue> { self }
}

impl AsHashDB<KeccakHasher, DBValue> for EarlyMergeDB {
	fn as_hash_db(&self) -> &HashDB<KeccakHasher, DBValue> { self }
	fn as_hash_db_mut(&mut self) -> &mut HashDB<KeccakHasher, DBValue> { self }
}

impl AsHashDB<KeccakHasher, DBValue> for OverlayRecentDB {
	fn as_hash_db(&self) -> &HashDB<KeccakHasher, DBValue> { self }
	fn as_hash_db_mut(&mut self) -> &mut HashDB<KeccakHasher, DBValue> { self }
}

impl AsHashDB<KeccakHasher, DBValue> for RefCountedDB {
	fn as_hash_db(&self) -> &HashDB<KeccakHasher, DBValue> { self }
	fn as_hash_db_mut(&mut self) -> &mut HashDB<KeccakHasher, DBValue> { self }
}

impl AsHashDB<KeccakHasher, DBValue> for OverlayDB {
	fn as_hash_db(&self) -> &HashDB<KeccakHasher, DBValue> { self }
	fn as_hash_db_mut(&mut self) -> &mut HashDB<KeccakHasher, DBValue> { self }
}

impl AsKeyedHashDB for ArchiveDB {
	fn as_keyed_hash_db(&self) -> &KeyedHashDB { self }
}

impl AsKeyedHashDB for EarlyMergeDB {
	fn as_keyed_hash_db(&self) -> &KeyedHashDB { self }
}

impl AsKeyedHashDB for OverlayRecentDB {
	fn as_keyed_hash_db(&self) -> &KeyedHashDB { self }
}

impl AsKeyedHashDB for RefCountedDB {
	fn as_keyed_hash_db(&self) -> &KeyedHashDB { self }
}

impl AsKeyedHashDB for OverlayDB {
	fn as_keyed_hash_db(&self) -> &KeyedHashDB { self }
}
