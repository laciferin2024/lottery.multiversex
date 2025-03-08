# 🛰️ Mission Reporting

**👤 Hiro**

📜 **Mission:** [Mission.md](./Mission.md)

## 1. Token Contract:

[LTRY-94ac38](https://devnet-explorer.multiversx.com/tokens/LTRY-94ac38)

## 2. Lottery Contract:

Find latest contract in `interactor/state.toml`

Mission
Accomplished: [Tx](https://devnet-explorer.multiversx.com/transactions/49dd0c9d51dc78627d700bc2e23cfe9ef17d3e0b85b6ab1185405a8a9fe61e2d)

For the sake of simplicity I set num_participants to 1 and its working as in the above tx.

### TODO:

- Interactor tests with EGLD Token (would allow me to test with test_wallets)
- Use Lottery Contract as Super Dapp with its own token[module]. Its integrated but need to verify the feasibility of
  this solution

### 3. AMM

- [x] Contract prototyped

> I reckon there are some finer implementations of the same.

- [x] Supports liquidity with EGLD [src/amm.rs#L24](src/amm.rs#L24)

### TODO:

- [x] Tests with scenarios
- [x] Write Integrate tests