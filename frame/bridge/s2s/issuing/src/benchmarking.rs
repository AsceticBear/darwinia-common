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

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use dp_asset::token::Token;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;
use sp_runtime::traits::UniqueSaturatedInto;
use sp_std::str::FromStr;
use sp_std::vec;

use crate::Pallet as Issuing;

const SEED: u32 = 0;
const SPEC_VERSION: u32 = 123;

benchmarks! {
	// dispatch handle benchmark
	dispatch_handle {
		let caller = <T as darwinia_evm::Config>::AddressMapping::into_account_id(
			H160::from_str("E1586e744b99bF8e4C981DfE4dD4369d6f8Ed88A").unwrap()
		);

		<T as Config>::RingCurrency::deposit_creating(&caller, U256::from(500).low_u128().unique_saturated_into());
		log::debug!("bear: --- benchmark: the caller is {:?}", caller);

		let mut input = vec![0; 4];
		let mut burn_action = &sha3::Keccak256::digest(&BURN_ACTION)[0..4];
		input.extend_from_slice(&mut burn_action);


		let token_info = TokenBurnInfo::encode(
			SPEC_VERSION,
			10000,
			// todo: confirm the token type
			0,
			H160::from_str("E1586e744b99bF8e4C981DfE4dD4369d6f8Ed884").unwrap(),
			H160::from_str("E1586e744b99bF8e4C981DfE4dD4369d6f8Ed885").unwrap(),
			H160::from_str("E1586e744b99bF8e4C981DfE4dD4369d6f8Ed886").unwrap(),
			vec![1; 32],
			U256::from(250),
			U256::from(10_000_000_000u128),
		);
		input.extend_from_slice(&token_info);
	}:dispatch_handle(RawOrigin::Signed(caller), input)

	// remote register benchmark
	// remote_register {
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	let token = Token::Native(TokenInfo::new(H160::default(), None, None));
	// }: remote_register(RawOrigin::Signed(caller), token)

	// // remote_issue
	// remote_issue {
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	let token = Token::Native(TokenInfo::default());
	// }: _(RawOrigin::Signed(caller), token)
}

impl_benchmark_test_suite!(Issuing, crate::tests::new_test_ext(), crate::tests::Test,);
