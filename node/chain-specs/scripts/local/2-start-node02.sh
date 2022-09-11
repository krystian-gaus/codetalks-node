#!/bin/bash

# exit when any command fails
set -e

BASE_DIR=$(dirname $(readlink -f $0))
CODETALKS_DIR=${BASE_DIR}/../../../../

cd ${CODETALKS_DIR}
./target/release/node-template \
  --base-path /tmp/node02 \
  --chain ./node/chain-specs/codetalks-local-testnet.json \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --validator \
  --rpc-methods Unsafe \
  --name hamilton \
  --node-key=1389bca6b073bf19bf975684144aff239381b55c6430c4dcf5b33d37f07e5bf2 \
  --password "hamilton-node-p@ssw0rd"
