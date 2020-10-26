//! Tests for ethereum-backing.

// --- substrate ---
use frame_support::{assert_err, assert_ok, traits::Contains};
use frame_system::EnsureRoot;
use sp_runtime::{traits::Dispatchable, AccountId32};
// --- darwinia ---
use crate::*;
use array_bytes::hex_bytes_unchecked;
use darwinia_ethereum_relay::{EthereumRelayHeaderParcel, EthereumRelayProofs, MMRProof};
use darwinia_relay_primitives::*;
use darwinia_staking::{RewardDestination, StakingBalance, StakingLedger, TimeDepositItem};
use darwinia_support::balance::lock::StakingLock;
use ethereum_primitives::{
	header::EthereumHeader, receipt::EthereumReceiptProof, EthereumBlockNumber, EthereumNetworkType,
};

type EthereumRelay = darwinia_ethereum_relay::Module<Test>;

decl_tests!();

pub struct UnusedTechnicalMembership;
impl Contains<AccountId> for UnusedTechnicalMembership {
	fn sorted_members() -> Vec<AccountId> {
		unimplemented!()
	}
}
parameter_types! {
	pub const EthereumRelayModuleId: ModuleId = ModuleId(*b"da/ethrl");
	pub const EthereumNetwork: EthereumNetworkType = EthereumNetworkType::Ropsten;
}
impl darwinia_ethereum_relay::Trait for Test {
	type ModuleId = EthereumRelayModuleId;
	type Event = ();
	type EthereumNetwork = EthereumNetwork;
	type RelayerGame = UnusedRelayerGame;
	type ApproveOrigin = EnsureRoot<AccountId>;
	type RejectOrigin = EnsureRoot<AccountId>;
	type ConfirmPeriod = ();
	type TechnicalMembership = UnusedTechnicalMembership;
	type ApproveThreshold = ();
	type RejectThreshold = ();
	type Call = Call;
	type Currency = Ring;
	type WeightInfo = ();
}

pub struct UnusedRelayerGame;
impl RelayerGameProtocol for UnusedRelayerGame {
	type Relayer = AccountId;
	type RelayHeaderId = EthereumBlockNumber;
	type RelayHeaderParcel = EthereumRelayHeaderParcel;
	type RelayProofs = EthereumRelayProofs;

	fn get_proposed_relay_header_parcels(
		_: RelayAffirmationId<Self::RelayHeaderId>,
	) -> Option<Vec<Self::RelayHeaderParcel>> {
		unimplemented!()
	}
	fn best_confirmed_header_id_of(_: &Self::RelayHeaderId) -> Self::RelayHeaderId {
		unimplemented!()
	}
	fn affirm(
		_: &Self::Relayer,
		_: Self::RelayHeaderParcel,
		_: Option<Self::RelayProofs>,
	) -> Result<Self::RelayHeaderId, DispatchError> {
		unimplemented!()
	}
	fn dispute_and_affirm(
		_: &Self::Relayer,
		_: Self::RelayHeaderParcel,
		_: Option<Self::RelayProofs>,
	) -> Result<(Self::RelayHeaderId, u32), DispatchError> {
		unimplemented!()
	}
	fn complete_relay_proofs(
		_: RelayAffirmationId<Self::RelayHeaderId>,
		_: Vec<Self::RelayProofs>,
	) -> DispatchResult {
		unimplemented!()
	}
	fn extend_affirmation(
		_: &Self::Relayer,
		_: RelayAffirmationId<Self::RelayHeaderId>,
		_: Vec<Self::RelayHeaderParcel>,
		_: Option<Vec<Self::RelayProofs>>,
	) -> Result<(Self::RelayHeaderId, u32, u32), DispatchError> {
		unimplemented!()
	}
}

pub struct ExtBuilder;
impl Default for ExtBuilder {
	fn default() -> Self {
		Self
	}
}
impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap();

		GenesisConfig::<Test> {
			token_redeem_address: fixed_hex_bytes_unchecked!(
				"0x49262B932E439271d05634c32978294C7Ea15d0C",
				20
			)
			.into(),
			deposit_redeem_address: fixed_hex_bytes_unchecked!(
				"0x6EF538314829EfA8386Fc43386cB13B4e0A67D1e",
				20
			)
			.into(),
			ring_token_address: fixed_hex_bytes_unchecked!(
				"0xb52FBE2B925ab79a821b261C82c5Ba0814AAA5e0",
				20
			)
			.into(),
			kton_token_address: fixed_hex_bytes_unchecked!(
				"0x1994100c58753793D52c6f457f189aa3ce9cEe94",
				20
			)
			.into(),
			ring_locked: 20000000000000,
			kton_locked: 5000000000000,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}

#[cfg_attr(test, derive(serde::Deserialize))]
#[derive(Clone, PartialEq, Encode, Decode, RuntimeDebug)]
pub struct TestReceiptProofThing {
	pub header: EthereumHeader,
	pub receipt_proof: EthereumReceiptProof,
	pub mmr_proof: MMRProof,
}

#[test]
fn genesis_config_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(EthereumBacking::pot::<Ring>(), 20000000000000);
		assert_eq!(EthereumBacking::pot::<Kton>(), 5000000000000);
	});
}

#[test]
fn verify_parse_token_redeem_proof() {
	ExtBuilder::default()
		.build()
		.execute_with(|| {
			// https://ropsten.etherscan.io/tx/0x1d3ef601b9fa4a7f1d6259c658d0a10c77940fa5db9e10ab55397eb0ce88807d
			let test_proof_relay_header_parcel : TestReceiptProofThing =  serde_json::from_str(r#"{"header":{"parent_hash":"0xd55ce7660d0161c38b34015ce5468e1661f1c77865f23415e246ac9ccf7b2b22","timestamp":1599124448,"number":8610261,"author":"0xad87c0e80ab5e13f15757d5139cc6c6fcb823be3","transactions_root":"0x6cf40dbc3f8ce55ffc0f863a65ffd285da787b77af952c319e2577c5ab278a3a","uncles_hash":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","extra_data":"0xde830207028f5061726974792d457468657265756d86312e34312e30826c69","state_root":"0xf108de6fa8fbf18e795cb3e1911d32e7d26178a69bb2f365662c8cba58bd7159","receipts_root":"0xfdb6d2cdb6bc9e78711a008b2912e62db28de4520dfcd49bd6f7086718c5d0cb","log_bloom":"0x00000000008000000000002000000004400000000000001000000000000000002000000000080000000010000040000000000001000000400000000000000000000000000000000011000008000000000210000000000000400000000000000000000000020000008104000000080800080000000000000000000030000000400004000000000001000000040000008000000002004000810010000000200000002000000000200000000000800000040010000400000200080000000000000000000402000000000200010000040000000001000001200800000000000020000000000000000000000000000010010000800000000080010000000004000000","gas_used":954909,"gas_limit":8000029,"difficulty":515540132,"seal":["0xa0deadf98810a6ccfb8d00e8f6bc7ad7f5d62d5a42760c0d9db8a549df76697704","0x88bf7326c26c57c69c"],"hash":"0xabf627ce77d9f92a40f34e3cace721c3f089000dae820d00d3e99314c263a0c3"},"receipt_proof":{"index":18,"proof":"0xf90654f90651b873f871a08905d6a9a81124e73b632ff8e0ac638331d4aa0f89bc5b296b5132ab1e6db295a0296366ce16b627f71457cefa27e7cbd6aa3f13ce1e2225bb06236089d5363667808080808080a0c73f3d756add498b44b70ae5d5b917fcc8c3adb72f10cc5cd245a862d9a2a17d8080808080808080b873f871a039af78839760433d410ab11ef453e8656451a51575e0be71fa7a72b54bfc296aa098d3ce8768b102d89494e55dde408e28d1ec148affca70a955d2b30bd9fcf008a078eacce43297ddfa328b51d92ccd8666abf1df855cef7ab3b1f4dbd16874ff658080808080808080808080808080b90564f9056120b9055df9055a01830e921db9010000000000008000000000002000000004400000000000001000000000000000000000000000000000000010000000000000000000000000400000000000000000000000000000000010000008000000000010000000000000000000000000000000000000020000000104000000000800080000000000000000000010000000400000000000000001000000000000008000000002004000000010000000200000000000000000000000000000800000000000000000000200080000000000000000000002000000000000000000040000000001000000000800000000000020000000000000000000000000000010000000000000000080000000000000000000f9044ff89b94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5a000000000000000000000000049262b932e439271d05634c32978294c7ea15d0ca00000000000000000000000000000000000000000000000001121d33597384000f89b94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5a00000000000000000000000007f5b598827359939606b3525712fb124a1c7851da00000000000000000000000000000000000000000000000001bc16d674ec80000f87a94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f842a0cc16f5dbb4873280815c1ee09dbd06736cffcc184412cf7a71a0fdb75d397ca5a000000000000000000000000049262b932e439271d05634c32978294c7ea15d0ca00000000000000000000000000000000000000000000000001121d33597384000f89b94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000049262b932e439271d05634c32978294c7ea15d0ca00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000001121d33597384000f8fc9449262b932e439271d05634c32978294c7ea15d0cf863a0c9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10a0000000000000000000000000b52fbe2b925ab79a821b261c82c5ba0814aaa5e0a0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5b8800000000000000000000000000000000000000000000000001121d3359738400000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000020e44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318f8fc94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f863a09bfafdc2ae8835972d7b64ef3f8f307165ac22ceffde4a742c52da5487f45fd1a0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5a000000000000000000000000049262b932e439271d05634c32978294c7ea15d0cb8800000000000000000000000000000000000000000000000001121d3359738400000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000020e44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318","header_hash":"0xabf627ce77d9f92a40f34e3cace721c3f089000dae820d00d3e99314c263a0c3"},"mmr_proof":{"member_leaf_index":8610261,"last_leaf_index":8610261,"proof":["0x36f3d834cbe12a5a20b063c432b88f5506bdce03b93fa3aa035a5d82fd50177c","0x541f71c116b054b7cf0f35461b41abdc0bc6e461ae54862c8dc3af6b48742a35","0x095f1b66aed2db73567706e227981454a80a442aa69b00aedaef787fd5401a78","0x942cfd846d464575b520554dd07fd4b07ba2cb8bee6234530db1c1b3a69e1c50","0x3174f9fdf5f7c9df6ac422a0a9e2aec05ccb7968bb1ff19b6cc0b64fbc930e77","0x5624f44ff885dcdc971303281bb76c6b6a957a6cc2d244b30f736fc6220f9b89","0xa3adc2ee02f7ceda7e63df8fb58094769afd3f7657c8f9f45cdc69d5c868b468","0xe588926a1643dc04b197a7ed30dd47d42d35fdb01f7aa80b2e293b058c11c6a1","0x25d7ac48be9565512424ef559b7e0ec7a91bfe9b57440913712a174e35eb47d8","0xf4505f26b22ae59f5560003a68a66e8c07a9f77f212e05ea060018bd0acd1f55","0xd55ce7660d0161c38b34015ce5468e1661f1c77865f23415e246ac9ccf7b2b22"]}}"#).unwrap();
			let ethereum_proof_relay_header_parcel = (test_proof_relay_header_parcel.header, test_proof_relay_header_parcel.receipt_proof, test_proof_relay_header_parcel.mmr_proof);
			let relay_header_parcel : EthereumRelayHeaderParcel = serde_json::from_str(r#"{"header":{"parent_hash":"0xabf627ce77d9f92a40f34e3cace721c3f089000dae820d00d3e99314c263a0c3","timestamp":1599124463,"number":8610262,"author":"0x52351e33b3c693cc05f21831647ebdab8a68eb95","transactions_root":"0xc6957c52dcac0eb2bda32765cc7c2b13406cf040021ea5caa5ba974d7a81fc95","uncles_hash":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","extra_data":"0x6c6f746f706f6f6c","state_root":"0x20a368875c1aa9f46a7be4de7b470b92440684d0d74416b77ace846a31d57ce6","receipts_root":"0x2bbd8af877b41e2cdff6e69dc100c69b061c874c3898e073ac24648ac9cd7d02","log_bloom":"0x00000000000000000000000000000000000000000000000000800000000000000000000000010000000000000000004000000000000000020000010000000000001000000000000000000008000000000001000000000000000000000000000000000000020000000000000000000800000000000000000000000010000000400000000000001000000000000000000000000000001000000000100000000000000000000000000000000000000000000000000000800000004000000080000000080002000008000000000040000000200200000000000002000000000020000000010000000000000000000000000000000000000000000000000000000000","gas_used":6717322,"gas_limit":8000000,"difficulty":515540132,"seal":["0xa046b0cc502bfed25a8700f53d6695eeed640b80218855b1dd5479c98a7b0e3e58","0x880011000003407250"],"hash":"0xd04a94f4ec0a29f53b40fb13a660b680c8b66ccbb4e864a3727535f667a07bb1"},"mmr_root":"0x95866493e3fd63b6a19c7815cc5cecaae57d5d7a3f1cc12f45e0028b090c0451"}"#).unwrap();

			assert_ok!(EthereumRelay::store_relay_header_parcel(relay_header_parcel));

			let expect_account_id = EthereumBacking::account_id_try_from_bytes(
				&hex_bytes_unchecked("0xe44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318"),
			).unwrap();

			assert_eq!(
				EthereumBacking::parse_token_redeem_proof(&ethereum_proof_relay_header_parcel),
				Ok((expect_account_id, (true, 1234500000), 0)),
			);
		});
}

#[test]
fn verify_redeem_ring() {
	ExtBuilder::default()
		.build()
		.execute_with(|| {

			// https://ropsten.etherscan.io/tx/0x1d3ef601b9fa4a7f1d6259c658d0a10c77940fa5db9e10ab55397eb0ce88807d
			let test_proof_relay_header_parcel : TestReceiptProofThing =  serde_json::from_str(r#"{"header":{"parent_hash":"0xd55ce7660d0161c38b34015ce5468e1661f1c77865f23415e246ac9ccf7b2b22","timestamp":1599124448,"number":8610261,"author":"0xad87c0e80ab5e13f15757d5139cc6c6fcb823be3","transactions_root":"0x6cf40dbc3f8ce55ffc0f863a65ffd285da787b77af952c319e2577c5ab278a3a","uncles_hash":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","extra_data":"0xde830207028f5061726974792d457468657265756d86312e34312e30826c69","state_root":"0xf108de6fa8fbf18e795cb3e1911d32e7d26178a69bb2f365662c8cba58bd7159","receipts_root":"0xfdb6d2cdb6bc9e78711a008b2912e62db28de4520dfcd49bd6f7086718c5d0cb","log_bloom":"0x00000000008000000000002000000004400000000000001000000000000000002000000000080000000010000040000000000001000000400000000000000000000000000000000011000008000000000210000000000000400000000000000000000000020000008104000000080800080000000000000000000030000000400004000000000001000000040000008000000002004000810010000000200000002000000000200000000000800000040010000400000200080000000000000000000402000000000200010000040000000001000001200800000000000020000000000000000000000000000010010000800000000080010000000004000000","gas_used":954909,"gas_limit":8000029,"difficulty":515540132,"seal":["0xa0deadf98810a6ccfb8d00e8f6bc7ad7f5d62d5a42760c0d9db8a549df76697704","0x88bf7326c26c57c69c"],"hash":"0xabf627ce77d9f92a40f34e3cace721c3f089000dae820d00d3e99314c263a0c3"},"receipt_proof":{"index":18,"proof":"0xf90654f90651b873f871a08905d6a9a81124e73b632ff8e0ac638331d4aa0f89bc5b296b5132ab1e6db295a0296366ce16b627f71457cefa27e7cbd6aa3f13ce1e2225bb06236089d5363667808080808080a0c73f3d756add498b44b70ae5d5b917fcc8c3adb72f10cc5cd245a862d9a2a17d8080808080808080b873f871a039af78839760433d410ab11ef453e8656451a51575e0be71fa7a72b54bfc296aa098d3ce8768b102d89494e55dde408e28d1ec148affca70a955d2b30bd9fcf008a078eacce43297ddfa328b51d92ccd8666abf1df855cef7ab3b1f4dbd16874ff658080808080808080808080808080b90564f9056120b9055df9055a01830e921db9010000000000008000000000002000000004400000000000001000000000000000000000000000000000000010000000000000000000000000400000000000000000000000000000000010000008000000000010000000000000000000000000000000000000020000000104000000000800080000000000000000000010000000400000000000000001000000000000008000000002004000000010000000200000000000000000000000000000800000000000000000000200080000000000000000000002000000000000000000040000000001000000000800000000000020000000000000000000000000000010000000000000000080000000000000000000f9044ff89b94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5a000000000000000000000000049262b932e439271d05634c32978294c7ea15d0ca00000000000000000000000000000000000000000000000001121d33597384000f89b94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5a00000000000000000000000007f5b598827359939606b3525712fb124a1c7851da00000000000000000000000000000000000000000000000001bc16d674ec80000f87a94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f842a0cc16f5dbb4873280815c1ee09dbd06736cffcc184412cf7a71a0fdb75d397ca5a000000000000000000000000049262b932e439271d05634c32978294c7ea15d0ca00000000000000000000000000000000000000000000000001121d33597384000f89b94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000049262b932e439271d05634c32978294c7ea15d0ca00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000001121d33597384000f8fc9449262b932e439271d05634c32978294c7ea15d0cf863a0c9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10a0000000000000000000000000b52fbe2b925ab79a821b261c82c5ba0814aaa5e0a0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5b8800000000000000000000000000000000000000000000000001121d3359738400000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000020e44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318f8fc94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f863a09bfafdc2ae8835972d7b64ef3f8f307165ac22ceffde4a742c52da5487f45fd1a0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5a000000000000000000000000049262b932e439271d05634c32978294c7ea15d0cb8800000000000000000000000000000000000000000000000001121d3359738400000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000020e44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318","header_hash":"0xabf627ce77d9f92a40f34e3cace721c3f089000dae820d00d3e99314c263a0c3"},"mmr_proof":{"member_leaf_index":8610261,"last_leaf_index":8610261,"proof":["0x36f3d834cbe12a5a20b063c432b88f5506bdce03b93fa3aa035a5d82fd50177c","0x541f71c116b054b7cf0f35461b41abdc0bc6e461ae54862c8dc3af6b48742a35","0x095f1b66aed2db73567706e227981454a80a442aa69b00aedaef787fd5401a78","0x942cfd846d464575b520554dd07fd4b07ba2cb8bee6234530db1c1b3a69e1c50","0x3174f9fdf5f7c9df6ac422a0a9e2aec05ccb7968bb1ff19b6cc0b64fbc930e77","0x5624f44ff885dcdc971303281bb76c6b6a957a6cc2d244b30f736fc6220f9b89","0xa3adc2ee02f7ceda7e63df8fb58094769afd3f7657c8f9f45cdc69d5c868b468","0xe588926a1643dc04b197a7ed30dd47d42d35fdb01f7aa80b2e293b058c11c6a1","0x25d7ac48be9565512424ef559b7e0ec7a91bfe9b57440913712a174e35eb47d8","0xf4505f26b22ae59f5560003a68a66e8c07a9f77f212e05ea060018bd0acd1f55","0xd55ce7660d0161c38b34015ce5468e1661f1c77865f23415e246ac9ccf7b2b22"]}}"#).unwrap();
			let ethereum_proof_relay_header_parcel = (test_proof_relay_header_parcel.header, test_proof_relay_header_parcel.receipt_proof, test_proof_relay_header_parcel.mmr_proof);
			let relay_header_parcel : EthereumRelayHeaderParcel = serde_json::from_str(r#"{"header":{"parent_hash":"0xabf627ce77d9f92a40f34e3cace721c3f089000dae820d00d3e99314c263a0c3","timestamp":1599124463,"number":8610262,"author":"0x52351e33b3c693cc05f21831647ebdab8a68eb95","transactions_root":"0xc6957c52dcac0eb2bda32765cc7c2b13406cf040021ea5caa5ba974d7a81fc95","uncles_hash":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","extra_data":"0x6c6f746f706f6f6c","state_root":"0x20a368875c1aa9f46a7be4de7b470b92440684d0d74416b77ace846a31d57ce6","receipts_root":"0x2bbd8af877b41e2cdff6e69dc100c69b061c874c3898e073ac24648ac9cd7d02","log_bloom":"0x00000000000000000000000000000000000000000000000000800000000000000000000000010000000000000000004000000000000000020000010000000000001000000000000000000008000000000001000000000000000000000000000000000000020000000000000000000800000000000000000000000010000000400000000000001000000000000000000000000000001000000000100000000000000000000000000000000000000000000000000000800000004000000080000000080002000008000000000040000000200200000000000002000000000020000000010000000000000000000000000000000000000000000000000000000000","gas_used":6717322,"gas_limit":8000000,"difficulty":515540132,"seal":["0xa046b0cc502bfed25a8700f53d6695eeed640b80218855b1dd5479c98a7b0e3e58","0x880011000003407250"],"hash":"0xd04a94f4ec0a29f53b40fb13a660b680c8b66ccbb4e864a3727535f667a07bb1"},"mmr_root":"0x95866493e3fd63b6a19c7815cc5cecaae57d5d7a3f1cc12f45e0028b090c0451"}"#).unwrap();

			assert_ok!(EthereumRelay::store_relay_header_parcel(relay_header_parcel));

			let expect_account_id = EthereumBacking::account_id_try_from_bytes(
				&hex_bytes_unchecked("0xe44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318"),
			).unwrap();
			let id1 = AccountId32::from([0; 32]);
			let ring_locked_before = EthereumBacking::pot::<Ring>();
			let _ = Ring::deposit_creating(&expect_account_id, 1);

			assert_ok!(EthereumBacking::redeem(
				Origin::signed(id1.clone()),
				RedeemFor::Token,
				ethereum_proof_relay_header_parcel.clone()
			));
			assert_eq!(Ring::free_balance(&expect_account_id), 1234500000 + 1);

			let ring_locked_after = EthereumBacking::pot::<Ring>();
			assert_eq!(ring_locked_after + 1234500000, ring_locked_before);

			// shouldn't redeem twice
			assert_err!(
				EthereumBacking::redeem(Origin::signed(id1.clone()), RedeemFor::Token, ethereum_proof_relay_header_parcel),
				<Error<Test>>::AssetAR,
			);
		});
}

#[test]
fn verify_redeem_kton() {
	ExtBuilder::default()
		.build()
		.execute_with(|| {
			// https://ropsten.etherscan.io/tx/0x2878ae39a9e0db95e61164528bb1ec8684be194bdcc236848ff14d3fe5ba335d
			// darwinia: 5FP2eFNSVxJzSrE3N2NEVFPhUU34VzYFD6DDtRXbYzTdwPn8
			// hex: 0x92ae5b41feba5ee68a61449c557efa9e3b894a6461c058ec2de45429adb44546
			// amount: 0.123456789123456789 KTON
			let test_proof_relay_header_parcel : TestReceiptProofThing =  serde_json::from_str(r#"{"header":{"parent_hash":"0x734ea7bd03f510e7dd0acc85a7ceb777d5d7d5ad5650785536fc09179a250143","timestamp":1599124483,"number":8610265,"author":"0x52351e33b3c693cc05f21831647ebdab8a68eb95","transactions_root":"0xd1c259805e50c5c4aa21b92f31d8a59f025f457a90a250ca516e8930d7bb05ec","uncles_hash":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","extra_data":"0x6c6f746f706f6f6c","state_root":"0xabb0917ee898c71f3826d97415ff9161b37d03146b54e1c7d536fabb23352a5c","receipts_root":"0x9873421df4b9e11b174bc08e4d79d9a65032ba702ea8d3e0e6c6e9217d1cbf03","log_bloom":"0x0000000000800000000000000000000440000000000000100000000000000000000000000000000000001000000000000000000000000040000000000000000000000000000040001020000800000000001000000000000000000004000000000000000002000000010400000000080008000000000000000000001000000040000000000000000000000000000000c000000002004000000010000000200000000000000000000000000000800000000000000000000202080000000000000000000002000000000000000000040000000001000000000000000000000020000000000000000000000000010010000000000000000080000000000000000000","gas_used":207923,"gas_limit":8000000,"difficulty":516043711,"seal":["0xa0a7ea34046ebd043b4bca8836bc8968113d026d6c852b80a83a9b06db2db938e5","0x8800118000012d2ccd"],"hash":"0x5f80ee45d62872fa4b0dbf779a8eb380e166ac80616d05875dd3e80e5fc40839"},"receipt_proof":{"index":4,"proof":"0xf90654f90651b853f851a0924e7317d57b9cb7ebf90321fdc9f800b94b64adbaae8da31dab0142e8c079ea80808080808080a0e58215be848c1293dd381210359d84485553000a82b67410406d183b42adbbdd8080808080808080b893f89180a0d1d9123dac06536f593ff89d28ac2373b3bc603fbee756e6054d6b2162e99337a01d2879f862c4f4f818f91e74fa43188c36a344c93132783006864e506a656076a0002cd3adf59d2aaa8313a0bbaa8fd411921077bfa1edcbe04018f7339bd273faa03bddfc128660298a289de46a9301b7f03cbe5364c22aedbc28f472bdbc318778808080808080808080808080b90564f9056120b9055df9055a0183032c33b901000000000000800000000000000000000440000000000000100000000000000000000000000000000000001000000000000000000000000040000000000000000000000000000040001020000800000000001000000000000000000004000000000000000002000000010400000000080008000000000000000000001000000040000000000000000000000000000000c000000002004000000010000000200000000000000000000000000000800000000000000000000202080000000000000000000002000000000000000000040000000001000000000000000000000020000000000000000000000000010010000000000000000080000000000000000000f9044ff89b941994100c58753793d52c6f457f189aa3ce9cee94f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5a000000000000000000000000049262b932e439271d05634c32978294c7ea15d0ca00000000000000000000000000000000000000000000000000000703b4d2a5000f89b94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5a00000000000000000000000007f5b598827359939606b3525712fb124a1c7851da00000000000000000000000000000000000000000000000001bc16d674ec80000f87a941994100c58753793d52c6f457f189aa3ce9cee94f842a0cc16f5dbb4873280815c1ee09dbd06736cffcc184412cf7a71a0fdb75d397ca5a000000000000000000000000049262b932e439271d05634c32978294c7ea15d0ca00000000000000000000000000000000000000000000000000000703b4d2a5000f89b941994100c58753793d52c6f457f189aa3ce9cee94f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000049262b932e439271d05634c32978294c7ea15d0ca00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000703b4d2a5000f8fc9449262b932e439271d05634c32978294c7ea15d0cf863a0c9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10a00000000000000000000000001994100c58753793d52c6f457f189aa3ce9cee94a0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5b8800000000000000000000000000000000000000000000000000000703b4d2a500000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000020e44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318f8fc941994100c58753793d52c6f457f189aa3ce9cee94f863a09bfafdc2ae8835972d7b64ef3f8f307165ac22ceffde4a742c52da5487f45fd1a0000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5a000000000000000000000000049262b932e439271d05634c32978294c7ea15d0cb8800000000000000000000000000000000000000000000000000000703b4d2a500000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000020e44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318","header_hash":"0x5f80ee45d62872fa4b0dbf779a8eb380e166ac80616d05875dd3e80e5fc40839"},"mmr_proof":{"member_leaf_index":8610265,"last_leaf_index":8610265,"proof":["0x36f3d834cbe12a5a20b063c432b88f5506bdce03b93fa3aa035a5d82fd50177c","0x541f71c116b054b7cf0f35461b41abdc0bc6e461ae54862c8dc3af6b48742a35","0x095f1b66aed2db73567706e227981454a80a442aa69b00aedaef787fd5401a78","0x942cfd846d464575b520554dd07fd4b07ba2cb8bee6234530db1c1b3a69e1c50","0x3174f9fdf5f7c9df6ac422a0a9e2aec05ccb7968bb1ff19b6cc0b64fbc930e77","0x5624f44ff885dcdc971303281bb76c6b6a957a6cc2d244b30f736fc6220f9b89","0xa3adc2ee02f7ceda7e63df8fb58094769afd3f7657c8f9f45cdc69d5c868b468","0xe588926a1643dc04b197a7ed30dd47d42d35fdb01f7aa80b2e293b058c11c6a1","0x25d7ac48be9565512424ef559b7e0ec7a91bfe9b57440913712a174e35eb47d8","0xddce19308c3a42831e215c55aa42af46b09d98ccb2c48fc25a74aebc929f4b5d","0x734ea7bd03f510e7dd0acc85a7ceb777d5d7d5ad5650785536fc09179a250143"]}}"#).unwrap();
			let ethereum_proof_relay_header_parcel = (test_proof_relay_header_parcel.header, test_proof_relay_header_parcel.receipt_proof, test_proof_relay_header_parcel.mmr_proof);
			let relay_header_parcel : EthereumRelayHeaderParcel = serde_json::from_str(r#"{"header":{"parent_hash":"0x5f80ee45d62872fa4b0dbf779a8eb380e166ac80616d05875dd3e80e5fc40839","timestamp":1599124488,"number":8610266,"author":"0x903b52a3b21885fcc62afbed7c9ae4ed1423871e","transactions_root":"0xaa9758d9753fe39172d8c6dee5d843d8877e4136ef1a9931e909dc71800a7ee3","uncles_hash":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","extra_data":"0x4d696e656420427920416e74506f6f6c","state_root":"0x0a747c18e71216f010f330d456c4cfe262c6ccdb00b9fd1557ab8e078d87e543","receipts_root":"0x875709970cfb16e7cd2d9a1a2aabb2e900667e1ed4ed6553cc5348a09b8e869e","log_bloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000000000000040000000000000040000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","gas_used":51271,"gas_limit":7992493,"difficulty":516295685,"seal":["0xa06ee0118099a229a341b525fcf1711b2a23965d074ba9ba2d1a13a356e70d11d7","0x884ce156b802047bea"],"hash":"0x298b6e52435d4fa40fce031a73f4959e54514bafa6b861017175b17b52c57dbd"},"mmr_root":"0x26226df72727f8317e86e28503558fb02221baef2cec85816f75ae79fc250925"}"#).unwrap();

			assert_ok!(EthereumRelay::store_relay_header_parcel(relay_header_parcel));

			let expect_account_id = EthereumBacking::account_id_try_from_bytes(
				&hex_bytes_unchecked("0xe44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318"),
			).unwrap();
			// 0.123456789123456789 KTON
			assert_eq!(
				EthereumBacking::parse_token_redeem_proof(&ethereum_proof_relay_header_parcel),
				Ok((expect_account_id.clone(), (false, 123400), 0)),
			);

			let id1 = AccountId32::from([0; 32]);
			let kton_locked_before = EthereumBacking::pot::<Kton>();
			let _ = Kton::deposit_creating(&expect_account_id, 1);

			assert_ok!(EthereumBacking::redeem(
				Origin::signed(id1.clone()),
				RedeemFor::Token,
				ethereum_proof_relay_header_parcel.clone()
			));
			assert_eq!(Kton::free_balance(&expect_account_id), 123400 + 1);

			let kton_locked_after = EthereumBacking::pot::<Kton>();
			assert_eq!(kton_locked_after + 123400, kton_locked_before);

			// shouldn't redeem twice
			assert_err!(
				EthereumBacking::redeem(Origin::signed(id1.clone()), RedeemFor::Token, ethereum_proof_relay_header_parcel),
				<Error<Test>>::AssetAR,
			);
		});
}

#[test]
fn verify_redeem_deposit() {
	ExtBuilder::default()
		.build()
		.execute_with(|| {
			// 1234ring -> 0.1234kton

			// _depositID    2
			// 0: address: 0xcC5E48BEb33b83b8bD0D9d9A85A8F6a27C51F5C5  _depositor
			// 1: uint128: 1001000000000000000000 _value
			// 2: uint128: 12 _months
			// 3: uint256: 1599125470 _startAt
			// 4: uint256: 1000 _unitInterest
			// 5: bool: false
			//  _data     0x92ae5b41feba5ee68a61449c557efa9e3b894a6461c058ec2de45429adb44546

			// transfer：https://ropsten.etherscan.io/tx/0x5a7004126466ce763501c89bcbb98d14f3c328c4b310b1976a38be1183d91919
			let test_proof_relay_header_parcel : TestReceiptProofThing =  serde_json::from_str(r#"{"header":{"parent_hash":"0x870d2655fe393a3d12f595bce56ce5115907af9735cc8de8d95c05e7467d1321","timestamp":1599126312,"number":8610453,"author":"0x52351e33b3c693cc05f21831647ebdab8a68eb95","transactions_root":"0xd5a01c26eeb8e826613e4da09ac06f305fab7d183a84d417559139db814e2a23","uncles_hash":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","extra_data":"0x6c6f746f706f6f6c","state_root":"0x6d63278b3c9dae252148948ddd0db13ee1f7c81dc557681db22e422d37d9abf1","receipts_root":"0x707af65a83888ce8d9696941a122d3429c0600fb09c4644ab2162b1382099928","log_bloom":"0x00000048240080400004204082080d00000800004000001001810000028001408040000140002800000020000404010014020000100300000000000000200000240000000120808100004008400000020083826000000000800020008000100000000024021842400000042048000820080004000008400000000010001880581090880302008080004001208220000000000363084002480080200200201800220080001800004000040009002000100840008a00408208000004c00881000f00000102000500004000100000070a408100c9844020001000100000030020000010020000021520080810020000000082002020000000400000002080000000","gas_used":7545472,"gas_limit":8000000,"difficulty":535275643,"seal":["0xa01548d3ecae87cb01cb142984086dc9f772b9d9094884269b1e3644ec108e52b5","0x880011800002104414"],"hash":"0x202591d2a7bff469ec186e3583e37b9c4bce2db847612ff975180436b5a4a1f1"},"receipt_proof":{"index":46,"proof":"0xf9065ff9065cb8b3f8b1a00465eebe6e5de09530e54a14732f416c6dd3df3e3d4de7e224057775e176005fa0364264753139e9b26d4caa5550e780af82744f1fa0212fb4d944843f40941767a0d5e53b63048540c8faf6b8cb035d471603e21ed03641b2750fae5b6029968928a0a2c1de87659c963a8a8970108e0087f59b102253748e0655f60005fc5f21463780808080a0985911552c5e2f0c8f1d3b43d8c68cb6cd0e23292e7a01ed65865d9c35b5364c8080808080808080b90214f90211a09a09b69760a6f7754adb10479eed2baa872b4994161458511eda43e19ada21d1a0778f7b69876965bfc8363c672c21912ca5f92f93980999e1da19b9849c075d45a0c120f0350037e907148309c3a2e5b07bfe220e630f67c1bd7d834b40e0c6e484a0138d6d6f0e5bcf7a46b8881303caeb3bbbd3cb32d654c0294172d44d3fad3ad2a0e0665be49fab75e8c174394d55813a6fc460d64e396ef4881a55a44b89e1fae3a00227d0ab9af8d74eddbf2f8ee9bb66f7ba856ae09571f9a24272f88854d9a771a00b40ca0d746f1610bcd9d1733cdbb4a03a495cc03e6b36586e78b7f752bf8908a02c5ae2bf60a79068b949a1a5cc831f468e8092615aa61b9c8e42f21449bad981a0d6fb9b1a137c24ab6aeadfd96385eeeeebf93061285e3d2e72c5227532d2f50ba0f8206f39008acc6ce0b550424a79d05daabe982f12aaaa499ebf6165c271721ba07e1caebbd285e75cceecfa73f76dc0d5c9b3a4471d37249a1f0c8f9f9f08404aa044e197285d11b10d4d1db722bc28eb33904014c70e1f0a48dc2557be28ee836da0d2255e960373f9d0a8af01d7a2eaa2ef61d4626be9a2ea25b92cd00b7eb2d370a0ac6e37eb951f28e057c20f2be5ec4929b03f523d6fb7728701dea74be9f7c7cba06929256f0c8f74646539b0407b77c905451e1b74bc1f968a73b4c51b4889c537a0fa4f77e7a759b28b652a748dff3f136fa4ed34458fd7e212af8614235a14b27380b9038df9038a20b90386f90383018371c62eb9010000000000000000400000200000000000000000000000001000000000000000000000000140000000000000000400000000000000000000000000000000000000000000000000008000000008000000000000000000000000000000000000000000000000020000000000000000000800080000000000000000000010000000001000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000800008000000208000000000000000000000002000000000000000000040000000001000000000000000000020020000000000000000000080000000000000000000000000000000000000000000000f90278f87a94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f842a0cc16f5dbb4873280815c1ee09dbd06736cffcc184412cf7a71a0fdb75d397ca5a00000000000000000000000006ef538314829efa8386fc43386cb13b4e0a67d1ea000000000000000000000000000000000000000000000003643aa647986040000f89b94b52fbe2b925ab79a821b261c82c5ba0814aaa5e0f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000006ef538314829efa8386fc43386cb13b4e0a67d1ea00000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000003643aa647986040000f9015c946ef538314829efa8386fc43386cb13b4e0a67d1ef842a0e77bf2fa8a25e63c1e5e29e1b2fcb6586d673931e020c4e3ffede453b830fb12a0000000000000000000000000000000000000000000000000000000000000001eb90100000000000000000000000000cc5e48beb33b83b8bd0d9d9a85a8f6a27c51f5c5000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000005f50b7de00000000000000000000000000000000000000000000000000000000000003e800000000000000000000000000000000000000000000003643aa64798604000000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000020e44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318","header_hash":"0x202591d2a7bff469ec186e3583e37b9c4bce2db847612ff975180436b5a4a1f1"},"mmr_proof":{"member_leaf_index":8610453,"last_leaf_index":8610453,"proof":["0x36f3d834cbe12a5a20b063c432b88f5506bdce03b93fa3aa035a5d82fd50177c","0x541f71c116b054b7cf0f35461b41abdc0bc6e461ae54862c8dc3af6b48742a35","0x095f1b66aed2db73567706e227981454a80a442aa69b00aedaef787fd5401a78","0x942cfd846d464575b520554dd07fd4b07ba2cb8bee6234530db1c1b3a69e1c50","0x3174f9fdf5f7c9df6ac422a0a9e2aec05ccb7968bb1ff19b6cc0b64fbc930e77","0xf50c2f20115cfdc7ae6c3bf3b3bf42f386ce517310a9ae985da2ef2c9f72b54e","0x4b2db3a5e7dafa1627b3ec6816bb6aa4b1212c5291ca1eeaa4f13a0e140fb814","0x07d9c0ef21208dbc3ef29d44293b77811e1c4937cb4d3c099d7a66d3d692ab2f","0xa733e5990271f7c5a68f159c23c58f5bc5fa5187b173697230ff1637077ce9df","0x870d2655fe393a3d12f595bce56ce5115907af9735cc8de8d95c05e7467d1321"]}}"#).unwrap();
			let ethereum_proof_relay_header_parcel = (test_proof_relay_header_parcel.header, test_proof_relay_header_parcel.receipt_proof, test_proof_relay_header_parcel.mmr_proof);
			let relay_header_parcel : EthereumRelayHeaderParcel = serde_json::from_str(r#"{"header":{"parent_hash":"0x202591d2a7bff469ec186e3583e37b9c4bce2db847612ff975180436b5a4a1f1","timestamp":1599126347,"number":8610454,"author":"0xad87c0e80ab5e13f15757d5139cc6c6fcb823be3","transactions_root":"0x1f6ca6fecd61433b30a7ead60a20e685a5af66ccb14159d307b862086f96570d","uncles_hash":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","extra_data":"0xde830207028f5061726974792d457468657265756d86312e34312e30826c69","state_root":"0x4f0682474b9c3826f17fce29206f0a00f105b30685d5213cf95a81c354384a68","receipts_root":"0x0b5260288bc3e9acad7bdc9e74b7f2d89c50ff9e8002ea6a338e74473a1c3a03","log_bloom":"0x00000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000600000000000000000008000000000000000000000000000000000000000000000000020000000000000000000800000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000010000000000000000020000000000000000000000020000000000000000000000000000000000000000000","gas_used":5876802,"gas_limit":8000029,"difficulty":534752913,"seal":["0xa01a8ee4e5ebb33f10cfbe3a4a7df452aae8b8125199fc5dbf47eec3b39303f72f","0x88bd5fbaa0aaf8f15b"],"hash":"0x8ba804b2aab01defd95a69dffdcecedda6bbd422ec7ff51f0ae792cacfe7bf5c"},"mmr_root":"0x0d378bc5ada4c0103f9e5a0ff623dc72d6e797963d4902c6fcbbba4f265fab74"}"#).unwrap();

			assert_ok!(EthereumRelay::store_relay_header_parcel(relay_header_parcel));

			let ring_locked_before = EthereumBacking::pot::<Ring>();
			let expect_account_id = EthereumBacking::account_id_try_from_bytes(
				&hex_bytes_unchecked("0xe44664996ab7b5d86c12e9d5ac3093f5b2efc9172cb7ce298cd6c3c51002c318"),
			).unwrap();
			let id1 = AccountId32::from([0; 32]);
			let controller = AccountId32::from([1; 32]);
			let _ = Ring::deposit_creating(&expect_account_id, 1);

			assert_ok!(Call::from(<darwinia_staking::Call<Test>>::bond(
				controller.clone(),
				StakingBalance::RingBalance(1),
				RewardDestination::Controller,
				0,
			)).dispatch(Origin::signed(expect_account_id.clone())));
			assert_ok!(EthereumBacking::redeem(
				Origin::signed(id1.clone()),
				RedeemFor::Deposit,
				ethereum_proof_relay_header_parcel.clone()
			));
			assert_eq!(Ring::free_balance(&expect_account_id), 1001000000000 + 1);

			let ring_locked_after = EthereumBacking::pot::<Ring>();
			assert_eq!(ring_locked_after + 1001000000000, ring_locked_before);

			let staking_ledger = Staking::ledger(&controller);
			assert_eq!(staking_ledger, Some(StakingLedger {
				stash: expect_account_id,
				active_ring: 1001000000001,
				active_deposit_ring: 1001000000000,
				deposit_items: vec![TimeDepositItem {
					value: 1001000000000,
					start_time: 1599125470000,
					expire_time: 1630229470000,
				}],
				ring_staking_lock: StakingLock { staking_amount: 1001000000001, unbondings: vec![] },
				..Default::default()
			}));

			// shouldn't redeem twice
			assert_err!(
				EthereumBacking::redeem(Origin::signed(id1.clone()), RedeemFor::Deposit, ethereum_proof_relay_header_parcel),
				<Error<Test>>::AssetAR,
			);
		});
}
