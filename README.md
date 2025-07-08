# VoidCaster

**VoidCaster** is an experimental blockchain implementation designed to demonstrate and explore consensus protocols. It uniquely integrates both **Tendermint** (classical BFT consensus) and **Bullshark-Narwhal** (DAG-based BFT) as co-existing or switchable consensus layers, paired with a simple modular execution engine.

> ⚠️ This project is educational and experimental. It is not production-ready.

---

## 🚀 Features

- 🧠 **Dual Consensus Layer**: Supports both [Tendermint](https://tendermint.com/) and [Bullshark-Narwhal](https://arxiv.org/pdf/2201.05677)
- 🧩 **Modular Architecture**: Decoupled consensus, execution, storage, and networking
- 🔐 **libp2p Networking**: Gossip and point-to-point messages via libp2p
- ⚙️ **Custom Execution Engine**: Interprets transactions and mutates state
- 📄 **Ethereum-Compatible Block Format** (optional)
- 🧪 **Designed for Consensus Visualization, Testing & Research**

---

## 📦 Branches

This repository maintains separate branches for different consensus protocol implementations:

- [`consensus/tendermint`](https://github.com/sajeevanjspy/void-caster/tree/consensus/tendermint) – Classic Tendermint BFT
- [`consensus/bullshark`](https://github.com/sajeevanjspy/void-caster/tree/consensus/bullshark) – DAG-based Bullshark/Narwhal BFT

---
