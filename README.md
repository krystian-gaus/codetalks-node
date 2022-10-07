# code.talks Node

The node is based on the FRAME-based [Substrate](https://www.substrate.io/) node, ready for hacking :rocket: (see [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template)). The code is based on a [Substrate exercise](https://github.com/krystian-gaus/substrate-exercise-7) that I completed within the [Substrate Runtime Developer Academy](https://www.industryconnect.org/substrate-runtime-developer-academy/).

**Remark:** The entire three-step process of launching a multi-node network is described in the [wiki](https://github.com/krystian-gaus/codetalks-node/wiki).

## Install Substrate

Run
```bash
sudo apt install build-essential
sudo apt install --assume-yes git clang curl libssl-dev
sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

More information can be found [here](https://docs.substrate.io/install/linux/).

## Build

Use Rust's native `cargo` command to build the template node:

```sh
cargo build --release
```

## Setup code.talks testnet

The **codetalks-node** contains the scripts to launch both a **local** and a **distributed** testnet.

- If you want to use [Polkadot Telemetry](https://telemetry.polkadot.io/#/0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3) or the [Polkadot{.js} app](https://polkadot.js.org/apps), it makes sense to set up port forwarding for the ports _30333_, _9945_, _9933_ (`node01`), _30334_, _9946_, _9934_ (`node02`), _30335_, _9947_,  _9935_ (`node03`).
- Note that the test network does _not_ use the default port _9944_ for the [Polkadot{.js} app](https://polkadot.js.org/apps), but the custom port _9945_ instead (or alternatively 9946, 9947).
- To display the nodes in the [telemetry app](https://telemetry.polkadot.io/#/0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3), add the **code.talks testnet** via the **[...](https://telemetry.polkadot.io/#all-chains/)** tab.

### Setup geographically distributed code.talks testnet

Assuming the nodes are running on different servers:

0. SSH into the **first server**, modify the `<HOST_IP>` of the `node/chain-specs/plain/codetalks-testnet-plain.json` and run
```bash
# create raw spec file
./target/release/node-template build-spec --chain=./node/chain-specs/plain/codetalks-testnet-plain.json --raw --disable-default-bootnode > ./node/chain-specs/codetalks-testnet.json
```
Then **copy** the freshly generated `node/chain-specs/codetalks-testnet.json` to the **other servers.**

1. SSH into the **first server** and run
```bash
# store keys
./node/chain-specs/scripts/distributed/1a-store-node01-keys.sh
# start bootnode
./node/chain-specs/scripts/distributed/1b-start-node01.sh
```
2. SSH into the **second server** and run
```bash
# store keys
./node/chain-specs/scripts/distributed/2a-store-node02-keys.sh
# start second node
./node/chain-specs/scripts/distributed/2b-start-node02.sh
```
3. SSH into the **third server** and run
```bash
# store keys
./node/chain-specs/scripts/distributed/3a-store-node03-keys.sh
# start third node
./node/chain-specs/scripts/distributed/3b-start-node03.sh
```

### Setup local code.talks testnet

Assuming the nodes are running on the same server:
1. Open a terminal and run
```bash
# store keys
./node/chain-specs/scripts/local/0-store-keys.sh
# start bootnode
./node/chain-specs/scripts/local/1-start-node01.sh
```
2. Open a **second terminal** and run
```bash
# start second node
./node/chain-specs/scripts/local/2-start-node02.sh
```
3. Open a **third terminal** and run
```bash
# start third node
./node/chain-specs/scripts/local/3-start-node03.sh
```

## Particle Structure for polkadot{.js}

Add the following JSON to the [polkadot{.js} Developer Settings](https://polkadot.js.org/apps/#/settings/developer) to display events:
```json
{
  "ParticleIndex": "u32",
  "ParticleIndexOf": "u32",
  "Particle": {
    "state": "[u8; 16]"
  },
  "ClassId": "u32",
  "TokenId": "u32"
}
```


---

# More on setting up a FRAME-based Substrate node

The following information is from the original [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template).

## Getting Started

Follow these steps to get started with the Node Template :hammer_and_wrench:

### Rust Setup

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-template -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-template -lruntime=debug --dev
```

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to
[our Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).

## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few
directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

-   Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
    nodes in the network to communicate with one another.
-   Consensus: Blockchains must have a way to come to
    [consensus](https://substrate.dev/docs/en/knowledgebase/advanced/consensus) on the state of the
    network. Substrate makes it possible to supply custom consensus engines and also ships with
    several consensus mechanisms that have been built on top of
    [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
-   RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

-   [`chain_spec.rs`](./node/src/chain_spec.rs): A
    [chain specification](https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec) is a
    source code file that defines a Substrate chain's initial (genesis) state. Chain specifications
    are useful for development and testing, and critical when architecting the launch of a
    production chain. Take note of the `development_config` and `testnet_genesis` functions, which
    are used to define the genesis state for the local development chain configuration. These
    functions identify some
    [well-known accounts](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#well-known-keys)
    and use them to configure the blockchain's initial state.
-   [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
    the libraries that this file imports and the names of the functions it invokes. In particular,
    there are references to consensus-related topics, such as the
    [longest chain rule](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#longest-chain-rule),
    the [Aura](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#aura) block authoring
    mechanism and the
    [GRANDPA](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#grandpa) finality
    gadget.

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/node-template --help
```

### Runtime

In Substrate, the terms
"[runtime](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#runtime)" and
"[state transition function](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#stf-state-transition-function)"
are analogous - they refer to the core logic of the blockchain that is responsible for validating
blocks and executing the state changes they define. The Substrate project in this repository uses
the [FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame) framework to construct a
blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules
called "pallets". At the heart of FRAME is a helpful
[macro language](https://substrate.dev/docs/en/knowledgebase/runtime/macros) that makes it easy to
create pallets and flexibly compose them to create blockchains that can address
[a variety of needs](https://www.substrate.io/substrate-users/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note
the following:

-   This file configures several pallets to include in the runtime. Each pallet configuration is
    defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
-   The pallets are composed into a single runtime by way of the
    [`construct_runtime!`](https://crates.parity.io/frame_support/macro.construct_runtime.html)
    macro, which is part of the core
    [FRAME Support](https://substrate.dev/docs/en/knowledgebase/runtime/frame#support-library)
    library.

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with the
[core Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a
template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is compromised of a number of blockchain primitives:

-   Storage: FRAME defines a rich set of powerful
    [storage abstractions](https://substrate.dev/docs/en/knowledgebase/runtime/storage) that makes
    it easy to use Substrate's efficient key-value database to manage the evolving state of a
    blockchain.
-   Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched)
    from outside of the runtime in order to update its state.
-   Events: Substrate uses [events](https://substrate.dev/docs/en/knowledgebase/runtime/events) to
    notify users of important changes in the runtime.
-   Errors: When a dispatchable fails, it returns an error.
-   Config: The `Config` configuration interface is used to define the types and parameters upon
    which a FRAME pallet depends.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
