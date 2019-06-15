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

//! Susy-specific PUB-SUB rpc interface.

use susy_jsonrpc_core::{Result, Value, Params};
use susy_jsonrpc_pubsub::{typed::Subscriber, SubscriptionId};
use susy_jsonrpc_derive::rpc;

/// Susy-specific PUB-SUB rpc interface.
#[rpc]
pub trait PubSub {
	/// Pub/Sub Metadata
	type Metadata;

	/// Subscribe to changes of any RPC method in Susy.
	#[pubsub(subscription = "susy_subscription", subscribe, name = "susy_subscribe")]
	fn susy_subscribe(&self, Self::Metadata, Subscriber<Value>, String, Option<Params>);

	/// Unsubscribe from existing Susy subscription.
	#[pubsub(subscription = "susy_subscription", unsubscribe, name = "susy_unsubscribe")]
	fn susy_unsubscribe(&self, Option<Self::Metadata>, SubscriptionId) -> Result<bool>;
}
