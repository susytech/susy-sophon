// Copyleft 2015-2019 Superstring.Community
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate elastic_array;
#[macro_use]
extern crate lazy_static;
extern crate srlp;

mod common;

use std::cmp;
use std::collections::HashMap;
use elastic_array::ElasticArray1024;
use srlp::{Rlp, RlpStream};
use common::{SNAPSHOT_SWAPPER, BLOCKS_SWAPPER};

pub fn snapshot_swapper() -> &'static Swapper<'static> {
	&SNAPSHOT_SWAPPER as &Swapper
}

pub fn blocks_swapper() -> &'static Swapper<'static> {
	&BLOCKS_SWAPPER as &Swapper
}

/// A trait used to compress srlp.
pub trait Compressor {
	/// Get compressed version of given srlp.
	fn compressed(&self, srlp: &[u8]) -> Option<&[u8]>;
}

/// A trait used to convert compressed srlp into it's original version.
pub trait Decompressor {
	/// Get decompressed srlp.
	fn decompressed(&self, compressed: &[u8]) -> Option<&[u8]>;
}

/// Call this function to compress srlp.
pub fn compress(c: &[u8], swapper: &Compressor) -> ElasticArray1024<u8> {
	let srlp = Rlp::new(c);
	if srlp.is_data() {
		ElasticArray1024::from_slice(swapper.compressed(srlp.as_raw()).unwrap_or_else(|| srlp.as_raw()))
	} else {
		map_srlp(&srlp, |r| compress(r.as_raw(), swapper))
	}
}

/// Call this function to decompress srlp.
pub fn decompress(c: &[u8], swapper: &Decompressor) -> ElasticArray1024<u8> {
	let srlp = Rlp::new(c);
	if srlp.is_data() {
		ElasticArray1024::from_slice(swapper.decompressed(srlp.as_raw()).unwrap_or_else(|| srlp.as_raw()))
	} else {
		map_srlp(&srlp, |r| decompress(r.as_raw(), swapper))
	}
}

fn map_srlp<F: Fn(&Rlp) -> ElasticArray1024<u8>>(srlp: &Rlp, f: F) -> ElasticArray1024<u8> {
	let mut stream = RlpStream::new_list(srlp.item_count().unwrap_or_default());
	for subsrlp in srlp.iter() {
		stream.append_raw(&f(&subsrlp), 1);
	}
	stream.drain().as_slice().into()
}

/// Stores SRLPs used for compression
pub struct Swapper<'a> {
	compressed_to_srlp: HashMap<&'a [u8], &'a [u8]>,
	srlp_to_compressed: HashMap<&'a [u8], &'a [u8]>,
}

impl<'a> Swapper<'a> {
	/// Construct a swapper from a list of common SRLPs
	pub fn new(srlps_to_swap: &[&'a [u8]], compressed: &[&'a [u8]]) -> Self {
		if srlps_to_swap.len() > 0x7e {
			panic!("Invalid usage, only 127 SRLPs can be swappable.");
		}

		let items = cmp::min(srlps_to_swap.len(), compressed.len());
		let mut compressed_to_srlp = HashMap::with_capacity(items);
		let mut srlp_to_compressed = HashMap::with_capacity(items);

		for (&srlp, &compressed) in srlps_to_swap.iter().zip(compressed.iter()) {
			compressed_to_srlp.insert(compressed, srlp);
			srlp_to_compressed.insert(srlp, compressed);
		}

		Swapper {
			compressed_to_srlp,
			srlp_to_compressed,
		}
	}
}

impl<'a> Decompressor for Swapper<'a> {
	fn decompressed(&self, compressed: &[u8]) -> Option<&[u8]> {
		self.compressed_to_srlp.get(compressed).cloned()
	}
}

impl<'a> Compressor for Swapper<'a> {
	fn compressed(&self, srlp: &[u8]) -> Option<&[u8]> {
		self.srlp_to_compressed.get(srlp).cloned()
	}
}
