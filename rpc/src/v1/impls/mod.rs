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

//! Sophon rpc interface implementation.

mod debug;
mod sof;
mod sof_filter;
mod sof_pubsub;
mod net;
mod susy;
#[cfg(any(test, feature = "accounts"))]
mod susy_accounts;
mod susy_set;
#[cfg(any(test, feature = "accounts"))]
mod personal;
mod private;
mod pubsub;
mod rpc;
#[cfg(any(test, feature = "accounts"))]
mod secretstore;
mod signer;
mod signing;
mod signing_unsafe;
mod traces;
mod susyweb;

pub mod light;

pub use self::debug::DebugClient;
pub use self::sof::{SofClient, SofClientOptions};
pub use self::sof_filter::SofFilterClient;
pub use self::sof_pubsub::SofPubSubClient;
pub use self::net::NetClient;
pub use self::susy::SusyClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::susy_accounts::SusyAccountsClient;
pub use self::susy_set::SusySetClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::susy_set::accounts::SusySetAccountsClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::personal::PersonalClient;
pub use self::private::PrivateClient;
pub use self::pubsub::PubSubClient;
pub use self::rpc::RpcClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::secretstore::SecretStoreClient;
pub use self::signer::SignerClient;
pub use self::signing::SigningQueueClient;
pub use self::signing_unsafe::SigningUnsafeClient;
pub use self::traces::TracesClient;
pub use self::susyweb::SusyWebClient;
