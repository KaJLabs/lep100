# LEP100-7: Composable NFT Standard

**Status:** Draft  
**Author:** J. King Kasr (KaJ Labs)  
**Category:** Standards Track  
**Language Target:** Lithic (.lithic)  
**VM Target:** LithoVM  

---

## Abstract

LEP100-7 defines a required interface and behavior set for Lithosphere interoperability and conformance testing.

## Motivation

Standardization ensures contracts, tooling, indexers, bridges, and AI providers interoperate safely under deterministic consensus.

## Specification (Summary)

- Parent NFT owns child assets.
- Deposit/withdraw semantics, event indexing.
- Optional child enumeration requirements.

## Conformance

Implementations MUST pass the official conformance suite under `tests/lep100-tests/`.

## Reference Implementation

See `lscl/` for canonical modules.

---

© Lithosphere • KaJ Labs
