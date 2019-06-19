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

//! Signing RPC implementation.

use std::sync::Arc;
use transient_hashmap::TransientHashMap;
use parking_lot::Mutex;

use sophon_types::{H160, H256, H520, U256};

use susy_jsonrpc_core::{BoxFuture, Result, Error};
use susy_jsonrpc_core::futures::{future, Future, Poll, Async};
use susy_jsonrpc_core::futures::future::Either;

use v1::helpers::deprecated::{self, DeprecationNotice};
use v1::helpers::dispatch::{self, Dispatcher};
use v1::helpers::errors;
use v1::helpers::external_signer::{
	SignerService, SigningQueue,
	ConfirmationReceiver as RpcConfirmationReceiver,
	ConfirmationResult as RpcConfirmationResult,
};
use v1::metadata::Metadata;
use v1::traits::{SofSigning, SusySigning};
use v1::types::{
	Bytes as RpcBytes,
	Either as RpcEither,
	RichRawTransaction as RpcRichRawTransaction,
	TransactionRequest as RpcTransactionRequest,
	ConfirmationPayload as RpcConfirmationPayload,
	ConfirmationResponse as RpcConfirmationResponse,
	Origin,
};

use susy_runtime::Executor;

/// After 60s entries that are not queried with `check_request` will get garbage collected.
const MAX_PENDING_DURATION_SEC: u32 = 60;

#[must_use = "futures do nothing unless polled"]
enum DispatchResult {
	Future(U256, RpcConfirmationReceiver),
	Value(RpcConfirmationResponse),
}

impl Future for DispatchResult {
	type Item = RpcConfirmationResponse;
	type Error = Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
		match *self {
			DispatchResult::Value(ref response) => Ok(Async::Ready(response.clone())),
			DispatchResult::Future(_uid, ref mut future) => try_ready!(future.poll()).map(Async::Ready),
		}
	}
}

fn schedule(executor: Executor,
	confirmations: Arc<Mutex<TransientHashMap<U256, Option<RpcConfirmationResult>>>>,
	id: U256,
	future: RpcConfirmationReceiver) {
	{
		let mut confirmations = confirmations.lock();
		confirmations.insert(id.clone(), None);
	}

	let future = future.then(move |result| {
		let mut confirmations = confirmations.lock();
		confirmations.prune();
		let result = result.and_then(|response| response);
		confirmations.insert(id, Some(result));
		Ok(())
	});
	executor.spawn(future);
}

/// Implementation of functions that require signing when no trusted signer is used.
pub struct SigningQueueClient<D> {
	signer: Arc<SignerService>,
	accounts: Arc<dispatch::Accounts>,
	dispatcher: D,
	executor: Executor,
	// None here means that the request hasn't yet been confirmed
	confirmations: Arc<Mutex<TransientHashMap<U256, Option<RpcConfirmationResult>>>>,
	deprecation_notice: DeprecationNotice,
}

impl<D: Dispatcher + 'static> SigningQueueClient<D> {
	/// Creates a new signing queue client given shared signing queue.
	pub fn new(signer: &Arc<SignerService>, dispatcher: D, executor: Executor, accounts: &Arc<dispatch::Accounts>) -> Self {
		SigningQueueClient {
			signer: signer.clone(),
			accounts: accounts.clone(),
			dispatcher,
			executor,
			confirmations: Arc::new(Mutex::new(TransientHashMap::new(MAX_PENDING_DURATION_SEC))),
			deprecation_notice: Default::default(),
		}
	}

	fn dispatch(&self, payload: RpcConfirmationPayload, origin: Origin) -> BoxFuture<DispatchResult> {
		let default_account = self.accounts.default_account();
		let accounts = self.accounts.clone();
		let dispatcher = self.dispatcher.clone();
		let signer = self.signer.clone();
		Box::new(dispatch::from_rpc(payload, default_account, &dispatcher)
			.and_then(move |payload| {
				let sender = payload.sender();
				if accounts.is_unlocked(&sender) {
					Either::A(dispatch::execute(dispatcher, &accounts, payload, dispatch::SignWith::Nothing)
						.map(|v| v.into_value())
						.map(DispatchResult::Value))
				} else {
					Either::B(future::done(
						signer.add_request(payload, origin)
							.map(|(id, future)| DispatchResult::Future(id, future))
							.map_err(|_| errors::request_rejected_limit())
					))
				}
			}))
	}
}

impl<D: Dispatcher + 'static> SusySigning for SigningQueueClient<D> {
	type Metadata = Metadata;

	fn compose_transaction(&self, _meta: Metadata, transaction: RpcTransactionRequest) -> BoxFuture<RpcTransactionRequest> {
		let default_account = self.accounts.default_account();
		Box::new(self.dispatcher.fill_optional_fields(transaction.into(), default_account, true).map(Into::into))
	}

	fn post_sign(&self, meta: Metadata, address: H160, data: RpcBytes) -> BoxFuture<RpcEither<U256, RpcConfirmationResponse>> {
		self.deprecation_notice.print("susy_postSign", deprecated::msgs::ACCOUNTS);
		let executor = self.executor.clone();
		let confirmations = self.confirmations.clone();

		Box::new(self.dispatch(
			RpcConfirmationPayload::SofSignMessage((address.clone(), data).into()),
			meta.origin
		).map(move |result| match result {
			DispatchResult::Value(v) => RpcEither::Or(v),
			DispatchResult::Future(id, future) => {
				schedule(executor, confirmations, id, future);
				RpcEither::Either(id.into())
			},
		}))
	}

	fn post_transaction(&self, meta: Metadata, request: RpcTransactionRequest) -> BoxFuture<RpcEither<U256, RpcConfirmationResponse>> {
		self.deprecation_notice.print("susy_postTransaction", deprecated::msgs::ACCOUNTS);
		let executor = self.executor.clone();
		let confirmations = self.confirmations.clone();

		Box::new(self.dispatch(RpcConfirmationPayload::SendTransaction(request), meta.origin)
			.map(|result| match result {
				DispatchResult::Value(v) => RpcEither::Or(v),
				DispatchResult::Future(id, future) => {
					schedule(executor, confirmations, id, future);
					RpcEither::Either(id.into())
				},
			}))
	}

	fn check_request(&self, id: U256) -> Result<Option<RpcConfirmationResponse>> {
		self.deprecation_notice.print("susy_checkRequest", deprecated::msgs::ACCOUNTS);
		let id: U256 = id.into();
		match self.confirmations.lock().get(&id) {
			None => Err(errors::request_not_found()), // Request info has been dropped, or even never been there
			Some(&None) => Ok(None), // No confirmation yet, request is known, confirmation is pending
			Some(&Some(ref confirmation)) => confirmation.clone().map(Some), // Confirmation is there
		}
	}

	fn decrypt_message(&self, meta: Metadata, address: H160, data: RpcBytes) -> BoxFuture<RpcBytes> {
		self.deprecation_notice.print("susy_decryptMessage", deprecated::msgs::ACCOUNTS);
		let res = self.dispatch(
			RpcConfirmationPayload::Decrypt((address.clone(), data).into()),
			meta.origin,
		);

		// when dispatch is complete - wait for result and then
		Box::new(res.flatten().and_then(move |response| {
			match response {
				RpcConfirmationResponse::Decrypt(data) => Ok(data),
				e => Err(errors::internal("Unexpected result.", e)),
			}
		}))
	}
}

impl<D: Dispatcher + 'static> SofSigning for SigningQueueClient<D> {
	type Metadata = Metadata;

	fn sign(&self, meta: Metadata, address: H160, data: RpcBytes) -> BoxFuture<H520> {
		self.deprecation_notice.print("sof_sign", deprecated::msgs::ACCOUNTS);
		let res = self.dispatch(
			RpcConfirmationPayload::SofSignMessage((address.clone(), data).into()),
			meta.origin,
		);

		Box::new(res.flatten().and_then(move |response| {
			match response {
				RpcConfirmationResponse::Signature(sig) => Ok(sig),
				e => Err(errors::internal("Unexpected result.", e)),
			}
		}))
	}

	fn send_transaction(&self, meta: Metadata, request: RpcTransactionRequest) -> BoxFuture<H256> {
		self.deprecation_notice.print("sof_sendTransaction", deprecated::msgs::ACCOUNTS);
		let res = self.dispatch(
			RpcConfirmationPayload::SendTransaction(request),
			meta.origin,
		);

		Box::new(res.flatten().and_then(move |response| {
			match response {
				RpcConfirmationResponse::SendTransaction(hash) => Ok(hash),
				e => Err(errors::internal("Unexpected result.", e)),
			}
		}))
	}

	fn sign_transaction(&self, meta: Metadata, request: RpcTransactionRequest) -> BoxFuture<RpcRichRawTransaction> {
		self.deprecation_notice.print("sof_signTransaction", deprecated::msgs::ACCOUNTS);

		let res = self.dispatch(
			RpcConfirmationPayload::SignTransaction(request),
			meta.origin,
		);

		Box::new(res.flatten().and_then(move |response| {
			match response {
				RpcConfirmationResponse::SignTransaction(tx) => Ok(tx),
				e => Err(errors::internal("Unexpected result.", e)),
			}
		}))
	}
}
