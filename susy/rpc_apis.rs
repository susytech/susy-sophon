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

use std::cmp::PartialEq;
use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;
use std::sync::{Arc, Weak};

pub use susy_rpc::signer::SignerService;

use account_utils::{self, AccountProvider};
use sofcore::client::Client;
use sofcore::miner::Miner;
use sofcore::snapshot::SnapshotService;
use sofcore_logger::RotatingLogger;
use sofcore_private_tx::Provider as PrivateTransactionManager;
use sofcore_service::PrivateTxService;
use hash_fetch::fetch::Client as FetchClient;
use susy_jsonrpc_core::{self as core, MetaIoHandler};
use light::client::LightChainClient;
use light::{Cache as LightDataCache, TransactionQueue as LightTransactionQueue};
use miner::external::ExternalMiner;
use susy_rpc::dispatch::{FullDispatcher, LightDispatcher};
use susy_rpc::informant::{ActivityNotifier, ClientNotifier};
use susy_rpc::{Host, Metadata, NetworkSettings};
use susy_runtime::Executor;
use parking_lot::{Mutex, RwLock};
use sync::{LightSync, ManageNetwork, SyncProvider};
use updater::Updater;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Api {
	/// SusyWeb (Safe)
	SusyWeb,
	/// Net (Safe)
	Net,
	/// Sof (Safe)
	Sof,
	/// Sof Pub-Sub (Safe)
	SofPubSub,
	/// Graviton-compatible "personal" API (DEPRECATED; only used in `--graviton` mode.)
	Personal,
	/// Signer - Confirm transactions in Signer (UNSAFE: Passwords, List of transactions)
	Signer,
	/// Susy - Custom extensions (Safe)
	Susy,
	/// Traces (Safe)
	Traces,
	/// Rpc (Safe)
	Rpc,
	/// Private transaction manager (Safe)
	Private,
	/// Whisper (Safe)
	// TODO: _if_ someone guesses someone else's key or filter IDs they can remove
	// BUT these are all ephemeral so it seems fine.
	Whisper,
	/// Whisper Pub-Sub (Safe but same concerns as above).
	WhisperPubSub,
	/// Susy PubSub - Generic Publish-Subscriber (Safety depends on other APIs exposed).
	SusyPubSub,
	/// Susy Accounts extensions (UNSAFE: Passwords, Side Effects (new account))
	SusyAccounts,
	/// Susy - Set methods (UNSAFE: Side Effects affecting node operation)
	SusySet,
	/// SecretStore (UNSAFE: arbitrary hash signing)
	SecretStore,
	/// Graviton-compatible (best-effort) debug API (Potentially UNSAFE)
	/// NOTE We don't aim to support all methods, only the ones that are useful.
	Debug,
}

impl FromStr for Api {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use self::Api::*;

		match s {
			"debug" => Ok(Debug),
			"sof" => Ok(Sof),
			"net" => Ok(Net),
			"susy" => Ok(Susy),
			"susy_accounts" => Ok(SusyAccounts),
			"susy_pubsub" => Ok(SusyPubSub),
			"susy_set" => Ok(SusySet),
			"personal" => Ok(Personal),
			"private" => Ok(Private),
			"pubsub" => Ok(SofPubSub),
			"rpc" => Ok(Rpc),
			"secretstore" => Ok(SecretStore),
			"shh" => Ok(Whisper),
			"shh_pubsub" => Ok(WhisperPubSub),
			"signer" => Ok(Signer),
			"traces" => Ok(Traces),
			"susyweb" => Ok(SusyWeb),
			api => Err(format!("Unknown api: {}", api)),
		}
	}
}

#[derive(Debug, Clone)]
pub enum ApiSet {
	// Unsafe context (like jsonrpc over http)
	UnsafeContext,
	// All possible APIs (safe context like token-protected WS interface)
	All,
	// Local "unsafe" context and accounts access
	IpcContext,
	// APIs for Susy Generic Pub-Sub
	PubSub,
	// Fixed list of APis
	List(HashSet<Api>),
}

impl Default for ApiSet {
	fn default() -> Self {
		ApiSet::UnsafeContext
	}
}

impl PartialEq for ApiSet {
	fn eq(&self, other: &Self) -> bool {
		self.list_apis() == other.list_apis()
	}
}

impl FromStr for ApiSet {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut apis = HashSet::new();

		for api in s.split(',') {
			match api {
				"all" => {
					apis.extend(ApiSet::All.list_apis());
				}
				"safe" => {
					// Safe APIs are those that are safe even in UnsafeContext.
					apis.extend(ApiSet::UnsafeContext.list_apis());
				}
				// Remove the API
				api if api.starts_with("-") => {
					let api = api[1..].parse()?;
					apis.remove(&api);
				}
				api => {
					let api = api.parse()?;
					apis.insert(api);
				}
			}
		}

		Ok(ApiSet::List(apis))
	}
}

fn to_modules(apis: &HashSet<Api>) -> BTreeMap<String, String> {
	let mut modules = BTreeMap::new();
	for api in apis {
		let (name, version) = match *api {
			Api::Debug => ("debug", "1.0"),
			Api::Sof => ("sof", "1.0"),
			Api::SofPubSub => ("pubsub", "1.0"),
			Api::Net => ("net", "1.0"),
			Api::Susy => ("susy", "1.0"),
			Api::SusyAccounts => ("susy_accounts", "1.0"),
			Api::SusyPubSub => ("susy_pubsub", "1.0"),
			Api::SusySet => ("susy_set", "1.0"),
			Api::Personal => ("personal", "1.0"),
			Api::Private => ("private", "1.0"),
			Api::Rpc => ("rpc", "1.0"),
			Api::SecretStore => ("secretstore", "1.0"),
			Api::Signer => ("signer", "1.0"),
			Api::Traces => ("traces", "1.0"),
			Api::SusyWeb => ("susyweb", "1.0"),
			Api::Whisper => ("shh", "1.0"),
			Api::WhisperPubSub => ("shh_pubsub", "1.0"),
		};
		modules.insert(name.into(), version.into());
	}
	modules
}

macro_rules! add_signing_methods {
	($namespace:ident, $handler:expr, $deps:expr, $dispatch:expr) => {{
		let deps = &$deps;
		let (dispatcher, accounts) = $dispatch;
		if deps.signer_service.is_enabled() {
			$handler.extend_with($namespace::to_delegate(SigningQueueClient::new(
				&deps.signer_service,
				dispatcher.clone(),
				deps.executor.clone(),
				accounts,
			)))
		} else {
			$handler.extend_with($namespace::to_delegate(SigningUnsafeClient::new(
				accounts,
				dispatcher.clone(),
			)))
			}
		}};
}

/// RPC dependencies can be used to initialize RPC endpoints from APIs.
pub trait Dependencies {
	type Notifier: ActivityNotifier;

	/// Create the activity notifier.
	fn activity_notifier(&self) -> Self::Notifier;

	/// Extend the given I/O handler with endpoints for each API.
	fn extend_with_set<S>(&self, handler: &mut MetaIoHandler<Metadata, S>, apis: &HashSet<Api>)
	where
		S: core::Middleware<Metadata>;
}

/// RPC dependencies for a full node.
pub struct FullDependencies {
	pub signer_service: Arc<SignerService>,
	pub client: Arc<Client>,
	pub snapshot: Arc<SnapshotService>,
	pub sync: Arc<SyncProvider>,
	pub net: Arc<ManageNetwork>,
	pub accounts: Arc<AccountProvider>,
	pub private_tx_service: Option<Arc<PrivateTxService>>,
	pub miner: Arc<Miner>,
	pub external_miner: Arc<ExternalMiner>,
	pub logger: Arc<RotatingLogger>,
	pub settings: Arc<NetworkSettings>,
	pub net_service: Arc<ManageNetwork>,
	pub updater: Arc<Updater>,
	pub graviton_compatibility: bool,
	pub experimental_rpcs: bool,
	pub ws_address: Option<Host>,
	pub fetch: FetchClient,
	pub executor: Executor,
	pub whisper_rpc: Option<::whisper::RpcFactory>,
	pub gas_price_percentile: usize,
	pub poll_lifetime: u32,
	pub allow_missing_blocks: bool,
}

impl FullDependencies {
	fn extend_api<S>(
		&self,
		handler: &mut MetaIoHandler<Metadata, S>,
		apis: &HashSet<Api>,
		for_generic_pubsub: bool,
	) where
		S: core::Middleware<Metadata>,
	{
		use susy_rpc::v1::*;

		let nonces = Arc::new(Mutex::new(dispatch::Reservations::new(
			self.executor.clone(),
		)));
		let dispatcher = FullDispatcher::new(
			self.client.clone(),
			self.miner.clone(),
			nonces.clone(),
			self.gas_price_percentile,
		);
		let account_signer = Arc::new(dispatch::Signer::new(self.accounts.clone())) as _;
		let accounts = account_utils::accounts_list(self.accounts.clone());

		for api in apis {
			match *api {
				Api::Debug => {
					handler.extend_with(DebugClient::new(self.client.clone()).to_delegate());
				}
				Api::SusyWeb => {
					handler.extend_with(SusyWebClient::new().to_delegate());
				}
				Api::Net => {
					handler.extend_with(NetClient::new(&self.sync).to_delegate());
				}
				Api::Sof => {
					let client = SofClient::new(
						&self.client,
						&self.snapshot,
						&self.sync,
						&accounts,
						&self.miner,
						&self.external_miner,
						SofClientOptions {
							pending_nonce_from_queue: self.graviton_compatibility,
							allow_pending_receipt_query: !self.graviton_compatibility,
							send_block_number_in_get_work: !self.graviton_compatibility,
							gas_price_percentile: self.gas_price_percentile,
							allow_missing_blocks: self.allow_missing_blocks,
							allow_experimental_rpcs: self.experimental_rpcs,
						}
					);
					handler.extend_with(client.to_delegate());

					if !for_generic_pubsub {
						let filter_client = SofFilterClient::new(
							self.client.clone(),
							self.miner.clone(),
							self.poll_lifetime,
						);
						handler.extend_with(filter_client.to_delegate());

						add_signing_methods!(SofSigning, handler, self, (&dispatcher, &account_signer));
					}
				}
				Api::SofPubSub => {
					if !for_generic_pubsub {
						let client =
							SofPubSubClient::new(self.client.clone(), self.executor.clone());
						let h = client.handler();
						self.miner
							.add_transactions_listener(Box::new(move |hashes| {
								if let Some(h) = h.upgrade() {
									h.notify_new_transactions(hashes);
								}
							}));

						if let Some(h) = client.handler().upgrade() {
							self.client.add_notify(h);
						}
						handler.extend_with(client.to_delegate());
					}
				}
				Api::Personal => {
					#[cfg(feature = "accounts")]
					handler.extend_with(
						PersonalClient::new(
							&self.accounts,
							dispatcher.clone(),
							self.graviton_compatibility,
							self.experimental_rpcs,
						).to_delegate(),
					);
				}
				Api::Signer => {
					handler.extend_with(
						SignerClient::new(
							account_signer.clone(),
							dispatcher.clone(),
							&self.signer_service,
							self.executor.clone(),
						).to_delegate(),
					);
				}
				Api::Susy => {
					let signer = match self.signer_service.is_enabled() {
						true => Some(self.signer_service.clone()),
						false => None,
					};
					handler.extend_with(
						SusyClient::new(
							self.client.clone(),
							self.miner.clone(),
							self.sync.clone(),
							self.updater.clone(),
							self.net_service.clone(),
							self.logger.clone(),
							self.settings.clone(),
							signer,
							self.ws_address.clone(),
							self.snapshot.clone().into(),
						).to_delegate(),
					);
					#[cfg(feature = "accounts")]
					handler.extend_with(SusyAccountsInfo::to_delegate(SusyAccountsClient::new(&self.accounts)));

					if !for_generic_pubsub {
						add_signing_methods!(SusySigning, handler, self, (&dispatcher, &account_signer));
					}
				}
				Api::SusyPubSub => {
					if !for_generic_pubsub {
						let mut rpc = MetaIoHandler::default();
						let apis = ApiSet::List(apis.clone())
							.retain(ApiSet::PubSub)
							.list_apis();
						self.extend_api(&mut rpc, &apis, true);
						handler.extend_with(
							PubSubClient::new(rpc, self.executor.clone()).to_delegate(),
						);
					}
				}
				Api::SusyAccounts => {
					#[cfg(feature = "accounts")]
					handler.extend_with(SusyAccounts::to_delegate(SusyAccountsClient::new(&self.accounts)));
				}
				Api::SusySet => {
					handler.extend_with(
						SusySetClient::new(
							&self.client,
							&self.miner,
							&self.updater,
							&self.net_service,
							self.fetch.clone(),
						).to_delegate(),
					);
					#[cfg(feature = "accounts")]
					handler.extend_with(
						SusySetAccountsClient::new(
							&self.accounts,
							&self.miner,
						).to_delegate(),
					);
				}
				Api::Traces => handler.extend_with(TracesClient::new(&self.client).to_delegate()),
				Api::Rpc => {
					let modules = to_modules(&apis);
					handler.extend_with(RpcClient::new(modules).to_delegate());
				}
				Api::SecretStore => {
					#[cfg(feature = "accounts")]
					handler.extend_with(SecretStoreClient::new(&self.accounts).to_delegate());
				}
				Api::Whisper => {
					if let Some(ref whisper_rpc) = self.whisper_rpc {
						let whisper = whisper_rpc.make_handler(self.net.clone());
						handler.extend_with(::susy_whisper::rpc::Whisper::to_delegate(whisper));
					}
				}
				Api::WhisperPubSub => {
					if !for_generic_pubsub {
						if let Some(ref whisper_rpc) = self.whisper_rpc {
							let whisper = whisper_rpc.make_handler(self.net.clone());
							handler.extend_with(::susy_whisper::rpc::WhisperPubSub::to_delegate(
								whisper,
							));
						}
					}
				}
				Api::Private => {
					handler.extend_with(
						PrivateClient::new(self.private_tx_service.as_ref().map(|p| p.provider()))
							.to_delegate(),
					);
				}
			}
		}
	}
}

impl Dependencies for FullDependencies {
	type Notifier = ClientNotifier;

	fn activity_notifier(&self) -> ClientNotifier {
		ClientNotifier {
			client: self.client.clone(),
		}
	}

	fn extend_with_set<S>(&self, handler: &mut MetaIoHandler<Metadata, S>, apis: &HashSet<Api>)
	where
		S: core::Middleware<Metadata>,
	{
		self.extend_api(handler, apis, false)
	}
}

/// Light client notifier. Doesn't do anything yet, but might in the future.
pub struct LightClientNotifier;

impl ActivityNotifier for LightClientNotifier {
	fn active(&self) {}
}

/// RPC dependencies for a light client.
pub struct LightDependencies<T> {
	pub signer_service: Arc<SignerService>,
	pub client: Arc<T>,
	pub sync: Arc<LightSync>,
	pub net: Arc<ManageNetwork>,
	pub accounts: Arc<AccountProvider>,
	pub logger: Arc<RotatingLogger>,
	pub settings: Arc<NetworkSettings>,
	pub on_demand: Arc<::light::on_demand::OnDemand>,
	pub cache: Arc<Mutex<LightDataCache>>,
	pub transaction_queue: Arc<RwLock<LightTransactionQueue>>,
	pub ws_address: Option<Host>,
	pub fetch: FetchClient,
	pub graviton_compatibility: bool,
	pub experimental_rpcs: bool,
	pub executor: Executor,
	pub whisper_rpc: Option<::whisper::RpcFactory>,
	pub private_tx_service: Option<Arc<PrivateTransactionManager>>,
	pub gas_price_percentile: usize,
	pub poll_lifetime: u32,
}

impl<C: LightChainClient + 'static> LightDependencies<C> {
	fn extend_api<T: core::Middleware<Metadata>>(
		&self,
		handler: &mut MetaIoHandler<Metadata, T>,
		apis: &HashSet<Api>,
		for_generic_pubsub: bool,
	) {
		use susy_rpc::v1::*;

		let dispatcher = LightDispatcher::new(
			self.sync.clone(),
			self.client.clone(),
			self.on_demand.clone(),
			self.cache.clone(),
			self.transaction_queue.clone(),
			Arc::new(Mutex::new(dispatch::Reservations::new(
				self.executor.clone(),
			))),
			self.gas_price_percentile,
		);
		let account_signer = Arc::new(dispatch::Signer::new(self.accounts.clone())) as _;
		let accounts = account_utils::accounts_list(self.accounts.clone());

		for api in apis {
			match *api {
				Api::Debug => {
					warn!(target: "rpc", "Debug API is not available in light client mode.")
				}
				Api::SusyWeb => {
					handler.extend_with(SusyWebClient::new().to_delegate());
				}
				Api::Net => {
					handler.extend_with(light::NetClient::new(self.sync.clone()).to_delegate());
				}
				Api::Sof => {
					let client = light::SofClient::new(
						self.sync.clone(),
						self.client.clone(),
						self.on_demand.clone(),
						self.transaction_queue.clone(),
						accounts.clone(),
						self.cache.clone(),
						self.gas_price_percentile,
						self.poll_lifetime,
					);
					handler.extend_with(Sof::to_delegate(client.clone()));

					if !for_generic_pubsub {
						handler.extend_with(SofFilter::to_delegate(client));
						add_signing_methods!(SofSigning, handler, self, (&dispatcher, &account_signer));
					}
				}
				Api::SofPubSub => {
					let client = SofPubSubClient::light(
						self.client.clone(),
						self.on_demand.clone(),
						self.sync.clone(),
						self.cache.clone(),
						self.executor.clone(),
						self.gas_price_percentile,
					);
					self.client.add_listener(client.handler() as Weak<_>);
					let h = client.handler();
					self.transaction_queue
						.write()
						.add_listener(Box::new(move |transactions| {
							if let Some(h) = h.upgrade() {
								h.notify_new_transactions(transactions);
							}
						}));
					handler.extend_with(SofPubSub::to_delegate(client));
				}
				Api::Personal => {
					#[cfg(feature = "accounts")]
					handler.extend_with(
						PersonalClient::new(
							&self.accounts,
							dispatcher.clone(),
							self.graviton_compatibility,
							self.experimental_rpcs,
						).to_delegate(),
					);
				}
				Api::Signer => {
					handler.extend_with(
						SignerClient::new(
							account_signer.clone(),
							dispatcher.clone(),
							&self.signer_service,
							self.executor.clone(),
						).to_delegate(),
					);
				}
				Api::Susy => {
					let signer = match self.signer_service.is_enabled() {
						true => Some(self.signer_service.clone()),
						false => None,
					};
					handler.extend_with(
						light::SusyClient::new(
							Arc::new(dispatcher.clone()),
							self.logger.clone(),
							self.settings.clone(),
							signer,
							self.ws_address.clone(),
							self.gas_price_percentile,
						).to_delegate(),
					);
					#[cfg(feature = "accounts")]
					handler.extend_with(
						SusyAccountsInfo::to_delegate(SusyAccountsClient::new(&self.accounts))
					);

					if !for_generic_pubsub {
						add_signing_methods!(SusySigning, handler, self, (&dispatcher, &account_signer));
					}
				}
				Api::SusyPubSub => {
					if !for_generic_pubsub {
						let mut rpc = MetaIoHandler::default();
						let apis = ApiSet::List(apis.clone())
							.retain(ApiSet::PubSub)
							.list_apis();
						self.extend_api(&mut rpc, &apis, true);
						handler.extend_with(
							PubSubClient::new(rpc, self.executor.clone()).to_delegate(),
						);
					}
				}
				Api::SusyAccounts => {
					#[cfg(feature = "accounts")]
					handler.extend_with(SusyAccounts::to_delegate(SusyAccountsClient::new(&self.accounts)));
				}
				Api::SusySet => handler.extend_with(
					light::SusySetClient::new(self.client.clone(), self.sync.clone(), self.fetch.clone())
						.to_delegate(),
				),
				Api::Traces => handler.extend_with(light::TracesClient.to_delegate()),
				Api::Rpc => {
					let modules = to_modules(&apis);
					handler.extend_with(RpcClient::new(modules).to_delegate());
				}
				Api::SecretStore => {
					#[cfg(feature = "accounts")]
					handler.extend_with(SecretStoreClient::new(&self.accounts).to_delegate());
				}
				Api::Whisper => {
					if let Some(ref whisper_rpc) = self.whisper_rpc {
						let whisper = whisper_rpc.make_handler(self.net.clone());
						handler.extend_with(::susy_whisper::rpc::Whisper::to_delegate(whisper));
					}
				}
				Api::WhisperPubSub => {
					if let Some(ref whisper_rpc) = self.whisper_rpc {
						let whisper = whisper_rpc.make_handler(self.net.clone());
						handler.extend_with(::susy_whisper::rpc::WhisperPubSub::to_delegate(
							whisper,
						));
					}
				}
				Api::Private => {
					if let Some(ref tx_manager) = self.private_tx_service {
						let private_tx_service = Some(tx_manager.clone());
						handler.extend_with(PrivateClient::new(private_tx_service).to_delegate());
					}
				}
			}
		}
	}
}

impl<T: LightChainClient + 'static> Dependencies for LightDependencies<T> {
	type Notifier = LightClientNotifier;

	fn activity_notifier(&self) -> Self::Notifier {
		LightClientNotifier
	}

	fn extend_with_set<S>(&self, handler: &mut MetaIoHandler<Metadata, S>, apis: &HashSet<Api>)
	where
		S: core::Middleware<Metadata>,
	{
		self.extend_api(handler, apis, false)
	}
}

impl ApiSet {
	/// Retains only APIs in given set.
	pub fn retain(self, set: Self) -> Self {
		ApiSet::List(&self.list_apis() & &set.list_apis())
	}

	pub fn list_apis(&self) -> HashSet<Api> {
		let mut public_list: HashSet<Api> = [
			Api::SusyWeb,
			Api::Net,
			Api::Sof,
			Api::SofPubSub,
			Api::Susy,
			Api::Rpc,
			Api::Whisper,
			Api::WhisperPubSub,
			Api::Private,
		]
			.into_iter()
			.cloned()
			.collect();

		match *self {
			ApiSet::List(ref apis) => apis.clone(),
			ApiSet::UnsafeContext => {
				public_list.insert(Api::Traces);
				public_list.insert(Api::SusyPubSub);
				public_list
			}
			ApiSet::IpcContext => {
				public_list.insert(Api::Traces);
				public_list.insert(Api::SusyPubSub);
				public_list.insert(Api::SusyAccounts);
				public_list
			}
			ApiSet::All => {
				public_list.insert(Api::Debug);
				public_list.insert(Api::Traces);
				public_list.insert(Api::SusyPubSub);
				public_list.insert(Api::SusyAccounts);
				public_list.insert(Api::SusySet);
				public_list.insert(Api::Signer);
				public_list.insert(Api::Personal);
				public_list.insert(Api::SecretStore);
				public_list
			}
			ApiSet::PubSub => [
				Api::Sof,
				Api::Susy,
				Api::SusyAccounts,
				Api::SusySet,
				Api::Traces,
			]
				.into_iter()
				.cloned()
				.collect(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::{Api, ApiSet};

	#[test]
	fn test_api_parsing() {
		assert_eq!(Api::Debug, "debug".parse().unwrap());
		assert_eq!(Api::SusyWeb, "susyweb".parse().unwrap());
		assert_eq!(Api::Net, "net".parse().unwrap());
		assert_eq!(Api::Sof, "sof".parse().unwrap());
		assert_eq!(Api::SofPubSub, "pubsub".parse().unwrap());
		assert_eq!(Api::Personal, "personal".parse().unwrap());
		assert_eq!(Api::Signer, "signer".parse().unwrap());
		assert_eq!(Api::Susy, "susy".parse().unwrap());
		assert_eq!(Api::SusyAccounts, "susy_accounts".parse().unwrap());
		assert_eq!(Api::SusySet, "susy_set".parse().unwrap());
		assert_eq!(Api::Traces, "traces".parse().unwrap());
		assert_eq!(Api::Rpc, "rpc".parse().unwrap());
		assert_eq!(Api::SecretStore, "secretstore".parse().unwrap());
		assert_eq!(Api::Private, "private".parse().unwrap());
		assert_eq!(Api::Whisper, "shh".parse().unwrap());
		assert_eq!(Api::WhisperPubSub, "shh_pubsub".parse().unwrap());
		assert!("rp".parse::<Api>().is_err());
	}

	#[test]
	fn test_api_set_default() {
		assert_eq!(ApiSet::UnsafeContext, ApiSet::default());
	}

	#[test]
	fn test_api_set_parsing() {
		assert_eq!(
			ApiSet::List(vec![Api::SusyWeb, Api::Sof].into_iter().collect()),
			"susyweb,sof".parse().unwrap()
		);
	}

	#[test]
	fn test_api_set_unsafe_context() {
		let expected = vec![
			// make sure this list contains only SAFE methods
			Api::SusyWeb,
			Api::Net,
			Api::Sof,
			Api::SofPubSub,
			Api::Susy,
			Api::SusyPubSub,
			Api::Traces,
			Api::Rpc,
			Api::Whisper,
			Api::WhisperPubSub,
			Api::Private,
		].into_iter()
		.collect();
		assert_eq!(ApiSet::UnsafeContext.list_apis(), expected);
	}

	#[test]
	fn test_api_set_ipc_context() {
		let expected = vec![
			// safe
			Api::SusyWeb,
			Api::Net,
			Api::Sof,
			Api::SofPubSub,
			Api::Susy,
			Api::SusyPubSub,
			Api::Traces,
			Api::Rpc,
			Api::Whisper,
			Api::WhisperPubSub,
			Api::Private,
			// semi-safe
			Api::SusyAccounts,
		].into_iter()
		.collect();
		assert_eq!(ApiSet::IpcContext.list_apis(), expected);
	}

	#[test]
	fn test_all_apis() {
		assert_eq!(
			"all".parse::<ApiSet>().unwrap(),
			ApiSet::List(
				vec![
					Api::SusyWeb,
					Api::Net,
					Api::Sof,
					Api::SofPubSub,
					Api::Susy,
					Api::SusyPubSub,
					Api::Traces,
					Api::Rpc,
					Api::SecretStore,
					Api::Whisper,
					Api::WhisperPubSub,
					Api::SusyAccounts,
					Api::SusySet,
					Api::Signer,
					Api::Personal,
					Api::Private,
					Api::Debug,
				].into_iter()
				.collect()
			)
		);
	}

	#[test]
	fn test_all_without_personal_apis() {
		assert_eq!(
			"personal,all,-personal".parse::<ApiSet>().unwrap(),
			ApiSet::List(
				vec![
					Api::SusyWeb,
					Api::Net,
					Api::Sof,
					Api::SofPubSub,
					Api::Susy,
					Api::SusyPubSub,
					Api::Traces,
					Api::Rpc,
					Api::SecretStore,
					Api::Whisper,
					Api::WhisperPubSub,
					Api::SusyAccounts,
					Api::SusySet,
					Api::Signer,
					Api::Private,
					Api::Debug,
				].into_iter()
				.collect()
			)
		);
	}

	#[test]
	fn test_safe_parsing() {
		assert_eq!(
			"safe".parse::<ApiSet>().unwrap(),
			ApiSet::List(
				vec![
					Api::SusyWeb,
					Api::Net,
					Api::Sof,
					Api::SofPubSub,
					Api::Susy,
					Api::SusyPubSub,
					Api::Traces,
					Api::Rpc,
					Api::Whisper,
					Api::WhisperPubSub,
					Api::Private,
				].into_iter()
				.collect()
			)
		);
	}
}
