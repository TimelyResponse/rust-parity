use rayon::prelude::{IntoParallelRefIterator, IndexedParallelIterator, ParallelIterator};
use db::Store;
use network::ConsensusParams;
use error::Error;
use canon::CanonBlock;
use accept_block::BlockAcceptor;
use accept_header::HeaderAcceptor;
use accept_transaction::TransactionAcceptor;
use deployments::{Deployments, ActiveDeployments};
use duplex_store::DuplexTransactionOutputProvider;
use VerificationLevel;

pub struct ChainAcceptor<'a> {
	pub block: BlockAcceptor<'a>,
	pub header: HeaderAcceptor<'a>,
	pub transactions: Vec<TransactionAcceptor<'a>>,
}

impl<'a> ChainAcceptor<'a> {
	pub fn new(store: &'a Store, consensus: &'a ConsensusParams, verification_level: VerificationLevel, block: CanonBlock<'a>, height: u32, deployments: &'a Deployments) -> Self {
		trace!(target: "verification", "Block verification {}", block.hash().to_reversed_str());
		let output_store = DuplexTransactionOutputProvider::new(store.as_transaction_output_provider(), block.raw());
		let headers = store.as_block_header_provider();
		let active_deployments = ActiveDeployments::new(deployments, height, headers, consensus);

		ChainAcceptor {
			block: BlockAcceptor::new(store.as_transaction_output_provider(), consensus, block, height, active_deployments, headers),
			header: HeaderAcceptor::new(headers, consensus, block.header(), height, deployments),
			transactions: block.transactions()
				.into_iter()
				.enumerate()
				.map(|(tx_index, tx)| TransactionAcceptor::new(
						store.as_transaction_meta_provider(),
						output_store,
						consensus,
						tx,
						verification_level,
						block.hash(),
						height,
						block.header.raw.time,
						tx_index,
						active_deployments,
						headers,
				))
				.collect(),
		}
	}

	pub fn check(&self) -> Result<(), Error> {
		try!(self.block.check());
		try!(self.header.check());
		try!(self.check_transactions());
		Ok(())
	}

	fn check_transactions(&self) -> Result<(), Error> {
		self.transactions.par_iter()
			.enumerate()
			.fold(|| Ok(()), |result, (index, tx)| result.and_then(|_| tx.check().map_err(|err| Error::Transaction(index, err))))
			.reduce(|| Ok(()), |acc, check| acc.and(check))
	}
}
