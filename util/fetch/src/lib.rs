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

//! A service to fetch any HTTP / HTTPS content.

#![warn(missing_docs)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate futures;

extern crate hyper;
extern crate hyper_rustls;
extern crate http;

extern crate tokio;
extern crate url;
extern crate bytes;

/// Fetch client implementation.
pub mod client;

pub use url::Url;
pub use self::client::{Client, Fetch, Error, Response, Request, Abort, BodyReader};
pub use hyper::Method;
