#!/bin/sh
export COREUM_CHAIN_ID="coreum-testnet-1"
export COREUM_DENOM="utestcore"
export COREUM_NODE="https://full-node.testnet-1.coreum.dev:26657"
export COREUM_VERSION="v2.0.2"

export COREUM_CHAIN_ID_ARGS="--chain-id=$COREUM_CHAIN_ID"
export COREUM_NODE_ARGS="--node=$COREUM_NODE"

export COREUM_HOME=$HOME/.core/"$COREUM_CHAIN_ID"

export COREUM_BINARY_NAME=$(arch | sed s/aarch64/cored-linux-arm64/ | sed s/x86_64/cored-linux-amd64/)

export PATH=$PATH:$COREUM_HOME/bin
