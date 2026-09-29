# Layer‑1 Core Contract Interface Specification

Explains how CosmaChain-Smart-Contracts submit calls to CosmaCore as SCSAction transactions and how state roots are anchored to Layer-0.

## Overview
Layer‑1 Core interacts with Smart‑Contracts through a unified interface. This interface ensures:

- deterministic execution
- safe state transitions
- compatibility with S3E2C3
- compatibility with CosmaCoin utility

---

## Interface Components

### 1. Transaction Submission
Layer‑1 Core submits contract calls via:

CosmaCoreClient::submit_tx()
Parameters:
- `TxType::SCS` or domain type
- sender
- payload (ABI encoded)

---

### 2. State Access
Contracts read/write state through:

ContractState::get()
ContractState::set()

Layer‑1 Core may query contract state for:
- balances
- SCS phases
- benefits
- payouts
- identity flags

---

### 3. Event Emission
Contracts emit events via:

emit_event(ContractEvent)

Layer‑1 Core listens for:
- SCSPhaseChanged
- LoyaltyReward
- IdentityVerified
- BalanceUpdated

---

### 4. Execution Flow
1. TxPool receives contract tx.
2. Execution Engine validates tx.
3. ContractEngine executes tx.
4. StateManager updates global state.
5. Consensus Engine finalizes block.
6. L0Bridge anchors state root.

---

## Design Goals

- Clean separation of concerns.
- Minimal interface surface.
- High auditability.
- Full compatibility with S3E2C3.


