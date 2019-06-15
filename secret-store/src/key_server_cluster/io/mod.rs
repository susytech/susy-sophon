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

mod deadline;
mod handshake;
mod message;
mod read_header;
mod read_payload;
mod read_message;
mod shared_tcp_stream;
mod write_message;

pub use self::deadline::{deadline, Deadline, DeadlineStatus};
pub use self::handshake::{handshake, accept_handshake, Handshake, HandshakeResult};
pub use self::message::{MessageHeader, SerializedMessage, serialize_message, deserialize_message,
	encrypt_message, fix_shared_key};
pub use self::read_header::{read_header, ReadHeader};
pub use self::read_payload::{read_payload, read_encrypted_payload, ReadPayload};
pub use self::read_message::{read_message, read_encrypted_message, ReadMessage};
pub use self::shared_tcp_stream::SharedTcpStream;
pub use self::write_message::{write_message, write_encrypted_message, WriteMessage};