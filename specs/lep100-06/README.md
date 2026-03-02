# LEP100-6: Non-Fungible Token (NFT) Standard

**Status:** Draft  
**Author:** J. King Kasr (KaJ Labs)  
**Category:** Standards Track  
**Language Target:** Lithic (.lithic)  
**VM Target:** LithoVM  

---

## Abstract

LEP100-6 defines a required interface and behavior set for Lithosphere interoperability and conformance testing.

## Motivation

Standardization ensures contracts, tooling, indexers, bridges, and AI providers interoperate safely under deterministic consensus.

## Specification (Summary)

- NFT ownership, approvals, transfers.
- Required events: Transfer/Approval/ApprovalForAll.
- Error codes for missing tokens and unauthorized ops.

## Conformance

Implementations MUST pass the official conformance suite under `tests/lep100-tests/`.

## Reference Implementation

See `lscl/` for canonical modules.

---

© Lithosphere • KaJ Labs
