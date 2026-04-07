#!/usr/bin/env bash
set -euo pipefail

CLUSTER="${CLUSTER:-devnet}"
KEYPAIR="${KEYPAIR:-$HOME/.config/solana/id.json}"

echo "Building workspace for $CLUSTER"
cargo build-sbf --manifest-path programs/fixr_core/Cargo.toml

echo "Deploying program to $CLUSTER"
solana program deploy \
  --keypair "$KEYPAIR" \
  --url "$CLUSTER" \
  target/deploy/fixr_core.so

echo "Deploy complete"
# rev-01120
