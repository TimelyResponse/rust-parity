//! Various chain-specific test dummies

extern crate chain;
extern crate primitives;
extern crate serialization as ser;

use chain::Block;

pub mod chain_builder;

pub use chain_builder::{ChainBuilder, TransactionBuilder};

pub fn block1() -> Block {
	let block: Block = "01000000ba8b9cda965dd8e536670f9ddec10e53aab14b20bacad27b9137190000000000190760b278fe7b8565fda3b968b918d5fd997f993b23674c0af3b6fde300b38f33a5914ce6ed5b1b01e32f570201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704e6ed5b1b014effffffff0100f2052a01000000434104b68a50eaa0287eff855189f949c1c6e5f58b37c88231373d8a59809cbae83059cc6469d65c665ccfd1cfeb75c6e8e19413bba7fbff9bc762419a76d87b16086eac000000000100000001a6b97044d03da79c005b20ea9c0e1a6d9dc12d9f7b91a5911c9030a439eed8f5000000004948304502206e21798a42fae0e854281abd38bacd1aeed3ee3738d9e1446618c4571d1090db022100e2ac980643b0b82c0e88ffdfec6b64e3e6ba35e7ba5fdd7d5d6cc8d25c6b241501ffffffff0100f2052a010000001976a914404371705fa9bd789a2fcd52d2c580b65d35549d88ac00000000".into();
	block
}

// https://webbtc.com/block/00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048
// height 1
pub fn block_h1() -> Block {
	let block: Block = "010000006fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000982051fd1e4ba744bbbe680e1fee14677ba1a3c3540bf7b1cdb606e857233e0e61bc6649ffff001d01e362990101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0104ffffffff0100f2052a0100000043410496b538e853519c726a2c91e61ec11600ae1390813a627c66fb8be7947be63c52da7589379515d4e0a604f8141781e62294721166bf621e73a82cbf2342c858eeac00000000".into();
	block
}

// https://webbtc.com/block/000000006a625f06636b8bb6ac7b960a8d03705d1ace08b1a19da3fdcc99ddbd.hex
// height 2
pub fn block_h2() -> Block {
	let block: Block = "010000004860eb18bf1b1620e37e9490fc8a427514416fd75159ab86688e9a8300000000d5fdcc541e25de1c7a5addedf24858b8bb665c9f36ef744ee42c316022c90f9bb0bc6649ffff001d08d2bd610101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d010bffffffff0100f2052a010000004341047211a824f55b505228e4c3d5194c1fcfaa15a456abdf37f9b9d97a4040afc073dee6c89064984f03385237d92167c13e236446b417ab79a0fcae412ae3316b77ac00000000".into();
	block
}

pub fn genesis() -> Block {
	let block: Block = "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c0101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000".into();
	block
}

// https://webbtc.com/block/00000000d1145790a8694403d4063f323d499e655c83426834d4ce2f8dd4a2ee
// block with the first transaction
// also is the source for 181
pub fn block_h170() -> Block {
	let block: Block = "0100000055bd840a78798ad0da853f68974f3d183e2bd1db6a842c1feecf222a00000000ff104ccb05421ab93e63f8c3ce5c2c2e9dbb37de2764b3a3175c8166562cac7d51b96a49ffff001d283e9e700201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0102ffffffff0100f2052a01000000434104d46c4968bde02899d2aa0963367c7a6ce34eec332b32e42e5f3407e052d64ac625da6f0718e7b302140434bd725706957c092db53805b821a85b23a7ac61725bac000000000100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000".into();
	block
}


// https://webbtc.com/block/000000002a22cfee1f2c846adbd12b3e183d4f97683f85dad08a79780a84bd55
// block 169
pub fn block_h169() -> Block {
	let block: Block = "01000000696aa63f0f22d9189c8536bb83b18737ae8336c25a67937f79957e5600000000982db9870a5e30d8f0b2a4ebccc5852b5a1e2413e9274c4947bfec6bdaa9b9d75bb76a49ffff001d2b719fdd0101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0101ffffffff0100f2052a010000004341045da87c7b825c75ca17ade8bb5cdbcd27af4ce97373aa9848c0c84693ca857cf379e14c2ce61ea2aaee9450d0939e21bd26894aa6dcc808656fa9974dc296589eac00000000".into();
	block
}

// https://webbtc.com/block/000000008d9dc510f23c2657fc4f67bea30078cc05a90eb89e84cc475c080805
// block with the source funds for the first transaction
pub fn block_h9() -> Block {
	let block: Block = "01000000c60ddef1b7618ca2348a46e868afc26e3efc68226c78aa47f8488c4000000000c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd37047fca6649ffff001d28404f530101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0134ffffffff0100f2052a0100000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000".into();
	block
}

// https://webbtc.com/block/0000000066356691a4353dd8bdc2c60da20d68ad34fff93d8839a133b2a6d42a
// block with some interesting test case
pub fn block_h221() -> Block {
	let block: Block = "01000000581d2b080bc47372c06cc5de8eb40386b00c72d4bdfecdd239c56ab600000000079f89b6e0f19f8c29d6c648ff390c9af2cf7c1da8eab6ae168cd208c745f467cc516b49ffff001d0171a0690201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d029e00ffffffff0100f2052a01000000434104b089ed9912db1edb2e7338154e2ba9c4eaac2ea1b9f95600005aac1fde07315ef3c16125cc387373232ac6fa6453f2046e317e2295321a58f244d6d2692d597eac00000000010000000173805864da01f15093f7837607ab8be7c3705e29a9d4a12c9116d709f8911e59000000004a493046022100f493d504a670e6280bc76ee285accf2796fd6a630659d4fa55dccb793fc9346402210080ecfbe069101993eb01320bb1d6029c138b27835e20ad54c232c36ffe10786301ffffffff0100e1f50500000000434104ea0d6650c8305f1213a89c65fc8f4343a5dac8e985c869e51d3aa02879b57c60cff49fcb99314d02dfc612d654e4333150ef61fa569c1c66415602cae387baf7ac00000000".into();
	block
}

// https://webbtc.com/block/0000000054487811fc4ff7a95be738aa5ad9320c394c482b27c0da28b227ad5d
// block for with transaction source for 221
pub fn block_h182() -> Block {
	let block: Block = "01000000e5c6af65c46bd826723a83c1c29d9efa189320458dc5298a0c8655dc0000000030c2a0d34bfb4a10d35e8166e0f5a37bce02fc1b85ff983739a191197f010f2f40df6a49ffff001d2ce7ac9e0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0129ffffffff0100f2052a01000000434104b10dd882c04204481116bd4b41510e98c05a869af51376807341fc7e3892c9034835954782295784bfc763d9736ed4122c8bb13d6e02c0882cb7502ce1ae8287ac000000000100000001be141eb442fbc446218b708f40caeb7507affe8acff58ed992eb5ddde43c6fa1010000004847304402201f27e51caeb9a0988a1e50799ff0af94a3902403c3ad4068b063e7b4d1b0a76702206713f69bd344058b0dee55a9798759092d0916dbbc3e592fee43060005ddc17401ffffffff0200e1f5050000000043410401518fa1d1e1e3e162852d68d9be1c0abad5e3d6297ec95f1f91b909dc1afe616d6876f92918451ca387c4387609ae1a895007096195a824baf9c38ea98c09c3ac007ddaac0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000".into();
	block
}

// https://webbtc.com/block/0000000054487811fc4ff7a95be738aa5ad9320c394c482b27c0da28b227ad5d
// block for with transaction source for 182
pub fn block_h181() -> Block {
	let block: Block = "01000000f2c8a8d2af43a9cd05142654e56f41d159ce0274d9cabe15a20eefb500000000366c2a0915f05db4b450c050ce7165acd55f823fee51430a8c993e0bdbb192ede5dc6a49ffff001d192d3f2f0201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0128ffffffff0100f2052a0100000043410435f0d8366085f73906a48309728155532f24293ea59fe0b33a245c4b8d75f82c3e70804457b7f49322aa822196a7521e4931f809d7e489bccb4ff14758d170e5ac000000000100000001169e1e83e930853391bc6f35f605c6754cfead57cf8387639d3b4096c54f18f40100000048473044022027542a94d6646c51240f23a76d33088d3dd8815b25e9ea18cac67d1171a3212e02203baf203c6e7b80ebd3e588628466ea28be572fe1aaa3f30947da4763dd3b3d2b01ffffffff0200ca9a3b00000000434104b5abd412d4341b45056d3e376cd446eca43fa871b51961330deebd84423e740daa520690e1d9e074654c59ff87b408db903649623e86f1ca5412786f61ade2bfac005ed0b20000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000".into();
	block
}
