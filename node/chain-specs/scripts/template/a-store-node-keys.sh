#!/bin/bash

# exit when any command fails
set -e

BASE_DIR=$(dirname $(readlink -f $0))
CODETALKS_DIR=${BASE_DIR}/../../../../
NODE=<NODE-ID>

echo "base dir: ${BASE_DIR}"
echo "codetalks dir: ${CODETALKS_DIR}"
echo "node ID: ${NODE}"

echo "Removing temporary node folder..."
rm -rf /tmp/${NODE}

echo "Creating fresh node folder..."
mkdir -p /tmp/${NODE}/chains/codetalks_testnet/keystore

echo "Storing aura keys for ${NODE}..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert --base-path /tmp/${NODE} \
  --chain ./node/chain-specs/codetalks-testnet.json \
  --scheme Sr25519 \
  --suri <SR25519-SECRET-SEED> \
  --password <SR25519-PASSWORD> \
  --key-type aura

echo "Storing grandpa keys for ${NODE}..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert \
  --base-path /tmp/${NODE} \
  --chain ./node/chain-specs/codetalks-testnet.json \
  --scheme Ed25519 \
  --suri <ED25519-SECRET-SEED> \
  --password <ED25519-PASSWORD> \
  --key-type gran

echo "Check whether keys were stored..."
[ "$(ls -A /tmp/${NODE}/chains/codetalks_testnet/keystore)" ] && echo "Keys were stored" || exit 1

echo "Finished"