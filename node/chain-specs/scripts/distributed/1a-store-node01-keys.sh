#!/bin/bash

# exit when any command fails
set -e

BASE_DIR=$(dirname $(readlink -f $0))
CODETALKS_DIR=${BASE_DIR}/../../../../
NODE=node01

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
  --suri 0x54614ac4b31afed6fb805163a3bc5bd315d5eb8760caee060a2485488dd52ca3 \
  --password "curie-keys-for-aura" \
  --key-type aura

echo "Storing grandpa keys for ${NODE}..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert \
  --base-path /tmp/${NODE} \
  --chain ./node/chain-specs/codetalks-testnet.json \
  --scheme Ed25519 \
  --suri 0x635dd1fd6f790a7eb015dbdd40cc402d9c2eac258d3026bbdf97f5f159ba8bc3 \
  --password "curie-keys-for-grandpa" \
  --key-type gran

echo "Check whether keys were stored..."
[ "$(ls -A /tmp/${NODE}/chains/codetalks_testnet/keystore)" ] && echo "Keys were stored" || exit 1

echo "Finished"