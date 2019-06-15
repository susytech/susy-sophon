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

use block_sync::BlockRequest;
use bytes::Bytes;
use sophon_types::H256;
use network::{PeerId};
use srlp::RlpStream;
use std::time::Instant;
use sync_io::SyncIo;
use types::BlockNumber;

use super::sync_packet::SyncPacket;
use super::sync_packet::SyncPacket::{
	GetBlockHeadersPacket,
	GetBlockBodiesPacket,
	GetReceiptsPacket,
	GetSnapshotManifestPacket,
	GetSnapshotDataPacket,
};

use super::{
	BlockSet,
	ChainSync,
	PeerAsking,
};

/// The Chain Sync Requester: requesting data to other peers
pub struct SyncRequester;

impl SyncRequester {
	/// Perform block download request`
	pub fn request_blocks(sync: &mut ChainSync, io: &mut SyncIo, peer_id: PeerId, request: BlockRequest, block_set: BlockSet) {
		match request {
			BlockRequest::Headers { start, count, skip } => {
				SyncRequester::request_headers_by_hash(sync, io, peer_id, &start, count, skip, false, block_set);
			},
			BlockRequest::Bodies { hashes } => {
				SyncRequester::request_bodies(sync, io, peer_id, hashes, block_set);
			},
			BlockRequest::Receipts { hashes } => {
				SyncRequester::request_receipts(sync, io, peer_id, hashes, block_set);
			},
		}
	}

	/// Request block bodies from a peer
	fn request_bodies(sync: &mut ChainSync, io: &mut SyncIo, peer_id: PeerId, hashes: Vec<H256>, set: BlockSet) {
		let mut srlp = RlpStream::new_list(hashes.len());
		trace!(target: "sync", "{} <- GetBlockBodies: {} entries starting from {:?}, set = {:?}", peer_id, hashes.len(), hashes.first(), set);
		for h in &hashes {
			srlp.append(&h.clone());
		}
		SyncRequester::send_request(sync, io, peer_id, PeerAsking::BlockBodies, GetBlockBodiesPacket, srlp.out());
		let peer = sync.peers.get_mut(&peer_id).expect("peer_id may originate either from on_packet, where it is already validated or from enumerating self.peers. qed");
		peer.asking_blocks = hashes;
		peer.block_set = Some(set);
	}

	/// Request headers from a peer by block number
	pub fn request_fork_header(sync: &mut ChainSync, io: &mut SyncIo, peer_id: PeerId, n: BlockNumber) {
		trace!(target: "sync", "{} <- GetForkHeader: at {}", peer_id, n);
		let mut srlp = RlpStream::new_list(4);
		srlp.append(&n);
		srlp.append(&1u32);
		srlp.append(&0u32);
		srlp.append(&0u32);
		SyncRequester::send_request(sync, io, peer_id, PeerAsking::ForkHeader, GetBlockHeadersPacket, srlp.out());
	}

	/// Find some headers or blocks to download for a peer.
	pub fn request_snapshot_data(sync: &mut ChainSync, io: &mut SyncIo, peer_id: PeerId) {
		// find chunk data to download
		if let Some(hash) = sync.snapshot.needed_chunk() {
			if let Some(ref mut peer) = sync.peers.get_mut(&peer_id) {
				peer.asking_snapshot_data = Some(hash.clone());
			}
			SyncRequester::request_snapshot_chunk(sync, io, peer_id, &hash);
		}
	}

	/// Request snapshot manifest from a peer.
	pub fn request_snapshot_manifest(sync: &mut ChainSync, io: &mut SyncIo, peer_id: PeerId) {
		trace!(target: "sync", "{} <- GetSnapshotManifest", peer_id);
		let srlp = RlpStream::new_list(0);
		SyncRequester::send_request(sync, io, peer_id, PeerAsking::SnapshotManifest, GetSnapshotManifestPacket, srlp.out());
	}

	/// Request headers from a peer by block hash
	fn request_headers_by_hash(sync: &mut ChainSync, io: &mut SyncIo, peer_id: PeerId, h: &H256, count: u64, skip: u64, reverse: bool, set: BlockSet) {
		trace!(target: "sync", "{} <- GetBlockHeaders: {} entries starting from {}, set = {:?}", peer_id, count, h, set);
		let mut srlp = RlpStream::new_list(4);
		srlp.append(h);
		srlp.append(&count);
		srlp.append(&skip);
		srlp.append(&if reverse {1u32} else {0u32});
		SyncRequester::send_request(sync, io, peer_id, PeerAsking::BlockHeaders, GetBlockHeadersPacket, srlp.out());
		let peer = sync.peers.get_mut(&peer_id).expect("peer_id may originate either from on_packet, where it is already validated or from enumerating self.peers. qed");
		peer.asking_hash = Some(h.clone());
		peer.block_set = Some(set);
	}

	/// Request block receipts from a peer
	fn request_receipts(sync: &mut ChainSync, io: &mut SyncIo, peer_id: PeerId, hashes: Vec<H256>, set: BlockSet) {
		let mut srlp = RlpStream::new_list(hashes.len());
		trace!(target: "sync", "{} <- GetBlockReceipts: {} entries starting from {:?}, set = {:?}", peer_id, hashes.len(), hashes.first(), set);
		for h in &hashes {
			srlp.append(&h.clone());
		}
		SyncRequester::send_request(sync, io, peer_id, PeerAsking::BlockReceipts, GetReceiptsPacket, srlp.out());
		let peer = sync.peers.get_mut(&peer_id).expect("peer_id may originate either from on_packet, where it is already validated or from enumerating self.peers. qed");
		peer.asking_blocks = hashes;
		peer.block_set = Some(set);
	}

	/// Request snapshot chunk from a peer.
	fn request_snapshot_chunk(sync: &mut ChainSync, io: &mut SyncIo, peer_id: PeerId, chunk: &H256) {
		trace!(target: "sync", "{} <- GetSnapshotData {:?}", peer_id, chunk);
		let mut srlp = RlpStream::new_list(1);
		srlp.append(chunk);
		SyncRequester::send_request(sync, io, peer_id, PeerAsking::SnapshotData, GetSnapshotDataPacket, srlp.out());
	}

	/// Generic request sender
	fn send_request(sync: &mut ChainSync, io: &mut SyncIo, peer_id: PeerId, asking: PeerAsking, packet_id: SyncPacket, packet: Bytes) {
		if let Some(ref mut peer) = sync.peers.get_mut(&peer_id) {
			if peer.asking != PeerAsking::Nothing {
				warn!(target:"sync", "Asking {:?} while requesting {:?}", peer.asking, asking);
			}
			peer.asking = asking;
			peer.ask_time = Instant::now();

			let result = io.send(peer_id, packet_id, packet);

			if let Err(e) = result {
				debug!(target:"sync", "Error sending request: {:?}", e);
				io.disconnect_peer(peer_id);
			}
		}
	}
}
