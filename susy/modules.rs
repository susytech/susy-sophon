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

use std::sync::{Arc, mpsc};

use sofcore::client::BlockChainClient;
use sync::{self, AttachedProtocol, SyncConfig, NetworkConfiguration, Params, ConnectionFilter};
use sofcore::snapshot::SnapshotService;
use light::Provider;

pub use sync::{SofSync, SyncProvider, ManageNetwork, PrivateTxHandler};
pub use sofcore::client::ChainNotify;
use sofcore_logger::Config as LogConfig;

pub type SyncModules = (
	Arc<SyncProvider>,
	Arc<ManageNetwork>,
	Arc<ChainNotify>,
	mpsc::Sender<sync::PriorityTask>,
);

pub fn sync(
	config: SyncConfig,
	network_config: NetworkConfiguration,
	chain: Arc<BlockChainClient>,
	snapshot_service: Arc<SnapshotService>,
	private_tx_handler: Option<Arc<PrivateTxHandler>>,
	provider: Arc<Provider>,
	_log_settings: &LogConfig,
	attached_protos: Vec<AttachedProtocol>,
	connection_filter: Option<Arc<ConnectionFilter>>,
) -> Result<SyncModules, sync::Error> {
	let sof_sync = SofSync::new(Params {
		config,
		chain,
		provider,
		snapshot_service,
		private_tx_handler,
		network_config,
		attached_protos,
	},
	connection_filter)?;

	Ok((
		sof_sync.clone() as Arc<SyncProvider>,
		sof_sync.clone() as Arc<ManageNetwork>,
		sof_sync.clone() as Arc<ChainNotify>,
		sof_sync.priority_tasks()
	))
}
