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
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use ethereum_primitives::EthereumAddress;
use sp_std::vec::Vec;

pub mod token;
pub trait BridgeAssetReceiver<R> {
	fn encode_call(token: token::Token, recipient: R) -> Result<Vec<u8>, ()>;
}

#[derive(Encode, Decode, Clone, Debug, Eq, PartialEq)]
pub enum RecipientAccount<AccountId> {
	EthereumAccount(EthereumAddress),
	DarwiniaAccount(AccountId),
}