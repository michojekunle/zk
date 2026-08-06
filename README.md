# 🛡️ ZK Research Implementations

Hey there! 👋 Welcome to my Zero-Knowledge research implementations repo. 

This is essentially my personal playground and log for all things ZK, which all started back during my time at the Web3Bridge ZK masterclass bootcamp. The goal here is pretty simple: get my hands dirty implementing the math, cryptographic primitives, and protocols that make zero-knowledge proofs work under the hood. 

If you're exploring applied cryptography or just looking to see some ZK math translated into code, you're in the right place.

## 🚀 What's Inside & Current Progress

Here's a breakdown of what I've been building and where things currently stand. 

### 🏗️ Core Primitives
The foundational math and building blocks.

| Protocol / Primitive | Directory | Status | What is it? |
|----------------------|-----------|--------|-------------|
| **Polynomials** | [`/polynomials`](./polynomials) | 🟢 Done | Univariate & multivariate math, evaluations, and basic ops. |
| **Sumcheck Protocol** | [`/sumcheck`](./sumcheck) | 🟢 Done | The interactive sumcheck protocol for multivariate polynomials. |
| **KZG Commitments** | [`/kzg`](./kzg) | 🟡 In Progress | Getting into the Kate-Zaverucha-Goldberg polynomial commitment scheme. |

### 🔐 Proof Systems
Putting the primitives together for actual proof systems.

| Protocol / Primitive | Directory | Status | What is it? |
|----------------------|-----------|--------|-------------|
| **GKR Protocol** | [`/gkr`](./gkr) | 🟡 In Progress | Building out the Goldwasser-Kalai-Rothblum protocol for layered circuits. |

### 🛠️ Tooling & Examples
Playing around with ZK DSLs.

| Protocol / Primitive | Directory | Status | What is it? |
|----------------------|-----------|--------|-------------|
| **Noir Examples** | [`/noir`](./noir) | 🟡 In Progress | Writing smart contracts and circuits using Noir. |

*Status Key: 🟢 Done | 🟡 In Progress | 🔴 On the radar*


## 🗺️ What's Next? (The Roadmap)

There's a lot more I want to build out. Here’s what I'm looking forward to diving into next:

- [ ] **FRI:** Fast Reed-Solomon Interactive Oracle Proofs of Proximity (hello STARKs!).
- [ ] **Plonk:** Really want to explore universal SNARKs and custom gates.
- [ ] **Groth16:** Gotta implement the industry standard at some point.
- [ ] **Halo2:** Accumulation schemes are fascinating.
- [ ] **Merkle Trees / Vector Commitments:** Expanding beyond just KZG.

## 💻 Running the Code

Want to poke around? It's all standard Rust.

**Prerequisites:** Just make sure you have [Rust & Cargo](https://www.rust-lang.org/tools/install) installed.

1. **Clone it down:**
   ```bash
   git clone https://github.com/michojekunle/zk.git
   cd zk
   ```

2. **Run tests for a specific package:**
   ```bash
   # e.g., Let's test the sumcheck protocol
   cargo test --manifest-path sumcheck/Cargo.toml
   ```

*(If you set up the crates as a workspace later, a simple `cargo test` from the root will run everything).*

## 🤝 Let's Connect!

If you're also learning ZK, notice a cool optimization I missed, or just want to chat about cryptography, feel free to open an issue, drop a PR, or reach out. 

## 📄 License

[MIT License](LICENSE) - Do whatever you'd like with this code!
