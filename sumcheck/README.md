# 🧮 Sumcheck Protocol

Hey! 👋 Welcome to my implementation of the Sumcheck Protocol.

## 🤔 What's going on here?

The sumcheck protocol is arguably one of the most important building blocks in modern interactive proofs (it's the backbone of GKR, for example!). This folder contains my from-scratch implementation of the protocol. It allows a prover to convince a verifier about the sum of a multivariate polynomial evaluated over a boolean hypercube, without the verifier having to compute the whole sum themselves.

## 🧮 How it works (Simplified)

The prover basically says "Hey, the sum of this massive polynomial over all binary inputs is `H`." Instead of the verifier checking all inputs (which takes forever), they do a back-and-forth dance with the prover. In each round, they "strip away" one variable by substituting a random field element. By the end, the verifier only has to evaluate the polynomial at one single random point to verify the entire sum! 

## ✅ What's implemented so far

- [x] Core interactive protocol loop
- [x] Prover logic (computing marginal polynomials round by round)
- [x] Verifier logic (checking degrees and evaluating random challenges)
- [ ] Non-interactive version using Fiat-Shamir

## 🚧 What's next?

- Plugging this directly into the GKR protocol implementation.
- Making the prover much more efficient (using book-keeping tables to avoid recomputing stuff).

## 💻 How to play around with this

To run the sumcheck protocol tests:

```bash
cargo test --manifest-path sumcheck/Cargo.toml
```
