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

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;
use sp_std::vec;
use dp_asset::token::Token;

use crate::Pallet as Issuing;

const SEED: u32 = 0;

benchmarks! {
	// dispatch handle benchmark
	dispatch_handle {
		let caller: T::AccountId = whitelisted_caller();
		// let caller = account("caller", 4, SEED);
		log::debug!("bear: --- benchmark: the caller is {:?}", caller);
		// System::<T>::set_block_number(0u32.into());

		let token = Token::Native(TokenInfo::default());

	}:dispatch_handle(RawOrigin::Signed(caller), vec![1, 2, 3, 4, 5, 6, 7, 8])

	// remote register benchmark
	remote_register {
		let caller: T::AccountId = whitelisted_caller();
		let token = Token::Native(TokenInfo::default());
	}: remote_register(origin: OriginFor<T>, token: Token)

	// remote_issue
	remote_issue {
		let caller: T::AccountId = whitelisted_caller();
		let token = Token::Native(TokenInfo::default());
	}: _(RawOrigin::Signed(caller), token)
}

impl_benchmark_test_suite!(Issuing, crate::tests::new_test_ext(), crate::tests::Test,);
