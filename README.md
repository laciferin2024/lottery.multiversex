# 🛰️ Mission Reporting

> ⚠️ **Disclaimer:** This project is a work in progress. Due to time constraints, I have prioritized end-to-end
> functionality for key contract features, which can be found here: [i.rs](interactor/src/i.rs). In **v2**, we will focus
> on stabilizing the remaining functionalities.


**👤 Hiro**

📜 **Mission:** [Mission.md](./Mission.md)

---

## 🚀 1. Token Contract

🔗 **LTRY-94ac38:** [View on Explorer](https://devnet-explorer.multiversx.com/tokens/LTRY-94ac38)

---

## 🎲 2. Lottery Contract

📌 **Find latest contract in:** `interactor/state.toml`

✅ **Mission Accomplished:
** [Transaction](https://devnet-explorer.multiversx.com/transactions/49dd0c9d51dc78627d700bc2e23cfe9ef17d3e0b85b6ab1185405a8a9fe61e2d)

> For simplicity, `num_participants` was set to `1`, and it works as shown in the above transaction.

### 🔧 TODO:

- [ ] Implement **Interactor tests with EGLD** (allows testing with `test_wallets`).
- [ ] Use **Lottery Contract as a Super Dapp** with its own token module.
    - ✅ Integrated, but feasibility needs verification.

---

## 💱 3. AMM

✅ **Contract Prototyped**
> _There might be more refined implementations of the same._

✅ **Supports Liquidity with EGLD:** [View Code](src/amm.rs#L24)

### 🔧 TODO:

- [x] **Tests with Scenarios**
- [x] **Write Integration Tests**  


