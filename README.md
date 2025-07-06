# VoidCaster

**VoidCaster** is an experimental blockchain implementation designed to demonstrate and explore consensus protocols. It uniquely integrates both **Tendermint** (classical BFT consensus) and **Bullshark-Narwhal** (DAG-based BFT) as co-existing or switchable consensus layers, paired with a simple modular execution engine.

> ⚠️ This project is educational and experimental. It is not production-ready.

---

## 🚀 Features

- 🧠 **Dual Consensus Layer**: Supports both [Tendermint](https://tendermint.com/) and [Bullshark-Narwhal](https://arxiv.org/abs/2302.12256)
- 🧩 **Modular Architecture**: Decoupled consensus, execution, storage, and networking
- 🔐 **libp2p Networking**: Gossip and point-to-point messages via libp2p
- ⚙️ **Custom Execution Engine**: Interprets transactions and mutates state
- 📄 **Ethereum-Compatible Block Format** (optional)
- 🧪 **Designed for Consensus Visualization, Testing & Research**

---
