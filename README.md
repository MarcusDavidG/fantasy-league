# ⚽ Stellar Fantasy League

**A fully on-chain fantasy sports contest platform built on Stellar Soroban.**

Live dApp → [stellar-fantasy-league.vercel.app](https://stellar-fantasy-league.vercel.app) *(after deployment)*

---

## What It Does

Stellar Fantasy League lets anyone create and participate in trustless fantasy sports contests where entry fees and prize payouts are handled entirely by a Soroban smart contract — no middleman, no manual payouts, fully verifiable on-chain.

| Flow | Who | What Happens |
|---|---|---|
| `create_contest` | Contest creator | Sets entry fee, prize split, max players |
| `join_contest` | Any player | Pays entry fee; contract holds escrow |
| `declare_winners` | Creator only | Distributes prize pool to 1/2/3 winners per split |
| `cancel_contest` | Creator only | Cancels contest, refunds all participants |
| `get_contest` | Anyone | Read-only view of contest + participants |

---

## Key Features

- **Multi-winner prize splits** — configure 60/30/10 or any split summing to 100%
- **Cancel & refund** — full participant refunds if the contest is cancelled
- **Max participants cap** — limit contest size (0 = unlimited)
- **Duplicate entry protection** — one join per address enforced on-chain
- **XLM native** — uses Stellar's native token via the Stellar Asset Contract (SAC)
- **Frontend dApp** — React + TypeScript, Lobstr/Freighter wallet support
- **Dual deployment** — testnet + mainnet contracts

---

## Repository Structure

```
stellar-fantasy-league/
├── contracts/
│   └── fantasy_sports/
│       └── src/
│           ├── lib.rs        # Soroban smart contract
│           └── test.rs       # 8 tests, 100% passing
├── frontend/                 # React + TypeScript dApp
│   ├── src/
│   │   ├── contract.ts       # Contract interaction (build/submit transactions)
│   │   ├── wallet.ts         # Wallet connection hook
│   │   ├── App.tsx           # Root component
│   │   └── components/
│   │       └── ContestPanel.tsx
│   ├── vercel.json
│   └── .env.example
├── scripts/
│   ├── deploy-testnet.sh
│   └── deploy-mainnet.sh
└── Cargo.toml
```

---

## Getting Started

### Prerequisites
- Rust + `wasm32-unknown-unknown` target
- Node.js v18+
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli/install-cli)

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
```

### Run Tests

```bash
cargo test
```

### Build WASM

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Deploy

```bash
# Testnet (free)
bash scripts/deploy-testnet.sh alice

# Mainnet (requires real XLM)
bash scripts/deploy-mainnet.sh alice
```

Each script prints the contract ID and XLM SAC address to add to the frontend env.

### Run Frontend Locally

```bash
cp frontend/.env.example frontend/.env.local
# fill in contract IDs from deployment output
cd frontend && npm install && npm run dev
```

### Deploy to Vercel

1. Push repo to GitHub
2. Import project in [vercel.com](https://vercel.com)
3. Set environment variables from `.env.local`
4. Deploy — Vercel auto-detects Vite

---

## Open Issues (Good for Contributors)

See the [Issues tab](../../issues) for open tasks including:

- **Good first:** Add loading skeleton UI components
- **Good first:** Write E2E tests with Playwright
- **Medium:** USDC token support alongside XLM
- **Medium:** Contest history page (read from Horizon events)
- **Hard:** Decentralized sports score oracle integration
- **Hard:** Multi-sig winner declaration (2-of-3 creator votes)

---

## Contributing

1. Fork the repo and create a feature branch
2. For contract changes, run `cargo test` before opening a PR
3. For frontend changes, run `npm run build` before opening a PR
4. Open a PR with a clear description of what you changed and why

---

## Deployments

| Network | Contract ID |
|---|---|
| Testnet | *(set after deploy)* |
| Mainnet | *(set after deploy)* |

---

## License

MIT
