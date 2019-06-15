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

use super::super::NetworkConfiguration;
use network::NetworkConfiguration as BasicNetworkConfiguration;
use std::convert::From;
use ipc::binary::{serialize, deserialize};

#[test]
fn network_settings_serialize() {
	let net_cfg = NetworkConfiguration::from(BasicNetworkConfiguration::new_local());
	let serialized = serialize(&net_cfg).unwrap();
	let deserialized = deserialize::<NetworkConfiguration>(&serialized).unwrap();

	assert_eq!(net_cfg.udp_port, deserialized.udp_port);
}
