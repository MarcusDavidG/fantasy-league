# Contributing to Stellar Fantasy League

Thank you for your interest in contributing! This project is listed on [GrantFox](https://contribute.grantfox.xyz) — contributors may be eligible for USDC rewards.

## Quick Start

```bash
# Contract (Rust)
cargo test                  # run all tests
cargo build --target wasm32-unknown-unknown --release

# Frontend (React + TypeScript)
cd frontend
npm install
cp .env.example .env.local  # fill in contract IDs
npm run dev
```

## How to Contribute

1. Browse the [Issues tab](../../issues) and find one labeled `good first issue` or `help wanted`
2. Comment on the issue to claim it — wait for maintainer assignment before starting
3. Fork the repo and create a branch: `git checkout -b feat/your-feature`
4. For contract changes: run `cargo test` — all 8 tests must pass
5. For frontend changes: run `npm run build` — must compile with zero errors
6. Open a PR referencing the issue: `Closes #N`

## Issue Labels

| Label | Meaning |
|---|---|
| `good first issue` | Small, self-contained — great starting point |
| `help wanted` | Open for any contributor |
| `contract` | Touches Rust/Soroban smart contract |
| `frontend` | Touches React/TypeScript dApp |
| `testing` | Adds or improves tests |

## Code Style

- **Rust:** run `cargo fmt` before committing
- **TypeScript:** run `npm run lint` before committing
- Keep PRs focused — one feature or fix per PR

## Questions?

Open a GitHub Discussion or reach out via the [GrantFox community](https://t.me/grantfoxcommunity).
