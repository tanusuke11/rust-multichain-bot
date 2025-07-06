# Rust Multichain Bot

A sophisticated multichain arbitrage and liquidation bot system built with Rust, TypeScript, and modern web technologies.

## Project Structure

This project follows a modular architecture with clear separation of concerns:

### Frontend (`frontend/`)
- UI built with Astro, React, Tailwind CSS, and shadcn/ui
- Modern, responsive interface for bot management and monitoring

### Runtime Components (`runtime/`)

#### 1. Master API (`1_master/`)
- TypeScript-based API server with Express
- Database management with SQLite (upgradeable to cloud DB)
- HTTP routing and business logic layer

#### 2. Worker Bot (`2_worker/`)
- Rust-based strategy execution engine
- Tokio async runtime for high-performance trading
- Modular chain integrations (Uniswap, Hyperliquid, etc.)

#### 3. Proxy (`3_proxy/`)
- Cloudflare Workers-based API proxy
- Handles external API requests with token transparency
- CORS support for frontend integration

#### 4. Seeker (`4_seeker/`)
- ML analysis and data collection module
- Rust implementation with Python integration via pyo3
- Jupyter notebooks for research and analysis

### Core Libraries

#### Chain Abstraction (`@chain/`)
- Unified interface for different blockchain networks
- EVM and HyperEVM support
- RPC management and contract interaction

#### Strategy Layer (`@strategy/`)
- Atomic arbitrage implementation
- Liquidation bot strategies
- Common trading utilities

#### Environment Management (`@env/`)
- Local simulation environment
- Cloud deployment configurations
- Environment switching logic

### Smart Contracts (`web3/`)
- Solidity and Move contract definitions
- Uniswap and Hyperliquid integrations

## Getting Started

1. Clone the repository
2. Install dependencies for each component
3. Configure environment variables
4. Use `just dev` to start the development environment

## Commands

Use the provided Justfile for common operations:

- `just build` - Build all components
- `just test` - Run tests
- `just dev` - Start development environment
- `just deploy` - Deploy to production
- `just clean` - Clean build artifacts

## License

[Add your license here]
