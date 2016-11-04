use std::net::SocketAddr;
use sync::create_sync_connection_factory;
use util::{open_db, init_db};
use {config, p2p};

pub fn start(cfg: config::Config) -> Result<(), String> {
	let mut el = p2p::event_loop();

	let p2p_cfg = p2p::Config {
		threads: 4,
		protocol_minimum: 70001,
		protocol_maximum: 70017,
		inbound_connections: 10,
		outbound_connections: 10,
		connection: p2p::NetConfig {
			magic: cfg.magic,
			local_address: SocketAddr::new("127.0.0.1".parse().unwrap(), cfg.port),
			services: Default::default(),
			user_agent: "pbtc".into(),
			start_height: 0,
			relay: false,
		},
		peers: cfg.connect.map_or_else(|| vec![], |x| vec![x]),
		seeds: cfg.seednode.map_or_else(|| vec![], |x| vec![x]),
	};

	let db = open_db(cfg.use_disk_database);
	init_db(&db);

	let sync_handle = el.handle();
	let sync_connection_factory = create_sync_connection_factory(&sync_handle, db);

	let p2p = p2p::P2P::new(p2p_cfg, sync_connection_factory, el.handle());
	try!(p2p.run().map_err(|_| "Failed to start p2p module"));
	el.run(p2p::forever()).unwrap();
	Ok(())
}
