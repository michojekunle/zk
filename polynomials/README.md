# 🔢 Polynomials

Hey! 👋 This is the math engine room of my ZK repo. 

## 🤔 What's going on here?

Before you can build fancy zero-knowledge proofs, you need a rock-solid foundation in polynomial arithmetic. Pretty much everything in ZK (from Plonk to STARKs to Sumcheck) boils down to polynomials. This folder houses all my custom implementations of univariate and multivariate polynomial math over finite fields.

## 🧮 What's inside?

I've basically built a mini algebra library here. It handles the heavy lifting so the other protocols (like KZG and GKR) don't have to worry about the underlying math.

## ✅ What's implemented so far

Here is where you can find the specific math and cryptographic implementations:
- [x] [**Univariate Polynomials**](./src/univariate) (Addition, Multiplication, Division)
- [x] [**Multilinear / Multivariate Polynomials**](./src/multilinear) (Representations and Evaluations, crucial for Sumcheck!)
- [x] [**Composed Polynomials**](./src/composed) 
- [x] [**Shamir's Secret Sharing**](./src/shamir_secret) implementation
- [ ] [Fast Fourier Transforms (FFT)](../fft) for faster multiplications

## 🚧 What's next?

- Implementing highly optimized FFT operations.
- Better memory management for massive polynomials.

## 💻 How to play around with this

To run the math tests and make sure the polynomial engine is humming:

```bash
cargo test --manifest-path polynomials/Cargo.toml
```
