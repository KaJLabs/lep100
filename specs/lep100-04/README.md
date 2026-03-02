# LEP100-4: Provenance Receipt Cryptographic Standard

**Status:** Draft  
**Author:** J. King Kasr (KaJ Labs)  
**Category:** Standards Track  
**Language Target:** Lithic (.lithic)  
**VM Target:** LithoVM  

---

## Abstract

LEP100-4 defines a required interface and behavior set for Lithosphere interoperability and conformance testing.

## Motivation

Standardization ensures contracts, tooling, indexers, bridges, and AI providers interoperate safely under deterministic consensus.

## Specification (Summary)

- Receipt hashing domain separation.
- Signature verification binding: model/input/output hashes + cost + timestamp.

## Conformance

Implementations MUST pass the official conformance suite under `tests/lep100-tests/`.

## Reference Implementation

See `lscl/` for canonical modules.

---

© Lithosphere • KaJ Labs
