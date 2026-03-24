# GovernanceTokenPlatform

## Project Descriptionnnsss
Governance Token Platform is a basic Soroban smart contract built on Stellar for managing community proposals, token-based voting, and proposal execution. It provides a simple on-chain governance flow where token holders can participate in decision-making.

## What it does
This contract lets an admin initialize the platform, mint governance tokens, transfer balances, create proposals, vote on proposals, and execute proposals after the voting period ends.

## Features
- Admin initialization
- Governance token minting
- Token transfers
- Proposal creation
- Yes / No voting
- Voting power based on token balance
- Proposal execution after voting ends
- On-chain balance and proposal lookup

## Deployed Smart Contract Link
Governance Token Platform: Transaction dc07a485fa5ff044ec1eba6ed3afe2fa7b3a55dd02aa27fcf532cf3db8b19dad
https://stellar.expert/explorer/testnet/tx/7163541593264128#7163541593264129
<img width="1902" height="875" alt="image" src="https://github.com/user-attachments/assets/d345513c-8e5b-419d-af13-9addfae2874c" />




Summary

## Basic Usage
1. Deploy the contract to Stellar using Soroban CLI.
2. Initialize it with the admin address.
3. Mint governance tokens to eligible voters.
4. Create proposals.
5. Cast votes.
6. Execute the proposal after the deadline.

## Notes
This is a starter template and is not production-ready. For a real governance system, add proposal snapshots, quorum rules, timelocks, delegation, and stronger role controls.
