#!/usr/bin/env bash
# deploy-testnet.sh — Deploy the fantasy_sports contract to Stellar Testnet
# Usage: bash deploy-testnet.sh <your-stellar-key-name>
# Example: bash deploy-testnet.sh alice

set -euo pipefail

KEY="${1:-alice}"
NETWORK="testnet"
WASM="target/wasm32-unknown-unknown/release/fantasy_sports.wasm"

echo "==> Building WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "==> Funding account on testnet (if new)..."
stellar keys generate --network "$NETWORK" "$KEY" 2>/dev/null || true
stellar keys fund --network "$NETWORK" "$KEY" 2>/dev/null || true

echo "==> Deploying contract..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM" \
  --source "$KEY" \
  --network "$NETWORK")

echo ""
echo "✅ Testnet contract deployed!"
echo "   CONTRACT_ID: $CONTRACT_ID"
echo ""

# Deploy XLM SAC (native asset)
echo "==> Deploying XLM Stellar Asset Contract..."
XLM_SAC=$(stellar contract asset deploy \
  --asset native \
  --source "$KEY" \
  --network "$NETWORK")

echo "   XLM_SAC: $XLM_SAC"
echo ""
echo "Add to frontend/.env.local:"
echo "  VITE_TESTNET_CONTRACT_ID=$CONTRACT_ID"
echo "  VITE_TESTNET_XLM_SAC=$XLM_SAC"
