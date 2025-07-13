# Rust Multichain Bot

A sophisticated multichain arbitrage and liquidation bot system built with Rust, TypeScript, and Li.Fi API integration for cross-chain routing and atomic arbitrage strategies.

## 🏗️ Project Architecture

This project follows a modular microservices architecture with dependency injection patterns:

### 🖥️ Frontend (`frontend/`)
- Modern web interface for bot management and monitoring
- Built with modern web technologies

### ⚡ Runtime Components (`runtime/`)

#### Master Service (`master/`)
- **TypeScript-based coordination service**
- PostgreSQL database with schema migrations
- API server for orchestrating bot operations
- Database schema: `db/schema.sql`, `db/migrations/`

#### Worker Bot (`worker/`) ⭐
- **Core Rust arbitrage engine with Li.Fi integration**
- Dependency injection architecture for modular components
- Chain abstraction layer (Ethereum, HyperEVM)
- Strategy implementations (Atomic Arbitrage, Liquidator)
- Environment management (Local, VPS)
- Docker containerized with `--bin worker` execution

#### Proxy Service (`proxy/`)
- **Cloudflare Workers-based API proxy**
- Handles external API requests and CORS
- Token transparency for frontend integration

#### Seeker Service (`seeker/`)
- **Market analysis and data collection**
- Rust-based scraper and analyzer modules
- Jupyter notebooks for research and analysis

### 🔗 Core Architecture Patterns

#### Chain Abstraction (`src/chain/`)
- Unified interface for different blockchain networks
- Secure EVM address derivation with k256 cryptography
- Support for Ethereum and HyperEVM chains
- Primary wallet address management

#### Strategy Layer (`src/strategy/`)
- Dependency injection pattern with Arc<dyn Trait>
- Atomic arbitrage with Li.Fi route optimization
- Liquidation bot strategies
- Modular strategy composition

#### Environment Management (`src/env/`)
- Local development and VPS production environments
- Configuration abstraction
- Environment-specific settings

#### Li.Fi Integration (`src/module/lifi/`)
- Cross-chain route discovery and optimization
- Transaction building with proper address injection
- Error handling and debugging capabilities

### 📁 Smart Contracts (`onchain/`)
- Contract definitions for Uniswap and Hyperliquid
- Integration templates and examples

## 🚀 Getting Started

### Prerequisites
- **Rust 1.85.0+** with cargo
- **Node.js 18+** for TypeScript components
- **Docker** for containerization
- **PostgreSQL** for database (master service)

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/tanusuke11/rust-multichain-bot.git
   cd rust-multichain-bot
   ```

2. **Environment Configuration**
   ```bash
   # Copy example environment files
   cp .env.example .env
   # Configure your API keys and settings
   ```

3. **Build and Run**
   ```bash
   # Build all components
   just build
   
   # Run worker bot locally
   just worker
   
   # Start development environment
   just dev
   ```

### 🐋 Docker Deployment

Worker bot is containerized for production deployment:

```bash
# Build Docker image
docker build -t rust-multichain-worker runtime/worker/

# Run container
docker run --env-file .env rust-multichain-worker
```

## 🔧 Key Technologies

- **Rust**: Core arbitrage logic with async/await patterns
- **Li.Fi API**: Cross-chain routing and transaction building
- **alloy-primitives**: EVM address handling and cryptography
- **k256**: Secure elliptic curve operations
- **TypeScript**: Master service coordination
- **PostgreSQL**: Transaction tracking and state management
- **Docker**: Containerized deployment

## 📊 Architecture Highlights

### Dependency Injection Pattern
- Clean separation of concerns with Arc<dyn Trait> patterns
- Testable and modular component architecture
- Constructor-based dependency injection

### Security Features
- Secure EVM private key handling with k256
- Environment variable protection (.copilotignore, .gitattributes)
- No sensitive data exposure in code

### Li.Fi Integration
- ✅ Route discovery and parsing
- ✅ Cross-chain transaction building
- ✅ Error handling and debugging
- ✅ Address injection via DI

## 📋 Available Commands

Use the provided Justfile for common operations:

```bash
just build        # Build all components
just test         # Run test suites
just worker       # Run worker bot locally
just dev          # Start development environment
just deploy       # Deploy to production
just clean        # Clean build artifacts
```

## 📁 Project Structure

```
runtime/
├── master/          # TypeScript coordination service
│   ├── src/         # API controllers and services
│   └── db/          # Database schema and migrations
├── worker/          # Rust arbitrage bot ⭐
│   ├── src/
│   │   ├── chain/   # Blockchain abstraction
│   │   ├── strategy/ # Trading strategies
│   │   ├── module/  # Li.Fi integration
│   │   └── env/     # Environment management
│   └── Dockerfile   # Container configuration
├── proxy/           # Cloudflare Workers proxy
└── seeker/          # Market analysis service

onchain/             # Smart contract definitions
frontend/            # Web interface
scripts/             # Automation scripts
types/               # Shared type definitions
```

## 🔒 Security Notice

This project contains sensitive API keys and blockchain secrets. Ensure proper:
- Environment variable management
- Private key security
- API key rotation
- Network security for production deployments

## 📝 License

[Add your license here]
