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

use types::transaction::{Transaction, SignedTransaction, Action};
use std::cmp::min;

use sophon_types::U256;
use susy_jsonrpc_core::Error;
use v1::helpers::CallRequest;

pub fn sign_call(request: CallRequest) -> Result<SignedTransaction, Error> {
	let max_gas = U256::from(500_000_000);
	let gas = min(request.gas.unwrap_or(max_gas), max_gas);
	let from = request.from.unwrap_or(0.into());

	Ok(Transaction {
		nonce: request.nonce.unwrap_or_else(|| 0.into()),
		action: request.to.map_or(Action::Create, Action::Call),
		gas,
		gas_price: request.gas_price.unwrap_or_else(|| 0.into()),
		value: request.value.unwrap_or(0.into()),
		data: request.data.unwrap_or_default(),
	}.fake_sign(from))
}
