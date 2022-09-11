#!/bin/bash

# exit when any command fails
set -e

BASE_DIR=$(dirname $(readlink -f $0))
CODETALKS_DIR=${BASE_DIR}/../../../../

cd ${CODETALKS_DIR}
./target/release/node-template \
  --base-path /tmp/node03 \
  --chain ./node/chain-specs/codetalks-local-testnet.json \
  --port 30335 \
  --ws-port 9947 \
  --rpc-port 9935 \
  --validator \
  --rpc-methods Unsafe \
  --name noether \
  --node-key=066382d32364b03e710667dc9d84e33e9d8e2c187d544a8b704dce3a1c99d5cc \
  --password "noether-node-p@ssw0rd"
