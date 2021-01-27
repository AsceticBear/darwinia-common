#![cfg_attr(not(feature = "std"), no_std)]

mod tests;

use sp_std::vec::Vec;

use darwinia_evm_primitives::Precompile;
use evm::{Context, ExitError, ExitSucceed};

use codec::Decode;
use darwinia_evm::{AccountBasicMapping, AddressMapping, CallInfo, Error, Trait};
use evm::ExitReason;
use frame_support::{
	debug, ensure,
	traits::{Currency, ExistenceRequirement},
};
use sp_core::{H160, U256};
use sp_runtime::{traits::UniqueSaturatedInto, DispatchError};
use sp_std::marker::PhantomData;
use sp_std::prelude::*;

type AccountId<T> = <T as frame_system::Trait>::AccountId;

pub struct WithDraw<T: Trait> {
	_maker: PhantomData<T>,
}

impl<T: Trait> Precompile for WithDraw<T> {
	fn execute(
		input: &[u8],
		_target_gas: Option<usize>,
		context: &Context,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		// decode input data
        let input = InputData::<T>::decode(&input).unwrap();
        debug::info!("bear: --- the input data dest {:?}", input.dest);
        debug::info!("bear: --- the input data value {:?}", input.value);
		debug::info!("bear: --- the context value {:?}", context);
		
		let helper = U256::from(10).checked_pow(U256::from(9)).unwrap_or(U256::MAX);
		let (context_value, _) = context.apparent_value.div_mod(helper);

		let caller = context.caller;
		let from = T::AddressMapping::into_account_id(caller);
		// let input_value = input.value.low_u128().unique_saturated_into();
		let context_value = context_value.low_u128().unique_saturated_into();

		let from_balance = <T as Trait>::Currency::free_balance(&from);
		let dest_balance = <T as Trait>::Currency::free_balance(&input.dest);
		debug::info!("bear: --- before transfer, from {:?}, balance {:?}", from, from_balance);
		debug::info!("bear: --- before transfer, dest {:?}, balance {:?}", input.dest, dest_balance);
		
		// let result = <T as Trait>::Currency::transfer(
		// 	&from,
		// 	&input.dest,
		// 	input_value,
		// 	ExistenceRequirement::KeepAlive,
		// );
		// a(300) -> 100 -> b(0)
		// a(300) -> 100 -> b(100)
		// 1. 进入 evm 的时候， a 会在 stack 里创建一个副本账户，所有的修改都是基于副本账户里
		// 2. evm precompile a = 200, substrate storage balance 修改主账户
		// 3. evm over, 副本账户（做了修改的）会覆盖本账户
	
		T::Currency::deposit_creating(
			&&input.dest,
			context_value,
		);
		// debug::info!("bear:--- -transfer back result {:?}", result);
		let from_balance = <T as Trait>::Currency::free_balance(&from);
		let dest_balance = <T as Trait>::Currency::free_balance(&input.dest);
		debug::info!("bear: --- before transfer, from {:?}, balance {:?}", from, from_balance);
		debug::info!("bear: --- before transfer, dest {:?}, balance {:?}", input.dest, dest_balance);

		// TODO: FIX ME LATER
		return Ok((ExitSucceed::Returned, vec![], 0))
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct InputData<T: frame_system::Trait> {
	pub dest: AccountId<T>,
	pub value: U256,
}

impl<T: frame_system::Trait> InputData<T> {
	fn decode(data: &[u8]) -> Result<Self, DispatchError> {
		if data.len() == 64 {
			let mut dest_bytes = [0u8; 32];
			dest_bytes.copy_from_slice(&data[0..32]);

			let mut value_bytes = [0u8; 32];
			value_bytes.copy_from_slice(&data[32..64]);

			return Ok(InputData {
				dest: <T as frame_system::Trait>::AccountId::decode(&mut dest_bytes.as_ref())
					.map_err(|_| DispatchError::Other("Invalid destination address"))?,
				value: U256::from_little_endian(&value_bytes),
			});
		}
		Err(DispatchError::Other("Invalid input data length"))
	}
}
