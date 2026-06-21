# HarvestPay Smart Escrow
**Instant, trustless agricultural payments for smallholder farmers via Soroban.**

## Problem & Solution
Smallholder rice farmers wait up to 45 days to receive payment from wholesale buyers, creating crippling working capital deficits. HarvestPay solves this by allowing wholesale buyers to lock USDC on-chain. Upon physical delivery confirmation by an arbiter (the local co-op), the contract instantly releases the funds to the farmer's wallet.

## Timeline
Hackathon MVP deployable in 48 hours.

## Stellar Features Used
- Soroban Smart Contracts
- USDC (Stellar Asset) integration

## Vision and Purpose
To empower agricultural supply chains in emerging markets by eliminating the middleman-induced cash flow freeze, replacing verbal agreements with composable, enforceable on-chain escrow.

## Prerequisites
- Rust (Latest stable toolchain)
- Soroban CLI (`v20.0.0` or latest)
- Target `wasm32-unknown-unknown` installed

## Build Instructions
Compile the smart contract to WebAssembly:
```bash
cargo build --target wasm32-unknown-unknown --release
soroban contract build

CBGHMQ53CMFNVW3CRRRIO3XXE725AVR6WOVU4EUWTSR3GIEP5JHZDILF