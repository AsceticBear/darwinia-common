// This file is part of Darwinia.
//
// Copyright (C) 2018-2021 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{ensure, traits::{Currency, ExistenceRequirement}};
use sp_core::{U256, H160};
use sp_runtime::traits::UniqueSaturatedInto;
use sp_std::marker::PhantomData;
use sp_std::prelude::*;
use sp_std::vec::Vec;

use codec::Decode;
use darwinia_evm::{AddressMapping, Trait};
use darwinia_evm_primitives::Precompile;
use evm::{Context, ExitError, ExitSucceed};

type AccountId<T> = <T as frame_system::Trait>::AccountId;

use frame_support::debug;

/// WithDraw Precompile Contract, used to withdraw balance from evm account to darwinia account
///
/// The contract address: 0000000000000000000000000000000000000005
pub struct WithDraw<T: Trait> {
	_maker: PhantomData<T>,
}

impl<T: Trait> Precompile for WithDraw<T> {
	/// The Withdraw process is divided into two part:
	/// 1. parse the withdrawal address and amount from the input parameter and get the caller address from the context
	/// 2. transfer from caller to withdrawal address
	///
	/// Input data encode rule:
	/// Part1: 32-bit prefix code, used for smart contract calls
	/// Part2: 32-bit substrate withdrawal public key
	/// Part3: 32-bit withdrawal amount, is encoded with U256 little-endian format
	fn execute(
		input: &[u8],
		_: Option<usize>,
		context: &Context,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		// Decode input data
		let input = InputData::<T>::decode(&input)?;
		debug::info!("bear: -- the input account {:?}", input.dest);
		// debug::info!("bear: -- the input value {:?}", input.value);
		debug::info!("bear: -- the context info {:?}", context);

		let helper = U256::from(10)
			.checked_pow(U256::from(9))
			.unwrap_or(U256::MAX);
		// let value = input.value.saturating_mul(helper);
		let contract_address = T::AddressMapping::into_account_id(context.address);
		let from_address = T::AddressMapping::into_account_id(context.caller);
		
		// use sp_std::str::FromStr;
		// let precompile_evm = H160::from_str("0000000000000000000000000000000000000005").unwrap();
		// let deploy_contract_evm = H160::from_str("fca29754bb5d5a60f96cfba820f7e76a8e1b6f97").unwrap();
		// let precompile_sub = T::AddressMapping::into_account_id(precompile_evm);
		// let deploy_contract_sub = T::AddressMapping::into_account_id(contract_address);
		let contract_balance = <T as Trait>::Currency::free_balance(&contract_address);
		// let deploy_contract_balance = <T as Trait>::Currency::free_balance(&deploy_contract_sub);
		let from_balance = <T as Trait>::Currency::free_balance(&from_address);
		let dest_balance = <T as Trait>::Currency::free_balance(&input.dest);
		debug::info!("bear: --- before transfer, from {:?}, balance {:?}", context.caller, from_balance);
		debug::info!("bear: --- before transfer, dest {:?}, balance {:?}", input.dest, dest_balance);
		debug::info!("bear: --- before transfer, cons {:?}, balance {:?}", context.address, contract_balance);

		// let result = T::Currency::transfer(
		// 	&from_address,
		// 	&input.dest,
		// 	value.low_u128().unique_saturated_into(),
		// 	ExistenceRequirement::AllowDeath,
		// );
		let (context_value, _) = context.apparent_value.div_mod(helper);
		let context_value = context_value.low_u128().unique_saturated_into();

		ensure!(contract_balance >= context_value, ExitError::Other("The contract balance should larger than context value".into()));
		// Slash precompile value
		T::Currency::slash(&contract_address, context_value);
		ensure!(<T as Trait>::Currency::free_balance(&contract_address) == 0u128.unique_saturated_into(), ExitError::Other("The contract balance should be zero after slash".into()));
		// Deposit create withdrawal target address
		T::Currency::deposit_creating(
			&&input.dest,
			context_value,
		);

		let from_balance = <T as Trait>::Currency::free_balance(&from_address);
		let dest_balance = <T as Trait>::Currency::free_balance(&input.dest);
		let contract_balance = <T as Trait>::Currency::free_balance(&contract_address);
		debug::info!("bear: --- after transfer, from {:?}, balance {:?}", context.caller, from_balance);
		debug::info!("bear: --- after transfer, dest {:?}, balance {:?}", input.dest, dest_balance);
		debug::info!("bear: --- after transfer, cons {:?}, balance {:?}", context.address, contract_balance);

		// match result {
		// 	Ok(()) => Ok((ExitSucceed::Returned, vec![], 10000)),
			// Err(error) => match error {
			// 	sp_runtime::DispatchError::BadOrigin => Err(ExitError::Other("BadOrigin".into())),
			// 	sp_runtime::DispatchError::CannotLookup => {
			// 		Err(ExitError::Other("CannotLookup".into()))
			// 	}
			// 	sp_runtime::DispatchError::Other(message) => Err(ExitError::Other(message.into())),
			// 	sp_runtime::DispatchError::Module { message, .. } => {
			// 		Err(ExitError::Other(message.unwrap_or("Module Error").into()))
			// 	}
			// },
		// }
		Ok((ExitSucceed::Returned, vec![], 10000))
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct InputData<T: frame_system::Trait> {
	pub dest: AccountId<T>,
	// pub value: U256,
}

impl<T: frame_system::Trait> InputData<T> {
	pub fn decode(data: &[u8]) -> Result<Self, ExitError> {
		debug::info!("bear: -- the data {:?}, data len {:?}", data, data.len());
		if data.len() == 32 {
			let mut dest_bytes = [0u8; 32];
			dest_bytes.copy_from_slice(&data[0..32]);
			debug::info!("bear: -- the dest bytes {:?}", dest_bytes) ;
			
			// let mut value_bytes = [0u8; 32];
			// value_bytes.copy_from_slice(&data[32..64]);
			// debug::info!("bear: -- the value bytes {:?}", value_bytes) ;

			return Ok(InputData {
				dest: <T as frame_system::Trait>::AccountId::decode(&mut dest_bytes.as_ref())
					.map_err(|_| ExitError::Other("Invalid destination address".into()))?,
				// value: U256::from_big_endian(&value_bytes),
			});
		}
		Err(ExitError::Other("Invalid input data length".into()))
	}
}
