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

//! Sof PUB-SUB rpc interface.

use susy_jsonrpc_core::Result;
use susy_jsonrpc_derive::rpc;
use susy_jsonrpc_pubsub::{typed, SubscriptionId};

use v1::types::pubsub;

/// Sof PUB-SUB rpc interface.
#[rpc]
pub trait SofPubSub {
	/// RPC Metadata
	type Metadata;

	/// Subscribe to Sof subscription.
	#[pubsub(subscription = "sof_subscription", subscribe, name = "sof_subscribe")]
	fn subscribe(&self, Self::Metadata, typed::Subscriber<pubsub::Result>, pubsub::Kind, Option<pubsub::Params>);

	/// Unsubscribe from existing Sof subscription.
	#[pubsub(subscription = "sof_subscription", unsubscribe, name = "sof_unsubscribe")]
	fn unsubscribe(&self, Option<Self::Metadata>, SubscriptionId) -> Result<bool>;
}
