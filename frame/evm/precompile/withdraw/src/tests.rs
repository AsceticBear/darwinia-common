#![cfg(test)]

use darwinia_evm::*;

use crate::InputData;
use codec::{Decode, Encode};
use frame_support::assert_err;
use frame_support::weights::Weight;
use frame_support::{assert_ok, impl_outer_dispatch, impl_outer_origin, parameter_types};
use sp_core::{Blake2Hasher, H160, H256, U256};
use sp_runtime::AccountId32;
use sp_runtime::DispatchError;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill, RuntimeDebug,
};
use std::{collections::BTreeMap, str::FromStr};

type Balance = u64;

impl_outer_origin! {
	pub enum Origin for Test where system = frame_system {}
}

impl_outer_dispatch! {
	pub enum OuterCall for Test where origin: Origin {
		self::EVM,
	}
}

darwinia_support::impl_test_account_data! {}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl frame_system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Call = OuterCall;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl darwinia_balances::Trait<RingInstance> for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = ();
	type ExistentialDeposit = ExistentialDeposit;
	type BalanceInfo = AccountData<Balance>;
	type AccountStore = System;
	type MaxLocks = ();
	type OtherCurrencies = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1000;
}
impl pallet_timestamp::Trait for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

/// Fixed gas price of `0`.
pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> U256 {
		// Gas price is always one token per gas.
		0.into()
	}
}

impl Trait for Test {
	type FeeCalculator = FixedGasPrice;
	type GasWeightMapping = ();
	type CallOrigin = EnsureAddressRoot<Self::AccountId>;
	type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;

	type AddressMapping = HashedAddressMapping<Blake2Hasher>;
	type Currency = Balances;

	type Event = Event<Test>;
	type Precompiles = ();
	type ChainId = SystemChainId;
	type Runner = darwinia_evm::runner::stack::Runner<Self>;
	type AccountBasicMapping = RawAccountBasicMapping<Test>;
}

type System = frame_system::Module<Test>;
type Balances = darwinia_balances::Module<Test, RingInstance>;
type EVM = Module<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	let mut accounts = BTreeMap::new();
	accounts.insert(
		H160::from_str("1000000000000000000000000000000000000001").unwrap(),
		GenesisAccount {
			nonce: U256::from(1),
			balance: U256::from(1000000),
			storage: Default::default(),
			code: vec![
				0x00, // STOP
			],
		},
	);
	accounts.insert(
		H160::from_str("1000000000000000000000000000000000000002").unwrap(),
		GenesisAccount {
			nonce: U256::from(1),
			balance: U256::from(1000000),
			storage: Default::default(),
			code: vec![
				0xff, // INVALID
			],
		},
	);

	darwinia_balances::GenesisConfig::<Test, RingInstance>::default()
		.assimilate_storage(&mut t)
		.unwrap();
	GenesisConfig { accounts }
		.assimilate_storage::<Test>(&mut t)
		.unwrap();
	t.into()
}

fn cat<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
	let mut v = Vec::with_capacity(a.len() + b.len());
	v.extend_from_slice(a);
	v.extend_from_slice(b);
	v
}

#[test]
fn test_input_encode_decode() {
	// 5EeUFyFjHsCJB8TaGXi1PkMgqkxMctcxw8hvfmNdCYGC76xj
	let dest: AccountId32 =
		hex_literal::hex!["723908ee9dc8e509d4b93251bd57f68c09bd9d04471c193fabd8f26c54284a4b"]
			.into();
	let dest_bytes: &[u8] = &dest.as_ref();
	let dest_hex = hex::encode(&dest_bytes);
	println!("dest hex {:?}", dest_hex);

	let value = U256::from(3_000_000_000u128);
	let mut value_bytes = [0u8; 32];
	value.to_little_endian(&mut value_bytes);
	let value_hex = hex::encode(&value_bytes);
	println!("value hex {:?}", value_hex);

	let input_bytes = cat(dest_bytes, &value_bytes);
	let input_decode = InputData::<Test>::decode(&input_bytes).unwrap();
	assert_eq!(input_decode.dest, dest);
	assert_eq!(input_decode.value, value);
}

#[test]
fn test_invalid_data_length_should_return_error() {
	let input_bytes = [0u8; 65];
	assert_err!(
		InputData::<Test>::decode(&input_bytes),
		DispatchError::Other("Invalid input data length"),
	);
}
