#  On-Chain Fantasy Sports Marketplace (Stellar Soroban)

Welcome to the future of decentralized fantasy sports! This project is a **fully on-chain fantasy sports marketplace** built on the high-performance **Stellar network** using **Soroban smart contracts**.

By utilizing Stellar's ultra-low fees, sub-second consensus times, and Soroban's robust Rust-based smart contract environment, this platform enables completely transparent, trustless, and decentralized fantasy contest creation, entry fee collection, and reward distribution.

---

##  Project Overview

The core of this marketplace is a secure, warning-free, and thoroughly tested Soroban smart contract written in Rust. It serves as a decentralized game master that:
1. **Creates Contests**: Managers/creators can spin up fantasy contests with specific entry fees, token assets (e.g., USDC, XLM wrapped tokens), and descriptive metadata.
2. **Collects Entry Fees**: Players join contests trustlessly. The contract pulls entry fees from the players' accounts and holds them securely in its own address as an escrow prize pool.
3. **Declares Winners & Distributes Rewards**: Once the real-world sports events finish, the creator submits the winner. The smart contract immediately transfers the entire prize pool escrow directly to the winner's Stellar wallet address.

---

## 📁 Repository Structure

The project follows a standard Soroban workspace layout:

```text
dream11/
├── Cargo.toml                        # Workspace cargo configuration
├── README.md                         # Project documentation and developer guide
└── contracts/
    └── fantasy_sports/
        ├── Cargo.toml                # Smart contract cargo dependencies
        └── src/
            ├── lib.rs                # Core smart contract code
            └── test.rs               # Complete test suite (100% pass)
```

---

## 🛠️ Smart Contract Architecture

The contract implements four fundamental functions:

| Function | Access | Description |
|---|---|---|
| `create_contest` | **Creator Signature Required** | Initializes a new contest by defining its unique ID, entry fee, details, and the token used. |
| `join_contest` | **Participant Signature Required** | Escrows the entry fee from the participant into the contract and adds it to the contest's prize pool. |
| `declare_winner` | **Creator Signature Required** | Resolves the contest by validating the winner and distributing 100% of the escrowed pool to their address. |
| `get_contest` | **Public Getter** | Returns the current state (prize pool, participants, status) of any contest. |

---

## 💻 Developer & Contribution Guide

### 1. Prerequisites
Ensure you have the following installed on your machine:
* [Rust & Cargo](https://rustup.rs/) (v1.75+)
* Target WASM build chain:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
* [Stellar CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup#install-the-stellar-cli) (formerly Soroban CLI):
  ```bash
  cargo install --locked stellar-cli --features opt
  ```

### 2. Running Tests
The smart contract includes a comprehensive test suite in `contracts/fantasy_sports/src/test.rs` which compiles cleanly and tests full flows, edge cases, and safety checks.

Run the tests using Cargo:
```bash
cargo test
```

### 3. Compiling to WebAssembly (WASM)
To compile the contract into optimized, deployable WebAssembly byte code:
```bash
cargo build --target wasm32-unknown-unknown --release
```
The compiled contract will be saved at:
`target/wasm32-unknown-unknown/release/fantasy_sports.wasm`

### 4. Deploying to Stellar Testnet

First, configure your Stellar CLI testnet identity:
```bash
stellar keys generate --network testnet alice
```

Deploy the compiled WASM file:
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/fantasy_sports.wasm \
  --source alice \
  --network testnet
```
This will return a **Contract ID** (e.g., `CD...`). Save this address!

### 5. Invoking the Contract

To create a contest via CLI:
```bash
stellar contract invoke \
  --id <YOUR_CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- \
  create_contest \
  --contest_id 1 \
  --creator alice \
  --token <USDC_TOKEN_ADDRESS> \
  --entry_fee 10000000 \
  --details "Match-Week-1-Premier-League"
```

---

## 🤝 Contributing

We are building the premier decentralized home for fantasy sports and we would love your help!

If you want to contribute:
1. **Fork the repo** and create a feature branch.
2. Check our open issues for planned improvements (e.g., multi-winner prize splits, team roster locking, decentralized oracle integration).
3. Open a **Pull Request** detailing your changes.

Let's make fantasy sports transparent and fun together! ⚽🏀🏏
