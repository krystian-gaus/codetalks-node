#!/bin/bash

# exit when any command fails
set -e

BASE_DIR=$(dirname $(readlink -f $0))
CODETALKS_DIR=${BASE_DIR}/../../../../
NODE=node02

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
  --suri 0x4b4187762f399a1aff6a7c3adcb6dc081ffe54b355389d4144837f4436e7e678 \
  --password "hamilton-keys-for-aura" \
  --key-type aura

echo "Storing grandpa keys for ${NODE}..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert \
  --base-path /tmp/${NODE} \
  --chain ./node/chain-specs/codetalks-testnet.json \
  --scheme Ed25519 \
  --suri 0xfe57620affda860d170387bc519a9cb828f76d0c1f9528bb80da973f4b8bce36 \
  --password "hamilton-keys-for-grandpa" \
  --key-type gran

echo "Check whether keys were stored..."
[ "$(ls -A /tmp/${NODE}/chains/codetalks_testnet/keystore)" ] && echo "Keys were stored" || exit 1

echo "Finished"