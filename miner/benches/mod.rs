#![feature(test)]

extern crate test;
extern crate miner;
extern crate chain;
extern crate primitives;
extern crate serialization as ser;

#[cfg(test)]
mod benchmarks {
	use std::collections::VecDeque;
	use super::chain::{Transaction, TransactionInput, OutPoint};
	use super::primitives::bytes::Bytes;
	use super::test::Bencher;
	use super::miner::{MemoryPool, MemoryPoolOrderingStrategy};

	fn prepare_independent_transactions(n: usize) -> VecDeque<Transaction> {
		(0..n).map(|nonce| Transaction {
			version: nonce as i32,
			inputs: vec![],
			outputs: vec![],
			lock_time: 0,
		}).collect()
	}

	fn prepare_dependent_transactions(n: usize) -> VecDeque<Transaction> {
		let previous_transaction: Transaction = "00000000000164000000000000000000000000".into();
		let mut previous_transaction_hash = previous_transaction.hash();
		let mut result = VecDeque::new();
		result.push_back(previous_transaction);
		result.extend((0..n).map(|_nonce| {
			let transaction = Transaction {
				version: 0,
				inputs: vec![
					TransactionInput {
						previous_output: OutPoint {
							hash: previous_transaction_hash.clone(),
							index: 0,
						},
						script_sig: Bytes::new_with_len(0),
						sequence: 0,
					},
				],
				outputs: vec![],
				lock_time: 0,
			};
			previous_transaction_hash = transaction.hash();
			transaction
		}));
		result
	}

	#[bench]
	// test benchmarks::bench_memory_pool_insert_independent_transaction ... bench:       1,520 ns/iter (+/- 341)
	fn bench_memory_pool_insert_independent_transaction(b: &mut Bencher) {
		let iterations = 100;
		let mut pool = MemoryPool::new();
		let mut transactions = prepare_independent_transactions(iterations);
		b.bench_n(iterations as u64, |b| b.iter(|| {
			pool.insert_verified(transactions.pop_front().unwrap())
		}));
	}

	#[bench]
	// test benchmarks::bench_memory_pool_insert_descendant_transaction  ... bench:       8,119 ns/iter (+/- 2,000)
	fn bench_memory_pool_insert_descendant_transaction(b: &mut Bencher) {
		let iterations = 100usize;
		let mut pool = MemoryPool::new();
		let mut transactions = prepare_dependent_transactions(iterations);
		pool.insert_verified(transactions.pop_front().unwrap());

		b.bench_n(iterations as u64, |b| b.iter(|| {
			pool.insert_verified(transactions.pop_front().unwrap())
		}));
	}

	#[bench]
	// test benchmarks::bench_memory_pool_insert_ancestor_transaction    ... bench:     398,755 ns/iter (+/- 81,533)
	// very slow due to weird usage scenario:
	// (1) transactions inserted to memorypool are verified
	// (2) verified => ancestors already verified
	// (3) ancestors verified => they are already in memorypool (not this case) or in database (why inserting to memorypool then)
	fn bench_memory_pool_insert_ancestor_transaction(b: &mut Bencher) {
		let iterations = 100usize;
		let mut pool = MemoryPool::new();
		let mut transactions = prepare_dependent_transactions(iterations);
		pool.insert_verified(transactions.pop_front().unwrap());

		b.bench_n(iterations as u64, |b| b.iter(|| {
			pool.insert_verified(transactions.pop_back().unwrap())
		}));
	}

	#[bench]
	// test benchmarks::bench_memory_pool_remove_dependent_in_order      ... bench:         757 ns/iter (+/- 211)
	fn bench_memory_pool_remove_independent_in_order(b: &mut Bencher) {
		let iterations = 100;
		let mut pool = MemoryPool::new();
		for transaction in prepare_independent_transactions(iterations) {
			pool.insert_verified(transaction)
		}
		b.bench_n(iterations as u64, |b| b.iter(|| {
			pool.remove_with_strategy(MemoryPoolOrderingStrategy::ByTimestamp)
		}));
	}

	#[bench]
	// test benchmarks::bench_memory_pool_remove_independent_in_order    ... bench:         455 ns/iter (+/- 139)
	fn bench_memory_pool_remove_dependent_in_order(b: &mut Bencher) {
		let iterations = 100;
		let mut pool = MemoryPool::new();
		for transaction in prepare_dependent_transactions(iterations) {
			pool.insert_verified(transaction)
		}
		b.bench_n(iterations as u64, |b| b.iter(|| {
			pool.remove_with_strategy(MemoryPoolOrderingStrategy::ByTimestamp)
		}));
	}
}
