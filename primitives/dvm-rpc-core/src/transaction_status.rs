use codec::{Decode, Encode};
use ethereum::Log;
use ethereum_types::Bloom;
use sp_core::{H160, H256};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

#[derive(Eq, PartialEq, Clone, Encode, Decode, RuntimeDebug)]
pub struct TransactionStatus {
	pub transaction_hash: H256,
	pub transaction_index: u32,
	pub from: H160,
	pub to: Option<H160>,
	pub contract_address: Option<H160>,
	pub logs: Vec<Log>,
	pub logs_bloom: Bloom,
}

impl Default for TransactionStatus {
	fn default() -> Self {
		TransactionStatus {
			transaction_hash: H256::default(),
			transaction_index: 0 as u32,
			from: H160::default(),
			to: None,
			contract_address: None,
			logs: Vec::new(),
			logs_bloom: Bloom::default(),
		}
	}
}