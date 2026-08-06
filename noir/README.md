# 🦇 Noir Examples & Integration

Hey! 👋 Welcome to the Noir section of the repo.

## 🤔 What's going on here?

While the rest of this repo is about building the low-level math and cryptography from scratch, this folder is all about using high-level DSLs (Domain Specific Languages) to actually *use* zero-knowledge proofs in practice. Specifically, I'm playing around with Noir by Aztec.

## 🧮 Why Noir?

Noir is awesome because it lets you write zero-knowledge circuits using syntax that feels remarkably like Rust. You don't have to think about polynomials or R1CS constraints; you just write logic, and the compiler handles the heavy cryptographic lifting.

## ✅ What's in here

- [ ] Basic arithmetic circuits
- [ ] Smart contract verification examples
- [ ] Hashes and signatures in Noir

## 🚧 What's next?

- Writing more complex applications (maybe a mini ZK-rollup or anonymous voting circuit).
- Exploring how Noir compiles down to different proof systems (like Plonk).

## 💻 How to play around with this

Make sure you have [Nargo](https://noir-lang.org/docs/getting_started/installation) installed.

To compile and prove a circuit:
```bash
cd noir/<project-name>
nargo prove
nargo verify
```
