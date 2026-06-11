# ⚽ Stellar Fantasy League

**A trustless, fully on-chain fantasy sports contest platform built on Stellar Soroban.**

Live dApp → [stellar-fantasy-league.vercel.app](https://stellar-fantasy-league.vercel.app) *(after deployment)*

---

## The Problem

Every fantasy sports league — whether on a platform like DraftKings or in a WhatsApp group — has the same problem: **someone holds the money**. That means payouts can be delayed, disputed, or never happen at all.

## The Solution

Stellar Fantasy League replaces the middleman with a smart contract. Entry fees go directly into escrow on-chain. When the contest ends, the contract pays out winners instantly and automatically. If a contest is cancelled, everyone gets a full refund — no questions asked, no waiting.

> **"Run fantasy sports leagues where the rules are enforced by code, not people."**

---

## Most Popular Use Case

A group of friends each put in 5 XLM for a Champions League matchweek. The organizer creates a contest, everyone joins by paying the entry fee, the contract holds the pool. After the matches, the organizer declares the top 3 — the contract instantly splits 60/30/10 to winners. No one chases anyone for money. No disputes. Fully verifiable on-chain.

This works for:
- **Friend group football/soccer prediction leagues**
- **Office Premier League sweepstakes**
- **FIFA World Cup tournament brackets**
- **NBA/NFL weekly pick'em pools**

---

## How It Works

| Function | Who | What Happens |
|---|---|---|
| `create_contest` | Contest creator | Sets entry fee, prize split, max players |
| `join_contest` | Any player | Pays entry fee; contract holds escrow |
| `declare_winners` | Creator only | Distributes prize pool to 1/2/3 winners per split |
| `cancel_contest` | Creator only | Cancels contest, refunds all participants |
| `get_contest` | Anyone | Read-only view of contest + participants |

---

## Key Features

- **Trustless escrow** — contract holds all funds, no one can run off with the pot
- **Instant payouts** — winners receive funds the moment they're declared
- **Multi-winner prize splits** — configure 60/30/10 or any split summing to 100%
- **Cancel & full refund** — every participant gets their entry fee back if cancelled
- **Max participants cap** — limit contest size (0 = unlimited)
- **Duplicate entry protection** — one join per address enforced on-chain
- **Low fees** — Stellar transactions cost fractions of a cent
- **XLM native** — uses Stellar's native token via the Stellar Asset Contract (SAC)
- **Freighter/Lobstr wallet support** — connect with popular Stellar wallets
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
├── ROADMAP.md
└── Cargo.toml
```

---

## Getting Started

### Prerequisites
- Rust + `wasm32-unknown-unknown` target
- Node.js v18+
- [Stellar CLI v26+](https://developers.stellar.org/docs/tools/cli/install-cli)

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli
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

Each script prints the contract ID and XLM SAC address to add to your frontend env.

### Run Frontend Locally

```bash
cp frontend/.env.example frontend/.env.local
# fill in contract IDs from deployment output
cd frontend && npm install && npm run dev
```

### Deploy to Vercel

1. Push repo to GitHub
2. Import project at [vercel.com](https://vercel.com)
3. Set environment variables from `.env.local`
4. Deploy — Vercel auto-detects Vite

---

## Deployments

| Network | Contract ID |
|---|---|
| Testnet | `CDLDKWJMCJZQD77MWQBRISKKSJPZO6B623UVNQRYB2EX6YZ3SDC4TKF3` |
| Mainnet | *(set after deploy)* |

---

## Open Issues (Good for Contributors)

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

## License

MIT
