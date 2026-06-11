# Stellar Fantasy League — Roadmap & Progress

## Current Status: Testnet Deployed ✅

---

## ✅ Completed

### Smart Contract
- [x] `create_contest` — creator sets entry fee, prize split, max participants
- [x] `join_contest` — participant pays entry fee; contract holds escrow
- [x] `declare_winners` — distributes prize pool per 1/2/3-way split
- [x] `cancel_contest` — cancels contest and refunds all participants
- [x] `get_contest` / `get_participants` — read-only views
- [x] Duplicate entry protection (one join per address)
- [x] Max participants cap enforcement
- [x] Prize split validation (must sum to 100%)
- [x] 8 unit tests — 100% passing
- [x] Upgraded to `soroban-sdk v22` (compatible with Stellar CLI v26)

### Frontend
- [x] React + TypeScript + Vite dApp
- [x] Freighter / Lobstr wallet connection
- [x] Create contest UI
- [x] Fetch & display contest details
- [x] Join contest
- [x] Declare winners (creator only)
- [x] Cancel & refund (creator only)
- [x] Testnet / Mainnet network switcher
- [x] Upgraded to `@stellar/stellar-sdk v15`
- [x] Fixed transaction timeout (30s → 300s)

### Infrastructure
- [x] Testnet deployment — `CDLDKWJMCJZQD77MWQBRISKKSJPZO6B623UVNQRYB2EX6YZ3SDC4TKF3`
- [x] Deploy scripts for testnet and mainnet
- [x] Vercel config for frontend deployment
- [x] GitHub repository set up

---

## 🔄 In Progress

- [ ] End-to-end testing of all contract functions via the frontend on testnet
- [ ] Mainnet deployment
- [ ] Frontend deployment to Vercel

---

## 📋 Planned — Short Term

- [ ] Add loading skeleton UI components (good first issue)
- [ ] Write E2E tests with Playwright (good first issue)
- [ ] Update `.env.example` with mainnet contract ID after mainnet deploy
- [ ] Improve error messages shown to users in the UI

---

## 📋 Planned — Medium Term

- [ ] USDC token support alongside XLM
- [ ] Contest history page (read from Horizon events)
- [ ] Mobile-responsive UI improvements
- [ ] Share contest link feature

---

## 📋 Planned — Long Term / Hard

- [ ] Decentralized sports score oracle integration (remove trust from winner declaration)
- [ ] Multi-sig winner declaration (2-of-3 creator votes)
- [ ] Tournament bracket support (multi-round contests)
- [ ] Leaderboard / player profiles

---

## Deployment History

| Date | Event |
|---|---|
| 2026-06-09 | Initial project scaffolded — contract + frontend |
| 2026-06-10 | Stellar CLI v26 installed, testnet account funded |
| 2026-06-10 | Contract deployed to testnet (soroban-sdk v21 — incompatible) |
| 2026-06-10 | Upgraded soroban-sdk to v22, stellar-sdk to v15, fixed tx timeout |
| 2026-06-10 | Contract redeployed to testnet — verified responding on-chain |
| 2026-06-11 | README updated, ROADMAP.md created |
