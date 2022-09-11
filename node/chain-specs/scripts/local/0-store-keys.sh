#!/bin/bash

# exit when any command fails
set -e

BASE_DIR=$(dirname $(readlink -f $0))
CODETALKS_DIR=${BASE_DIR}/../../../../

echo "$BASE_DIR"
echo "$CODETALKS_DIR"

echo "Removing temporary node folders..."
rm -rf /tmp/node01
rm -rf /tmp/node02
rm -rf /tmp/node03

echo "Creating fresh node folders..."
mkdir -p /tmp/node01/chains/codetalks_testnet/keystore
mkdir -p /tmp/node02/chains/codetalks_testnet/keystore
mkdir -p /tmp/node03/chains/codetalks_testnet/keystore

echo "Storing aura keys for node01..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert --base-path /tmp/node01 \
  --chain ./node/chain-specs/codetalks-local-testnet.json \
  --scheme Sr25519 \
  --suri 0x54614ac4b31afed6fb805163a3bc5bd315d5eb8760caee060a2485488dd52ca3 \
  --password "curie-keys-for-aura" \
  --key-type aura

echo "Storing grandpa keys for node01..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert \
  --base-path /tmp/node01 \
  --chain ./node/chain-specs/codetalks-local-testnet.json \
  --scheme Ed25519 \
  --suri 0x635dd1fd6f790a7eb015dbdd40cc402d9c2eac258d3026bbdf97f5f159ba8bc3 \
  --password "curie-keys-for-grandpa" \
  --key-type gran

echo "Check whether keys were stored..."
[ "$(ls -A /tmp/node01/chains/codetalks_testnet/keystore)" ] && echo "Keys were stored" || exit 1

echo "Storing aura keys for node02..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert --base-path /tmp/node02 \
  --chain ./node/chain-specs/codetalks-local-testnet.json \
  --scheme Sr25519 \
  --suri 0x4b4187762f399a1aff6a7c3adcb6dc081ffe54b355389d4144837f4436e7e678 \
  --password "hamilton-keys-for-aura" \
  --key-type aura

echo "Storing grandpa keys for node02..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert \
  --base-path /tmp/node02 \
  --chain ./node/chain-specs/codetalks-local-testnet.json \
  --scheme Ed25519 \
  --suri 0xfe57620affda860d170387bc519a9cb828f76d0c1f9528bb80da973f4b8bce36 \
  --password "hamilton-keys-for-grandpa" \
  --key-type gran

echo "Check whether keys were stored..."
[ "$(ls -A /tmp/node02/chains/codetalks_testnet/keystore)" ] && echo "Keys were stored" || exit 1

echo "Storing aura keys for node03..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert --base-path /tmp/node03 \
  --chain ./node/chain-specs/codetalks-local-testnet.json \
  --scheme Sr25519 \
  --suri 0x530144a9003dde2c5598019165c5b83ffaffcd0209a4e31da8a7ffec04e26730 \
  --password "noether-keys-for-aura" \
  --key-type aura

echo "Storing grandpa keys for node03..."
cd ${CODETALKS_DIR}
./target/release/node-template key insert \
  --base-path /tmp/node03 \
  --chain ./node/chain-specs/codetalks-local-testnet.json \
  --scheme Ed25519 \
  --suri 0xaa563c025ecfae1a047bd28388585c8ba04c65285b82e1240524f711fe67c739 \
  --password "noether-keys-for-grandpa" \
  --key-type gran

echo "Check whether keys were stored..."
[ "$(ls -A /tmp/node03/chains/codetalks_testnet/keystore)" ] && echo "Keys were stored" || exit 1

echo "Finished"