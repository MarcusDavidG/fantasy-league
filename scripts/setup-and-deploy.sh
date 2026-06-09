#!/usr/bin/env bash
# setup-and-deploy.sh
# Installs Stellar CLI, deploys to testnet + mainnet, and writes .env.local
# Usage: bash setup-and-deploy.sh <your-key-name> <your-mainnet-stellar-address>
# Example: bash setup-and-deploy.sh alice GXXXXXX...

set -euo pipefail

KEY="${1:-alice}"
MAINNET_ADDRESS="${2:-}"
WASM="target/wasm32-unknown-unknown/release/fantasy_sports.wasm"

echo "========================================"
echo " Stellar Fantasy League — Full Deploy"
echo "========================================"
echo ""

# ── 1. Install system deps + Stellar CLI ────────────────────────────────────
echo "==> Installing system dependencies..."
sudo apt-get install -y libdbus-1-dev libudev-dev pkg-config

echo "==> Installing Stellar CLI..."
cargo install stellar-cli
export PATH="$HOME/.cargo/bin:$PATH"
echo "    stellar $(stellar --version)"
echo ""

# ── 2. Build WASM ───────────────────────────────────────────────────────────
echo "==> Building contract WASM..."
cargo build --target wasm32-unknown-unknown --release
echo "    Built: $WASM"
echo ""

# ── 3. Testnet deploy ───────────────────────────────────────────────────────
echo "==> [TESTNET] Setting up key..."
stellar keys generate --network testnet "$KEY" 2>/dev/null || true
stellar keys fund --network testnet "$KEY" 2>/dev/null || true

echo "==> [TESTNET] Deploying contract..."
TESTNET_CONTRACT=$(stellar contract deploy \
  --wasm "$WASM" \
  --source "$KEY" \
  --network testnet)
echo "    Contract: $TESTNET_CONTRACT"

echo "==> [TESTNET] Deploying XLM SAC..."
TESTNET_XLM_SAC=$(stellar contract asset deploy \
  --asset native \
  --source "$KEY" \
  --network testnet)
echo "    XLM SAC: $TESTNET_XLM_SAC"
echo ""

# ── 4. Mainnet deploy ───────────────────────────────────────────────────────
echo "==> [MAINNET] About to deploy — this costs real XLM (~0.5 XLM)"
if [[ -z "$MAINNET_ADDRESS" ]]; then
  read -p "   Enter your mainnet Stellar address (G...): " MAINNET_ADDRESS
fi
read -p "   Continue with mainnet deploy? (y/N) " confirm
[[ "$confirm" =~ ^[Yy]$ ]] || { echo "Skipping mainnet."; MAINNET_CONTRACT=""; MAINNET_XLM_SAC=""; }

if [[ -n "$MAINNET_ADDRESS" && "$confirm" =~ ^[Yy]$ ]]; then
  # Add mainnet key from address (assumes you'll sign via Lobstr or external signer)
  stellar keys add "$KEY-mainnet" --identity "$MAINNET_ADDRESS" 2>/dev/null || true

  echo "==> [MAINNET] Deploying contract..."
  MAINNET_CONTRACT=$(stellar contract deploy \
    --wasm "$WASM" \
    --source "$KEY-mainnet" \
    --network mainnet)
  echo "    Contract: $MAINNET_CONTRACT"

  echo "==> [MAINNET] Deploying XLM SAC..."
  MAINNET_XLM_SAC=$(stellar contract asset deploy \
    --asset native \
    --source "$KEY-mainnet" \
    --network mainnet)
  echo "    XLM SAC: $MAINNET_XLM_SAC"
fi

# ── 5. Write .env.local ─────────────────────────────────────────────────────
echo ""
echo "==> Writing frontend/.env.local..."
cat > frontend/.env.local <<EOF
VITE_TESTNET_CONTRACT_ID=${TESTNET_CONTRACT}
VITE_TESTNET_XLM_SAC=${TESTNET_XLM_SAC}
VITE_MAINNET_CONTRACT_ID=${MAINNET_CONTRACT:-}
VITE_MAINNET_XLM_SAC=${MAINNET_XLM_SAC:-}
EOF

echo ""
echo "========================================"
echo "✅ All done!"
echo ""
echo "  Testnet contract : $TESTNET_CONTRACT"
echo "  Testnet XLM SAC  : $TESTNET_XLM_SAC"
[[ -n "${MAINNET_CONTRACT:-}" ]] && echo "  Mainnet contract : $MAINNET_CONTRACT"
[[ -n "${MAINNET_XLM_SAC:-}" ]]  && echo "  Mainnet XLM SAC  : $MAINNET_XLM_SAC"
echo ""
echo "Next steps:"
echo "  1. cd frontend && npm run build"
echo "  2. Push to GitHub"
echo "  3. Import project on vercel.com, set the env vars above"
echo "  4. Deploy!"
echo "========================================"
