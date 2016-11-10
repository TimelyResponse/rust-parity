#!/bin/bash

cargo test\
	-p bitcrypto\
	-p chain\
	-p db\
	-p ethcore-devtools\
	-p import\
	-p keys\
	-p message\
	-p miner\
	-p pbtc\
	-p p2p\
	-p primitives\
	-p script\
	-p serialization\
	-p sync\
	-p verification
