#![allow(dead_code)] // TODO: remove after connecting with Client

use bit_vec::BitVec;
use murmur3::murmur3_32;
use chain::{Transaction, OutPoint};
use ser::serialize;
use message::types;
use script::Script;

/// Constant optimized to create large differences in the seed for different values of `hash_functions_num`.
const SEED_OFFSET: u32 = 0xFBA4C795;

/// Filter, which controls data relayed over connection.
#[derive(Debug)]
pub struct ConnectionFilter {
	/// Bloom filter, if set.
	bloom: Option<ConnectionBloom>,
	/// Filter update type.
	filter_flags: types::FilterFlags,
}

/// Connection bloom filter
#[derive(Debug)]
struct ConnectionBloom {
	/// Filter storage.
	filter: BitVec,
	/// Number of hash functions to use in bloom filter.
	hash_functions_num: u32,
	/// Value to add to Murmur3 hash seed when calculating hash.
	tweak: u32,
}

impl Default for ConnectionFilter {
	fn default() -> Self {
		ConnectionFilter {
			bloom: None,
			filter_flags: types::FilterFlags::None,
		}
	}
}

impl ConnectionFilter {
	#[cfg(test)]
	/// Create new connection with given filter params
	pub fn with_filterload(message: &types::FilterLoad) -> Self {
		ConnectionFilter {
			bloom: Some(ConnectionBloom::new(message)),
			filter_flags: message.flags,
		}
	}

	/// Check if transaction is matched && update filter
	pub fn match_update_transaction(&mut self, transaction: &Transaction) -> bool {
		match self.bloom {
			/// if no filter is set for the connection => match everything
			None => true,
			/// filter using bloom filter, then update
			Some(ref mut bloom) => {
				let transaction_hash = transaction.hash();
				let mut is_match = false;

				// match if filter contains any arbitrary script data element in any scriptPubKey in tx
				for (output_index, output) in transaction.outputs.iter().enumerate() {
					let script = Script::new(output.script_pubkey.clone());
					for instruction in script.iter().filter_map(|i| i.ok()) {
						if let Some(instruction_data) = instruction.data {
							if bloom.contains(instruction_data) {
								is_match = true;

								let is_update_needed = self.filter_flags == types::FilterFlags::All
									|| (self.filter_flags == types::FilterFlags::PubKeyOnly && (script.is_pay_to_public_key() || script.is_multisig_script()));
								if is_update_needed {
									bloom.insert(&serialize(&OutPoint {
										hash: transaction_hash.clone(),
										index: output_index as u32,
									}));
								}
							}
						}
					}
				}
				if is_match {
					return is_match;
				}

				// match if filter contains transaction itself
				if bloom.contains(&*transaction_hash) {
					return true;
				}

				// match if filter contains an outpoint this transaction spends
				for input in &transaction.inputs {
					// check if match previous output
					let previous_output = serialize(&input.previous_output);
					is_match = bloom.contains(&*previous_output);
					if is_match {
						return true;
					}

					// check if match any arbitrary script data element in any scriptSig in tx
					let script = Script::new(input.script_sig.clone());
					for instruction in script.iter().filter_map(|i| i.ok()) {
						if let Some(instruction_data) = instruction.data {
							is_match = bloom.contains(&*instruction_data);
							if is_match {
								return true;
							}
						}
					}
				}

				// no matches
				false
			},
		}
	}

	/// Load filter
	pub fn load(&mut self, message: &types::FilterLoad) {
		self.bloom = Some(ConnectionBloom::new(message));
		self.filter_flags = message.flags;
	}

	/// Add filter
	pub fn add(&mut self, message: &types::FilterAdd) {
		// ignore if filter is not currently set
		if let Some(ref mut bloom) = self.bloom {
			bloom.insert(&message.data);
		}
	}

	/// Clear filter
	pub fn clear(&mut self) {
		self.bloom = None;
	}
}

impl ConnectionBloom {
	/// Create with given parameters
	pub fn new(message: &types::FilterLoad) -> Self {
		ConnectionBloom {
			filter: BitVec::from_bytes(&message.filter),
			hash_functions_num: message.hash_functions,
			tweak: message.tweak,
		}
	}

	/// True if filter contains given bytes
	pub fn contains(&self, data: &[u8]) -> bool {
		for hash_function_idx in 0..self.hash_functions_num {
			let murmur_seed = hash_function_idx.overflowing_mul(SEED_OFFSET).0.overflowing_add(self.tweak).0;
			let murmur_hash = murmur3_32(&mut data.as_ref(), murmur_seed) as usize % self.filter.len();
			if !self.filter.get(murmur_hash).expect("mod operation above") {
				return false;
			}
		}
		true
	}

	/// Add bytes to the filter
	pub fn insert(&mut self, data: &[u8]) {
		for hash_function_idx in 0..self.hash_functions_num {
			let murmur_seed = hash_function_idx.overflowing_mul(SEED_OFFSET).0.overflowing_add(self.tweak).0;
			let murmur_hash = murmur3_32(&mut data.as_ref(), murmur_seed) as usize % self.filter.len();
			self.filter.set(murmur_hash, true);
		}
	}
}

#[cfg(test)]
mod tests {
	use std::iter::{Iterator, repeat};
	use test_data;
	use message::types;
	use chain::Transaction;
	use primitives::hash::H256;
	use primitives::bytes::Bytes;
	use ser::serialize;
	use super::{ConnectionFilter, ConnectionBloom};

	fn default_filterload() -> types::FilterLoad {
		types::FilterLoad {
			filter: Bytes::from(repeat(0u8).take(1024).collect::<Vec<_>>()),
			hash_functions: 10,
			tweak: 5,
			flags: types::FilterFlags::None,
		}
	}

	fn make_filteradd(data: &[u8]) -> types::FilterAdd {
		types::FilterAdd {
			data: data.into(),
		}
	}

	#[test]
	fn bloom_insert_data() {
		let mut bloom = ConnectionBloom::new(&default_filterload());

		assert!(!bloom.contains(&*H256::default()));

		bloom.insert(&*H256::default());
		assert!(bloom.contains(&*H256::default()));
	}

	#[test]
	fn connection_filter_matches_transaction_by_hash() {
		let tx1: Transaction = test_data::TransactionBuilder::with_output(10).into();
		let tx2: Transaction = test_data::TransactionBuilder::with_output(20).into();

		let mut filter = ConnectionFilter::with_filterload(&default_filterload());

		assert!(!filter.match_update_transaction(&tx1));
		assert!(!filter.match_update_transaction(&tx2));

		filter.add(&make_filteradd(&*tx1.hash()));

		assert!(filter.match_update_transaction(&tx1));
		assert!(!filter.match_update_transaction(&tx2));
	}

	#[test]
	fn connection_filter_matches_transaction_by_output_script_data_element() {
		// https://webbtc.com/tx/eb3b82c0884e3efa6d8b0be55b4915eb20be124c9766245bcc7f34fdac32bccb
		// output script: OP_DUP OP_HASH160 380cb3c594de4e7e9b8e18db182987bebb5a4f70 OP_EQUALVERIFY OP_CHECKSIG
		let tx1: Transaction = "01000000024de8b0c4c2582db95fa6b3567a989b664484c7ad6672c85a3da413773e63fdb8000000006b48304502205b282fbc9b064f3bc823a23edcc0048cbb174754e7aa742e3c9f483ebe02911c022100e4b0b3a117d36cab5a67404dddbf43db7bea3c1530e0fe128ebc15621bd69a3b0121035aa98d5f77cd9a2d88710e6fc66212aff820026f0dad8f32d1f7ce87457dde50ffffffff4de8b0c4c2582db95fa6b3567a989b664484c7ad6672c85a3da413773e63fdb8010000006f004730440220276d6dad3defa37b5f81add3992d510d2f44a317fd85e04f93a1e2daea64660202200f862a0da684249322ceb8ed842fb8c859c0cb94c81e1c5308b4868157a428ee01ab51210232abdc893e7f0631364d7fd01cb33d24da45329a00357b3a7886211ab414d55a51aeffffffff02e0fd1c00000000001976a914380cb3c594de4e7e9b8e18db182987bebb5a4f7088acc0c62d000000000017142a9bc5447d664c1d0141392a842d23dba45c4f13b17500000000".into();
		let tx1_out_data: Bytes = "380cb3c594de4e7e9b8e18db182987bebb5a4f70".into();
		let tx2 = Transaction::default();

		let mut filter = ConnectionFilter::with_filterload(&default_filterload());

		assert!(!filter.match_update_transaction(&tx1));
		assert!(!filter.match_update_transaction(&tx2));

		filter.add(&make_filteradd(&tx1_out_data));

		assert!(filter.match_update_transaction(&tx1));
		assert!(!filter.match_update_transaction(&tx2));
	}

	#[test]
	fn connection_filter_matches_transaction_by_previous_output_point() {
		// https://webbtc.com/tx/eb3b82c0884e3efa6d8b0be55b4915eb20be124c9766245bcc7f34fdac32bccb
		// output script: OP_DUP OP_HASH160 380cb3c594de4e7e9b8e18db182987bebb5a4f70 OP_EQUALVERIFY OP_CHECKSIG
		let tx1: Transaction = "01000000024de8b0c4c2582db95fa6b3567a989b664484c7ad6672c85a3da413773e63fdb8000000006b48304502205b282fbc9b064f3bc823a23edcc0048cbb174754e7aa742e3c9f483ebe02911c022100e4b0b3a117d36cab5a67404dddbf43db7bea3c1530e0fe128ebc15621bd69a3b0121035aa98d5f77cd9a2d88710e6fc66212aff820026f0dad8f32d1f7ce87457dde50ffffffff4de8b0c4c2582db95fa6b3567a989b664484c7ad6672c85a3da413773e63fdb8010000006f004730440220276d6dad3defa37b5f81add3992d510d2f44a317fd85e04f93a1e2daea64660202200f862a0da684249322ceb8ed842fb8c859c0cb94c81e1c5308b4868157a428ee01ab51210232abdc893e7f0631364d7fd01cb33d24da45329a00357b3a7886211ab414d55a51aeffffffff02e0fd1c00000000001976a914380cb3c594de4e7e9b8e18db182987bebb5a4f7088acc0c62d000000000017142a9bc5447d664c1d0141392a842d23dba45c4f13b17500000000".into();
		let tx1_previous_output: Bytes = serialize(&tx1.inputs[0].previous_output);
		let tx2 = Transaction::default();

		let mut filter = ConnectionFilter::with_filterload(&default_filterload());

		assert!(!filter.match_update_transaction(&tx1));
		assert!(!filter.match_update_transaction(&tx2));

		filter.add(&make_filteradd(&tx1_previous_output));

		assert!(filter.match_update_transaction(&tx1));
		assert!(!filter.match_update_transaction(&tx2));
	}

	#[test]
	fn connection_filter_matches_transaction_by_input_script_data_element() {
		// https://webbtc.com/tx/eb3b82c0884e3efa6d8b0be55b4915eb20be124c9766245bcc7f34fdac32bccb
		// output script: OP_DUP OP_HASH160 380cb3c594de4e7e9b8e18db182987bebb5a4f70 OP_EQUALVERIFY OP_CHECKSIG
		let tx1: Transaction = "01000000024de8b0c4c2582db95fa6b3567a989b664484c7ad6672c85a3da413773e63fdb8000000006b48304502205b282fbc9b064f3bc823a23edcc0048cbb174754e7aa742e3c9f483ebe02911c022100e4b0b3a117d36cab5a67404dddbf43db7bea3c1530e0fe128ebc15621bd69a3b0121035aa98d5f77cd9a2d88710e6fc66212aff820026f0dad8f32d1f7ce87457dde50ffffffff4de8b0c4c2582db95fa6b3567a989b664484c7ad6672c85a3da413773e63fdb8010000006f004730440220276d6dad3defa37b5f81add3992d510d2f44a317fd85e04f93a1e2daea64660202200f862a0da684249322ceb8ed842fb8c859c0cb94c81e1c5308b4868157a428ee01ab51210232abdc893e7f0631364d7fd01cb33d24da45329a00357b3a7886211ab414d55a51aeffffffff02e0fd1c00000000001976a914380cb3c594de4e7e9b8e18db182987bebb5a4f7088acc0c62d000000000017142a9bc5447d664c1d0141392a842d23dba45c4f13b17500000000".into();
		let tx1_input_data: Bytes = "304502205b282fbc9b064f3bc823a23edcc0048cbb174754e7aa742e3c9f483ebe02911c022100e4b0b3a117d36cab5a67404dddbf43db7bea3c1530e0fe128ebc15621bd69a3b01".into();
		let tx2 = Transaction::default();

		let mut filter = ConnectionFilter::with_filterload(&default_filterload());

		assert!(!filter.match_update_transaction(&tx1));
		assert!(!filter.match_update_transaction(&tx2));

		filter.add(&make_filteradd(&tx1_input_data));

		assert!(filter.match_update_transaction(&tx1));
		assert!(!filter.match_update_transaction(&tx2));
	}
}
