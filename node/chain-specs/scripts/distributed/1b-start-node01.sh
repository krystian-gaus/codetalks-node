#!/bin/bash

# exit when any command fails
set -e

BASE_DIR=$(dirname $(readlink -f $0))
CODETALKS_DIR=${BASE_DIR}/../../../../
NODE=node01

cd ${CODETALKS_DIR}
./target/release/node-template \
  --base-path /tmp/${NODE} \
  --chain ./node/chain-specs/codetalks-testnet.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --validator \
  --rpc-methods Unsafe \
  --name curie \
  --node-key=468ace82a1e7f78f753ef996d0370df3685fdbb73c1d3481c804acf5f69636ce \
  --password "curie-node-p@ssw0rd"
