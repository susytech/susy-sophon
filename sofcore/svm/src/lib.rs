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

//! Sophon virtual machine.

extern crate bit_set;
extern crate sophon_types;
extern crate parking_lot;
extern crate heapsize;
extern crate vm;
extern crate keccak_hash as hash;
extern crate memory_cache;
extern crate susy_bytes as bytes;
extern crate num_bigint;

#[macro_use]
extern crate lazy_static;

#[cfg_attr(feature = "svm-debug", macro_use)]
extern crate log;

#[cfg(test)]
extern crate rustc_hex;

pub mod svm;
pub mod interpreter;

#[macro_use]
pub mod factory;
mod vmtype;
mod instructions;

#[cfg(test)]
mod tests;

pub use vm::{
    Schedule, CleanDustMode, EnvInfo, CallType, ActionParams, Ext,
    ContractCreateResult, MessageCallResult, CreateContractAddress,
    GasLeft, ReturnData
};
pub use self::svm::{Finalize, FinalizationResult, CostType};
pub use self::instructions::{InstructionInfo, Instruction};
pub use self::vmtype::VMType;
pub use self::factory::Factory;
