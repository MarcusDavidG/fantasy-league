#!/usr/bin/env bash
# deploy-mainnet.sh — Deploy the fantasy_sports contract to Stellar Mainnet
# Usage: bash deploy-mainnet.sh <your-stellar-key-name>
# Example: bash deploy-mainnet.sh alice
#
# IMPORTANT: This costs real XLM (~0.5 XLM per deploy). Confirm before running.

set -euo pipefail

KEY="${1:-alice}"
NETWORK="mainnet"
WASM="target/wasm32-unknown-unknown/release/fantasy_sports.wasm"

echo "⚠️  You are deploying to MAINNET. This costs real XLM."
read -p "   Continue? (y/N) " confirm
[[ "$confirm" =~ ^[Yy]$ ]] || { echo "Aborted."; exit 1; }

echo "==> Building WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "==> Deploying contract..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM" \
  --source "$KEY" \
  --network "$NETWORK")

echo ""
echo "✅ Mainnet contract deployed!"
echo "   CONTRACT_ID: $CONTRACT_ID"
echo ""

# Deploy XLM SAC for mainnet
echo "==> Deploying XLM Stellar Asset Contract on mainnet..."
XLM_SAC=$(stellar contract asset deploy \
  --asset native \
  --source "$KEY" \
  --network "$NETWORK")

echo "   XLM_SAC: $XLM_SAC"
echo ""
echo "Add to frontend/.env.local:"
echo "  VITE_MAINNET_CONTRACT_ID=$CONTRACT_ID"
echo "  VITE_MAINNET_XLM_SAC=$XLM_SAC"
