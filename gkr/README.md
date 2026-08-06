# ⚡ GKR Protocol

Hey! 👋 Welcome to the GKR section of my ZK research repo.

## 🤔 What's going on here?

This folder is where I'm tackling the Goldwasser-Kalai-Rothblum (GKR) protocol. It's an interactive proof system specifically designed for layered arithmetic circuits. The really cool thing about GKR is that the prover's time is strictly tied to the size of the circuit, and the verifier gets away with doing very little work. 

## 🧮 How it works (Simplified)

Imagine an arithmetic circuit as a bunch of layers (gates doing addition or multiplication). GKR works by using the sumcheck protocol (which I implemented elsewhere in this repo!) recursively, layer by layer, starting from the outputs all the way back to the inputs. The verifier basically "reduces" the claim about the output layer to a claim about the input layer.

## ✅ What's implemented so far

- [ ] Circuit representation (layers, gates, wires)
- [ ] Multilinear extension generation for layers
- [ ] Integrating the sumcheck protocol for layer-to-layer reduction
- [ ] End-to-end Prover and Verifier interaction

## 🚧 What's next?

- Optimizing the prover time (maybe some hardware acceleration down the line?)
- Cleaning up the circuit API to make it easier to define custom circuits.

## 💻 How to play around with this

To run the tests and step through a sample circuit execution:

```bash
cargo test --manifest-path gkr/Cargo.toml
```