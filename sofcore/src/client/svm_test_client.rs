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

//! Simple Client used for SVM tests.

use std::fmt;
use std::sync::Arc;
use sophon_types::{H256, U256, H160};
use {factory, journaldb, trie, susykv_memorydb};
use susykv::{self, KeyValueDB};
use {state, state_db, client, executive, trace, db, spec, pod_state};
use types::{log_entry, receipt, transaction};
use factory::Factories;
use svm::{VMType, FinalizationResult};
use vm::{self, ActionParams};
use softrie;

/// SVM test Error.
#[derive(Debug)]
pub enum SvmTestError {
	/// Trie integrity error.
	Trie(Box<softrie::TrieError>),
	/// SVM error.
	Svm(vm::Error),
	/// Initialization error.
	ClientError(::error::Error),
	/// Post-condition failure,
	PostCondition(String),
}

impl<E: Into<::error::Error>> From<E> for SvmTestError {
	fn from(err: E) -> Self {
		SvmTestError::ClientError(err.into())
	}
}

impl fmt::Display for SvmTestError {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		use self::SvmTestError::*;

		match *self {
			Trie(ref err) => write!(fmt, "Trie: {}", err),
			Svm(ref err) => write!(fmt, "SVM: {}", err),
			ClientError(ref err) => write!(fmt, "{}", err),
			PostCondition(ref err) => write!(fmt, "{}", err),
		}
	}
}

use sophon;
use sofjson::spec::ForkSpec;

/// Simplified, single-block SVM test client.
pub struct SvmTestClient<'a> {
	state: state::State<state_db::StateDB>,
	spec: &'a spec::Spec,
	dump_state: fn(&state::State<state_db::StateDB>) -> Option<pod_state::PodState>,
}

fn no_dump_state(_: &state::State<state_db::StateDB>) -> Option<pod_state::PodState> {
	None
}

impl<'a> fmt::Debug for SvmTestClient<'a> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.debug_struct("SvmTestClient")
			.field("state", &self.state)
			.field("spec", &self.spec.name)
			.finish()
	}
}

impl<'a> SvmTestClient<'a> {
	/// Converts a json spec definition into spec.
	pub fn spec_from_json(spec: &ForkSpec) -> Option<spec::Spec> {
		match *spec {
			ForkSpec::Frontier => Some(sophon::new_frontier_test()),
			ForkSpec::Homestead => Some(sophon::new_homestead_test()),
			ForkSpec::SIP150 => Some(sophon::new_sip150_test()),
			ForkSpec::SIP158 => Some(sophon::new_sip161_test()),
			ForkSpec::Byzantium => Some(sophon::new_byzantium_test()),
			ForkSpec::Constantinople => Some(sophon::new_constantinople_test()),
			ForkSpec::ConstantinopleFix => Some(sophon::new_constantinople_fix_test()),
			ForkSpec::SIP158ToByzantiumAt5 => Some(sophon::new_transition_test()),
			ForkSpec::FrontierToHomesteadAt5 | ForkSpec::HomesteadToDaoAt5 | ForkSpec::HomesteadToSIP150At5 => None,
		}
	}

	/// Change default function for dump state (default does not dump)
	pub fn set_dump_state_fn(&mut self, dump_state: fn(&state::State<state_db::StateDB>) -> Option<pod_state::PodState>) {
		self.dump_state = dump_state;
	}

	/// Creates new SVM test client with in-memory DB initialized with genesis of given Spec.
	/// Takes a `TrieSpec` to set the type of trie.
	pub fn new_with_trie(spec: &'a spec::Spec, trie_spec: trie::TrieSpec) -> Result<Self, SvmTestError> {
		let factories = Self::factories(trie_spec);
		let state =	Self::state_from_spec(spec, &factories)?;

		Ok(SvmTestClient {
			state,
			spec,
			dump_state: no_dump_state,
		})
	}

	/// Creates new SVM test client with an in-memory DB initialized with genesis of given chain Spec.
	pub fn new(spec: &'a spec::Spec) -> Result<Self, SvmTestError> {
		Self::new_with_trie(spec, trie::TrieSpec::Secure)
	}

	/// Creates new SVM test client with an in-memory DB initialized with given PodState.
	/// Takes a `TrieSpec` to set the type of trie.
	pub fn from_pod_state_with_trie(spec: &'a spec::Spec, pod_state: pod_state::PodState, trie_spec: trie::TrieSpec) -> Result<Self, SvmTestError> {
		let factories = Self::factories(trie_spec);
		let state =	Self::state_from_pod(spec, &factories, pod_state)?;

		Ok(SvmTestClient {
			state,
			spec,
			dump_state: no_dump_state,
		})
	}

	/// Creates new SVM test client with an in-memory DB initialized with given PodState.
	pub fn from_pod_state(spec: &'a spec::Spec, pod_state: pod_state::PodState) -> Result<Self, SvmTestError> {
		Self::from_pod_state_with_trie(spec, pod_state, trie::TrieSpec::Secure)
	}

	fn factories(trie_spec: trie::TrieSpec) -> Factories {
		Factories {
			vm: factory::VmFactory::new(VMType::Interpreter, 5 * 1024),
			trie: trie::TrieFactory::new(trie_spec),
			accountdb: Default::default(),
		}
	}

	fn state_from_spec(spec: &'a spec::Spec, factories: &Factories) -> Result<state::State<state_db::StateDB>, SvmTestError> {
		let db = Arc::new(susykv_memorydb::create(db::NUM_COLUMNS.expect("We use column-based DB; qed")));
		let journal_db = journaldb::new(db.clone(), journaldb::Algorithm::EarlyMerge, db::COL_STATE);
		let mut state_db = state_db::StateDB::new(journal_db, 5 * 1024 * 1024);
		state_db = spec.ensure_db_good(state_db, factories)?;

		let genesis = spec.genesis_header();
		// Write DB
		{
			let mut batch = susykv::DBTransaction::new();
			state_db.journal_under(&mut batch, 0, &genesis.hash())?;
			db.write(batch)?;
		}

		state::State::from_existing(
			state_db,
			*genesis.state_root(),
			spec.engine.account_start_nonce(0),
			factories.clone()
		).map_err(SvmTestError::Trie)
	}

	fn state_from_pod(spec: &'a spec::Spec, factories: &Factories, pod_state: pod_state::PodState) -> Result<state::State<state_db::StateDB>, SvmTestError> {
		let db = Arc::new(susykv_memorydb::create(db::NUM_COLUMNS.expect("We use column-based DB; qed")));
		let journal_db = journaldb::new(db.clone(), journaldb::Algorithm::EarlyMerge, db::COL_STATE);
		let state_db = state_db::StateDB::new(journal_db, 5 * 1024 * 1024);
		let mut state = state::State::new(
			state_db,
			spec.engine.account_start_nonce(0),
			factories.clone(),
		);
		state.populate_from(pod_state);
		state.commit()?;
		Ok(state)
	}

	/// Return current state.
	pub fn state(&self) -> &state::State<state_db::StateDB> {
		&self.state
	}

	/// Execute the VM given ActionParams and tracer.
	/// Returns amount of gas left and the output.
	pub fn call<T: trace::Tracer, V: trace::VMTracer>(
		&mut self,
		params: ActionParams,
		tracer: &mut T,
		vm_tracer: &mut V,
	) -> Result<FinalizationResult, SvmTestError>
	{
		let genesis = self.spec.genesis_header();
		let info = client::EnvInfo {
			number: genesis.number(),
			author: *genesis.author(),
			timestamp: genesis.timestamp(),
			difficulty: *genesis.difficulty(),
			last_hashes: Arc::new([H256::default(); 256].to_vec()),
			gas_used: 0.into(),
			gas_limit: *genesis.gas_limit(),
		};
		self.call_envinfo(params, tracer, vm_tracer, info)
	}

	/// Execute the VM given envinfo, ActionParams and tracer.
	/// Returns amount of gas left and the output.
	pub fn call_envinfo<T: trace::Tracer, V: trace::VMTracer>(
		&mut self,
		params: ActionParams,
		tracer: &mut T,
		vm_tracer: &mut V,
		info: client::EnvInfo,
	) -> Result<FinalizationResult, SvmTestError>
	{
		let mut substate = state::Substate::new();
		let machine = self.spec.engine.machine();
		let schedule = machine.schedule(info.number);
		let mut executive = executive::Executive::new(&mut self.state, &info, &machine, &schedule);
		executive.call(
			params,
			&mut substate,
			tracer,
			vm_tracer,
		).map_err(SvmTestError::Svm)
	}

	/// Executes a SignedTransaction within context of the provided state and `EnvInfo`.
	/// Returns the state root, gas left and the output.
	pub fn transact<T: trace::Tracer, V: trace::VMTracer>(
		&mut self,
		env_info: &client::EnvInfo,
		transaction: transaction::SignedTransaction,
		tracer: T,
		vm_tracer: V,
	) -> TransactResult<T::Output, V::Output> {
		let initial_gas = transaction.gas;
		// Verify transaction
		let is_ok = transaction.verify_basic(true, None, false);
		if let Err(error) = is_ok {
			return TransactResult::Err {
				state_root: *self.state.root(),
				error: error.into(),
				end_state: (self.dump_state)(&self.state),
			};
		}

		// Apply transaction
		let result = self.state.apply_with_tracing(&env_info, self.spec.engine.machine(), &transaction, tracer, vm_tracer);
		let scheme = self.spec.engine.machine().create_address_scheme(env_info.number);

		// Touch the coinbase at the end of the test to simulate
		// miner reward.
		// Details: https://octonion.institute/susytech/susy-sophon/issues/9431
		let schedule = self.spec.engine.machine().schedule(env_info.number);
		self.state.add_balance(&env_info.author, &0.into(), if schedule.no_empty {
			state::CleanupMode::NoEmpty
		} else {
			state::CleanupMode::ForceCreate
		}).ok();
		// Touching also means that we should remove the account if it's within sip161
		// conditions.
		self.state.kill_garbage(
			&vec![env_info.author].into_iter().collect(),
			schedule.kill_empty,
			&None,
			false
		).ok();

		self.state.commit().ok();

		let state_root = *self.state.root();

		let end_state = (self.dump_state)(&self.state);

		match result {
			Ok(result) => {
				TransactResult::Ok {
					state_root,
					gas_left: initial_gas - result.receipt.gas_used,
					outcome: result.receipt.outcome,
					output: result.output,
					trace: result.trace,
					vm_trace: result.vm_trace,
					logs: result.receipt.logs,
					contract_address: if let transaction::Action::Create = transaction.action {
						Some(executive::contract_address(scheme, &transaction.sender(), &transaction.nonce, &transaction.data).0)
					} else {
						None
					},
					end_state,
				}
			},
			Err(error) => TransactResult::Err {
				state_root,
				error,
				end_state,
			},
		}
	}
}

/// A result of applying transaction to the state.
#[derive(Debug)]
pub enum TransactResult<T, V> {
	/// Successful execution
	Ok {
		/// State root
		state_root: H256,
		/// Amount of gas left
		gas_left: U256,
		/// Output
		output: Vec<u8>,
		/// Traces
		trace: Vec<T>,
		/// VM Traces
		vm_trace: Option<V>,
		/// Created contract address (if any)
		contract_address: Option<H160>,
		/// Generated logs
		logs: Vec<log_entry::LogEntry>,
		/// outcome
		outcome: receipt::TransactionOutcome,
		/// end state if needed
		end_state: Option<pod_state::PodState>,
	},
	/// Transaction failed to run
	Err {
		/// State root
		state_root: H256,
		/// Execution error
		error: ::error::Error,
		/// end state if needed
		end_state: Option<pod_state::PodState>,
	},
}
