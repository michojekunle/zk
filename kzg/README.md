# 🧩 KZG Commitments

Hey! 👋 Welcome to the KZG polynomial commitment section of my ZK repo. 

## 🤔 What's going on here?

This directory is all about the Kate-Zaverucha-Goldberg (KZG) polynomial commitment scheme. It's one of the most widely used commitment schemes in zero-knowledge proofs (hello, Plonk!). The goal here is to implement the core mechanics of committing to a polynomial and generating/verifying evaluation proofs.

## 🧮 How it works (Simplified)

Instead of sending an entire massive polynomial, a prover can send a tiny "commitment" (just one group element). When they later claim "hey, my polynomial evaluates to `y` at point `x`", they can provide a small proof to back it up. The verifier can check this proof against the original commitment without ever needing the full polynomial. It's essentially magic, powered by elliptic curve pairings.

## ✅ What's implemented so far

- [ ] Trusted setup generation
- [ ] Polynomial commitment logic
- [ ] Evaluation proof generation
- [ ] Verification logic using pairings

## 🚧 What's next?

- Batch proofs (proving multiple evaluations at once)
- Optimizing polynomial evaluations

## 💻 How to play around with this

To run the tests and see the KZG logic in action:

```bash
cargo test --manifest-path kzg/Cargo.toml
```
