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


