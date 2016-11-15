use std::sync::Arc;
use parking_lot::Mutex;
use bytes::Bytes;
use message::{Command, Error};
use p2p::Context;
use net::PeerContext;
use protocol::{Protocol, PingProtocol, SyncProtocol, AddrProtocol, SeednodeProtocol};
use util::{ConfigurableSynchronizer, PeerInfo};

pub trait SessionFactory {
	fn new_session(context: Arc<Context>, info: PeerInfo, synchronous: bool) -> Session;
}

pub struct SeednodeSessionFactory;

impl SessionFactory for SeednodeSessionFactory {
	fn new_session(context: Arc<Context>, info: PeerInfo, synchronous: bool) -> Session {
		let peer_context = Arc::new(PeerContext::new(context, info, synchronous));
		let ping = PingProtocol::new(peer_context.clone()).boxed();
		let addr = AddrProtocol::new(peer_context.clone()).boxed();
		let seed = SeednodeProtocol::new(peer_context.clone()).boxed();
		Session::new(peer_context, vec![ping, addr, seed])
	}
}

pub struct NormalSessionFactory;

impl SessionFactory for NormalSessionFactory {
	fn new_session(context: Arc<Context>, info: PeerInfo, synchronous: bool) -> Session {
		let peer_context = Arc::new(PeerContext::new(context, info, synchronous));
		let ping = PingProtocol::new(peer_context.clone()).boxed();
		let addr = AddrProtocol::new(peer_context.clone()).boxed();
		let sync = SyncProtocol::new(peer_context.clone()).boxed();
		Session::new(peer_context, vec![ping, addr, sync])
	}
}

pub struct Session {
	peer_context: Arc<PeerContext>,
	protocols: Mutex<Vec<Box<Protocol>>>,
	synchronizer: Mutex<ConfigurableSynchronizer>,
}

impl Session {
	pub fn new(peer_context: Arc<PeerContext>, protocols: Vec<Box<Protocol>>) -> Self {
		Session {
			peer_context: peer_context,
			protocols: Mutex::new(protocols),
			synchronizer: Mutex::new(ConfigurableSynchronizer::new(false)),
		}
	}

	pub fn initialize(&self) {
		for protocol in self.protocols.lock().iter_mut() {
			protocol.initialize();
		}
	}

	pub fn on_message(&self, command: Command, payload: Bytes) -> Result<(), Error> {
		self.protocols.lock()
			.iter_mut()
			.map(|protocol| {
				protocol.on_message(&command, &payload)
			})
			.collect::<Result<Vec<_>, Error>>()
			.map(|_| ())
	}

	pub fn on_close(&self) {
		for protocol in self.protocols.lock().iter_mut() {
			protocol.on_close();
		}
	}
}

