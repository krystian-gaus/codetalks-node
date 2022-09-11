#!/bin/bash

# exit when any command fails
set -e

BASE_DIR=$(dirname $(readlink -f $0))
CODETALKS_DIR=${BASE_DIR}/../../../../
NODE=node03

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
  --suri 0x530144a9003dde2c5598019165c5b83ffaffcd0209a4e31da8a7ffec04e26730 \
  --password "noether-keys-for-aura" \
  --key-type aura

echo "Storing grandpa keys for ${NODE}..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert \
  --base-path /tmp/${NODE} \
  --chain ./node/chain-specs/codetalks-testnet.json \
  --scheme Ed25519 \
  --suri 0xaa563c025ecfae1a047bd28388585c8ba04c65285b82e1240524f711fe67c739 \
  --password "noether-keys-for-grandpa" \
  --key-type gran

echo "Check whether keys were stored..."
[ "$(ls -A /tmp/${NODE}/chains/codetalks_testnet/keystore)" ] && echo "Keys were stored" || exit 1

echo "Finished"