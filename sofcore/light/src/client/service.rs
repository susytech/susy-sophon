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

//! Minimal IO service for light client.
//! Just handles block import messages and passes them to the client.

use std::fmt;
use std::sync::Arc;

use sofcore_db as db;
use sofcore_blockchain::BlockChainDB;
use sofcore::client::ClientIoMessage;
use sofcore::error::Error as CoreError;
use sofcore::spec::Spec;
use io::{IoContext, IoError, IoHandler, IoService};

use cache::Cache;
use parking_lot::Mutex;

use super::{ChainDataFetcher, LightChainNotify, Client, Config as ClientConfig};

/// Errors on service initialization.
#[derive(Debug)]
pub enum Error {
	/// Core error.
	Core(CoreError),
	/// I/O service error.
	Io(IoError),
}

impl From<CoreError> for Error {
	#[inline]
	fn from(err: CoreError) -> Error {
		Error::Core(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::Core(ref msg) => write!(f, "Core error: {}", msg),
			Error::Io(ref err) => write!(f, "I/O service error: {}", err),
		}
	}
}

/// Light client service.
pub struct Service<T> {
	client: Arc<Client<T>>,
	io_service: IoService<ClientIoMessage>,
}

impl<T: ChainDataFetcher> Service<T> {
	/// Start the service: initialize I/O workers and client itself.
	pub fn start(config: ClientConfig, spec: &Spec, fetcher: T, db: Arc<BlockChainDB>, cache: Arc<Mutex<Cache>>) -> Result<Self, Error> {
		let io_service = IoService::<ClientIoMessage>::start().map_err(Error::Io)?;
		let client = Arc::new(Client::new(config,
			db.key_value().clone(),
			db::COL_LIGHT_CHAIN,
			spec,
			fetcher,
			io_service.channel(),
			cache,
		)?);

		io_service.register_handler(Arc::new(ImportBlocks(client.clone()))).map_err(Error::Io)?;
		spec.engine.register_client(Arc::downgrade(&client) as _);

		Ok(Service {
			client,
			io_service,
		})
	}

	/// Set the actor to be notified on certain chain events
	pub fn add_notify(&self, notify: Arc<LightChainNotify>) {
		self.client.add_listener(Arc::downgrade(&notify));
	}

	/// Register an I/O handler on the service.
	pub fn register_handler(&self, handler: Arc<IoHandler<ClientIoMessage> + Send>) -> Result<(), IoError> {
		self.io_service.register_handler(handler)
	}

	/// Get a handle to the client.
	pub fn client(&self) -> &Arc<Client<T>> {
		&self.client
	}
}

struct ImportBlocks<T>(Arc<Client<T>>);

impl<T: ChainDataFetcher> IoHandler<ClientIoMessage> for ImportBlocks<T> {
	fn message(&self, _io: &IoContext<ClientIoMessage>, message: &ClientIoMessage) {
		if let ClientIoMessage::BlockVerified = *message {
			self.0.import_verified();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Service;
	use sofcore::spec::Spec;

	use std::sync::Arc;
	use cache::Cache;
	use client::fetch;
	use std::time::Duration;
	use parking_lot::Mutex;
	use sofcore::test_helpers;

	#[test]
	fn it_works() {
		let db = test_helpers::new_db();
		let spec = Spec::new_test();
		let cache = Arc::new(Mutex::new(Cache::new(Default::default(), Duration::from_secs(6 * 3600))));

		Service::start(Default::default(), &spec, fetch::unavailable(), db, cache).unwrap();
	}
}
