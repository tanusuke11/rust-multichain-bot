# GitHub Copilot Instructions for Rust Multichain Bot

## 🔒 Security Configuration

**CRITICAL**: This project contains sensitive environment variables, API keys, and blockchain-related secrets.

### Files Excluded from Copilot Training

The following files are configured to NOT be used for Copilot training via both `.gitattributes` and `.copilotignore`:

#### Environment Files

- ❌ `.env` files (all locations) - Contains API keys and secrets
- ❌ `.env.*` files (local, prod, dev variants)
- ✅ `.env.example` files - Safe templates without sensitive data

#### Archive Directory

- ❌ `.archive/` directory and all contents - May contain historical sensitive data

#### Sensitive Configuration Files

- ❌ Private keys (`*.key`, `*.pem`)
- ❌ Wallet files (`wallet.json`, `keystore/`, `private_keys/`)
- ❌ Secrets files (`secrets.toml`, `secrets.json`)
- ❌ Blockchain configs (`rpc_config.json`, `mainnet.json`, etc.)
- ✅ General configuration files (`config.toml`, `config.json`) - Allowed for training

### Dual Protection Approach

1. **`.gitattributes`** - Uses `linguist-generated=true` to mark files as excluded
2. **`.copilotignore`** - Provides additional protection for Copilot indexing

## 📋 Project Overview

Rust-based multichain arbitrage bot with Li.Fi API integration for cross-chain routing and atomic arbitrage strategies.

### Project Structure

```
runtime/
├── master/          # Master coordination service (TypeScript)
├── proxy/           # Cloudflare Worker proxy
├── seeker/          # Market analysis service (Rust)
└── worker/          # Main arbitrage bot (Rust) ⭐
```

### Key Technologies

- **Rust**: Core arbitrage logic, Li.Fi integration
- **Li.Fi API**: Cross-chain routing and transaction building
- **Docker**: Containerization
- **PostgreSQL**: Transaction tracking

## 🔧 Development Guidelines

1. **Never expose sensitive data** in code or examples
2. **Use `.env.example`** for documenting required environment variables
3. **Test API integrations** in isolated environments
4. **Follow Rust best practices** for error handling and async programming

## 🚀 Li.Fi Integration Status

✅ Route discovery and parsing working correctly  
✅ Transaction building functional  
✅ Error handling and debugging implemented  
✅ Security configurations in place
