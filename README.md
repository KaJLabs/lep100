# LEP100 — Lithosphere Enhancement Proposals (Asset & AI Protocol Stack)

[![Status](https://img.shields.io/badge/status-final-blue.svg)](#)
[![Standards](https://img.shields.io/badge/standards-LEP100-7b2cff.svg)](#)
[![Language](https://img.shields.io/badge/language-Lithic%20(.lithic)-111827.svg)](#)
[![VM](https://img.shields.io/badge/target-LithoVM-0ea5e9.svg)](#)
[![Network](https://img.shields.io/badge/network-Lithosphere-22c55e.svg)](#)
[![Tests](https://img.shields.io/badge/conformance-LEP100%201%E2%80%9314%20suite-f97316.svg)](#conformance)
[![License](https://img.shields.io/badge/license-Apache--2.0-black.svg)](#license)

**Author:** J. King Kasr  
**Organization:** KaJ Labs  
**Language Target:** Lithic (`.lithic`)  
**VM Target:** LithoVM  

---

## Overview

**LEP100** defines the modular smart-contract standards stack for Lithosphere, combining:

- **AI-native execution** (typed services, async fulfillments, receipts, optional zk verification)
- **Assets & NFTs** (NFT, composable NFTs, shared ownership, multi-token)
- **Royalties, metadata, and marketplace hooks**
- **Cross-chain bridge interfaces** (mint/burn + replay protection)
- **Privacy-preserving account linking** (PPAL)

All specifications prioritize deterministic execution, capability-based security, composable interfaces, and conformance testing.

---

## LEP100 Stack

| LEP | Title |
|---:|---|
| **LEP100-1** | Lithic Core Specification |
| **LEP100-2** | AI Service Provider Standard |
| **LEP100-3** | Budget & Cost Accounting Model |
| **LEP100-4** | Provenance Receipt Cryptographic Standard |
| **LEP100-5** | zk-Verifiable AI Execution Extension |
| **LEP100-6** | Non-Fungible Token (NFT) Standard |
| **LEP100-7** | Composable NFT Standard |
| **LEP100-8** | Shared NFT Ownership Standard |
| **LEP100-9** | Multi-Token Standard |
| **LEP100-10** | Royalty Standard |
| **LEP100-11** | Metadata Standard |
| **LEP100-12** | Marketplace Hooks Standard |
| **LEP100-13** | Bridge Mint/Burn Interface |
| **LEP100-14** | Privacy-Preserving Account Linking (PPAL) |

---

## Repository Layout

```text
lep100/
├─ docs/                 # MkDocs-ready documentation
├─ specs/                # RFC-style specs (Markdown)
├─ lscl/                 # Reference modules (Lithic)
├─ tests/                # Conformance tests (Lithic)
└─ rust/                 # Optional Rust workspace (CLI + helpers)
```

---

## Conformance

An implementation is **LEP100-compliant** if it:

1. Implements required ABIs  
2. Emits required events  
3. Enforces required error codes  
4. Passes the official conformance suite (LEP100-1 → LEP100-14)  
5. Preserves deterministic state transitions under LithoVM  

---

# 🪙 LEP100 Token Deployment

## Deploy LEP100 Token

Deploying LEP100 tokens on Lithosphere is conceptually similar to ERC-20 deployment, but with Lithosphere-specific tooling (Lithic + dual Cosmos/EVM support).

[![Network](https://img.shields.io/badge/Network-Lithosphere-00ff88?style=for-the-badge)](https://makalu.litho.ai)
[![Standard](https://img.shields.io/badge/Standard-LEP100-blue?style=for-the-badge)](#)
[![Language](https://img.shields.io/badge/Lithic-Supported-purple?style=for-the-badge)](https://github.com/KaJLabs/Lithic)
[![EVM](https://img.shields.io/badge/EVM-Compatible-green?style=for-the-badge)](#)
[![License](https://img.shields.io/badge/License-MIT-black?style=for-the-badge)](#)

> Deploy LEP100 tokens on the Lithosphere Network (Makalu Testnet & Mainnet)

---

## 🌐 Overview

**LEP100** is the native fungible token standard on Lithosphere, built for:

- ⚡ High-performance execution (~1s blocks)
- 🔗 Cross-chain compatibility (Cosmos + EVM)
- 🤖 Agent-native programmability
- 🧩 Token lifecycle management (minting, vesting, governance)

This repository demonstrates how to deploy LEP100 tokens using:

- **Lithic (native)** — recommended 
- **Solidity (EVM-compatible)** 
- **Litho Finance (no-code UI)** 

---

## ⚙️ Network Configuration

### 🧪 Makalu Testnet

| Parameter        | Value                     |
|----------------|---------------------------|
| RPC Endpoint    | https://rpc.litho.ai     |
| Cosmos Chain ID | lithosphere_777777-1     |
| EVM Chain ID    | 700777                   |
| Explorer        | https://makalu.litho.ai  |

---
### 1️⃣ Lithic Deployment (Recommended)

#### 📦 Install Lithic CLI
```text
git clone https://github.com/KaJLabs/Lithic
cd Lithic
cargo build --release
```

###3 🏗 Initialize Project

```text
lithic new token
cd token
```

#### 🧾 Example LEP100 Contract
```text
contract LEP100Token {

   name: string = "MyToken";
   symbol: string = "MTK";
   decimals: u8 = 18;

   total_supply: u256;
   balances: mapping(address => u256);

   init(initial_supply: u256) {
       total_supply = initial_supply;
       balances[msg.sender] = initial_supply;
   }

   fn transfer(to: address, amount: u256) {
       assert(balances[msg.sender] >= amount);

       balances[msg.sender] -= amount;
       balances[to] += amount;
   }

   fn balance_of(owner: address) -> u256 {
       return balances[owner];
   }
}
```

#### 🔨 Build
lithic build

#### 🚀 Deploy
```text
lithic deploy \
 --network makalu \
 --private-key YOUR_PRIVATE_KEY \
 --args 1000000000000000000000000
```

#### 🔍 Verify
👉 https://makalu.litho.ai

### 2️⃣ Solidity Deployment (EVM)
#### 📦 Install Dependencies
```text
npm install --save-dev hardhat @openzeppelin/contracts
```

#### 🧾 Contract

```text
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract LEP100Token is ERC20 {
   constructor(uint256 initialSupply) ERC20("MyToken", "MTK") {
       _mint(msg.sender, initialSupply);
   }
}
```

### ⚙️ Hardhat Config

```text
module.exports = {
 networks: {
   litho: {
     url: "https://rpc.litho.ai",
     chainId: 700777,
     accounts: [process.env.PRIVATE_KEY]
   }
 },
 solidity: "0.8.20"
};
```

#### 🚀 Deploy
```text
npx hardhat run scripts/deploy.js --network litho
```


### 3️⃣ Litho Finance (No-Code)
Deploy via UI:
Connect wallet
Select Create Token
Choose LEP100
Configure supply + vesting
Click Deploy

### 🤖 Agent Integration
LEP100 tokens are designed for AI agent interaction.
🧠 AgentCraft
Create and Deploy Powerful AI Agents Onchain
Visual workflow builder
Connect LLMs, APIs, databases
Build agents that reason, remember, and act
No-code creation
Automated onchain deployment

### 🧱 Ecosystem
Litho Finance — Token lifecycle platform
Lithic — Smart contract language
LEP100 — Token standard
AgentCraft — AI agent builder

### 🔐 Best Practices
✅ Use multisig wallets
✅ Add vesting contracts
✅ Audit contracts before mainnet
✅ Test on Makalu testnet
✅ Monitor token activity

---

## Credits

Designed and proposed by **J. King Kasr** • Maintained by **KaJ Labs**.

---

## License

Apache-2.0
