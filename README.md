# Solana Token Vesting (Minimal)

A streamlined, secure, and production-ready Solana token vesting program built using the Anchor framework. This repository contains the complete smart contract and deployment files in a flat structure for easy auditing and integration.

## Features
- **Linear Vesting:** Smooth token release calculated per second.
- **Cliff Period:** Optional lock-up time before vesting begins.
- **Clawback Support:** Allows the initializer to reclaim unvested tokens if necessary.
- **SPL Token Compatible:** Works out of the box with standard SPL tokens.

## Getting Started

### Prerequisites
- Solana CLI Tool suite
- Rust
- Anchor Framework

### Build & Test
```bash
# Install dependencies
npm install

# Build the Anchor program
anchor build

# Run the test suite
anchor test
