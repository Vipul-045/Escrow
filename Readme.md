# Escrow-Anchor

A Solana Anchor-based program implementing a token escrow system.

## Overview

Escrow-Anchor is a smart contract (on Solana, using the Anchor framework) that facilitates a trustless escrow between two parties exchanging different tokens. It allows a "maker" to deposit tokens into escrow, specify what they want in return, and a "taker" to fulfill the other side of the trade. The contract manages the funds and ensures that tokens are exchanged atomically or refunded if the deal is not completed.

## Features

- **Initialize Escrow**: The maker deposits token A into escrow, specifying the amount and the token B they wish to receive.
- **Claim Escrow**: The taker deposits token B; the contract swaps tokens between parties, ensuring both sides are fully satisfied or the transaction fails.
- **Cancel Escrow**: If the deal is not finalized, the maker can refund their tokens from escrow.

### Main Data Structure

```rust
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amt: u64,
    pub bump: u8,
}
```

## Usage

### Clone the Repo

```bash
https://github.com/Vipul-045/Escrow.git
cd Escrow
```

### Install Dependencies

```bash
yarn install
```

### Build the Project

```bash
anchor build
```

### Test the Project

```bash
anchor test
```

## Example Flow

- **Airdrop and mint tokens** for maker and taker accounts.
- Initialize_escrow creates escrow by calling 'initialize_escrow', funds are deposited.
- Claim_escrow fulfills the deal by calling `claim_escrow`, token swap occurs.
- If no taker, maker can call `cancel_escrow` to reclaim their tokens.

## Key Files

- `programs/escrow-anchor/src/lib.rs`: Anchor program logic and instruction definitions.
- `programs/escrow-anchor/src/instructions/`: Individual instruction handlers (`initialize_escrow`, `claim_escrow`, `cancel_escrow`).
- `programs/escrow-anchor/src/state.rs`: Escrow account state.
- `tests/escrow.ts`: Integration tests covering end-to-end flows.
