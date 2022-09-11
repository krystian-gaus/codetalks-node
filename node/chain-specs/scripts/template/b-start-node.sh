#!/bin/bash

# exit when any command fails
set -e

BASE_DIR=$(dirname $(readlink -f $0))
CODETALKS_DIR=${BASE_DIR}/../../../../
NODE=<NODE-ID>

cd ${CODETALKS_DIR}
./target/release/node-template \
  --base-path /tmp/${NODE} \
  --chain ./node/chain-specs/codetalks-testnet.json \
  --port <PORT> \
  --ws-port <WS-PORT> \
  --rpc-port <RPC-PORT> \
  --validator \
  --rpc-methods Unsafe \
  --name <NODE-NAME> \
  --node-key=<NODE-KEY> \
  --password <NODE-PASSWORD>
