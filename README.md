# Pencil Solana

A decentralized structured finance protocol built on Solana, enabling tranched asset pools with senior and junior tranche mechanisms.

## Overview

Pencil Solana is a DeFi protocol that implements a structured finance model with multiple tranches. It allows users to participate in asset pools with different risk-return profiles through senior and junior tranches, providing flexible investment opportunities on the Solana blockchain.

## Features

### Core Functionality

- **System Configuration Management**: Centralized system-wide parameter configuration with multi-level admin roles
- **Asset Pool Creation**: Create and manage asset pools with customizable parameters
- **Dual Tranche Structure**:
  - **Senior Tranche**: Priority returns with fixed interest rates and lower risk
  - **Junior Tranche**: First-loss protection with higher potential returns
- **Fundraising Mechanism**: Subscribe to senior or junior tranches during funding periods
- **Repayment Management**: Structured repayment schedules with interest distribution
- **Token System**:
  - **GROW Token**: Fungible token representing senior tranche positions
  - **Junior NFT**: Non-fungible token representing junior tranche positions
- **Early Exit Options**: Exit positions before maturity with configurable fees

### Key Components

1. **System Config**: Global protocol parameters and admin management
2. **Asset Pools**: Individual structured finance products with specific terms
3. **Funding Accounts**: Handle subscription and fundraising logic
4. **Pools**:
   - Senior Pool (GROW token holders)
   - First Loss Pool (Junior NFT holders)
   - Junior Interest Pool (profit distribution)
5. **Repayment Records**: Track loan repayments and interest distribution

## Architecture

### State Accounts

- `SystemConfig`: Platform-wide configuration and admin roles
- `AssetPool`: Asset pool metadata and parameters
- `Funding`: Fundraising state and subscription tracking
- `Subscription`: Individual user subscription records
- `SeniorPool`: Senior tranche pool state
- `FirstLossPool`: Junior tranche first-loss pool
- `JuniorInterestPool`: Interest distribution for junior tranche
- `RepaymentRecord`: Repayment history tracking
- `JuniorNFTMetadata`: NFT metadata for junior positions

### Instructions

#### System Configuration
- `initialize_system_config`: Initialize platform configuration

#### Asset Pool Management
- `create_asset_pool`: Create a new asset pool
- `approve_asset_pool`: Approve an asset pool for fundraising

#### Fundraising
- `subscribe_senior`: Subscribe to senior tranche
- `subscribe_junior`: Subscribe to junior tranche
- `complete_funding`: Complete the fundraising phase
- `refund_subscription`: Refund subscription if funding fails

#### Repayment
- `repay`: Make repayment to the pool
- `claim_interest`: Claim interest earnings
- `withdraw_principal`: Withdraw principal after maturity
- `early_exit_senior`: Exit senior position early with fees

#### Token Management
- `mint_grow_token`: Mint GROW tokens for senior positions
- `burn_grow_token`: Burn GROW tokens when exiting
- `mint_junior_nft`: Mint NFT for junior positions

## Getting Started

### Prerequisites

- Rust 1.75+
- Solana CLI 1.18+
- Anchor Framework 0.32+
- Node.js 18+
- Yarn

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd pencil-solana
```

2. Install dependencies:
```bash
yarn install
```

3. Build the program:
```bash
anchor build
```

4. Run tests:
```bash
anchor test
```

### Deployment

#### Quick Deployment (Recommended)

Use the automated deployment script (equivalent to the EVM version's `deployToEDUChain.js`):

```bash
# 0. Configure network (required for first deployment)
solana config set --url devnet
# Request airdrop if balance is insufficient
solana airdrop 2

# 1. Build the program
anchor build

# 2. Deploy program to Devnet
anchor deploy --provider.cluster devnet

# 3. Run initialization script (initialize SystemConfig, create sample asset pools, etc.)
# Note: Ensure cluster is set to "devnet" in Anchor.toml
anchor run deploy

# 4. View configuration
anchor run configure
```

After deployment is complete, a `deployment-solana.json` file will be generated containing all deployed account addresses and configuration information.

**Important Notes:**
- Steps 3 and 4 will automatically use the network configured in `Anchor.toml`
- Ensure the `[provider]` section in `Anchor.toml` is configured correctly:
  ```toml
  [provider]
  cluster = "Devnet"  # or "Testnet" / "Mainnet"
  wallet = "~/.config/solana/id.json"
  ```

#### Manual Deployment

1. Configure Solana network:
```bash
solana config set --url devnet
solana airdrop 2  # Get test SOL
```

2. Configure `Anchor.toml`:
```toml
[provider]
cluster = "Devnet"
wallet = "~/.config/solana/id.json"
```

3. Deploy the program:
```bash
anchor deploy --provider.cluster devnet
```

4. Run initialization script:
```bash
ts-node scripts/deploy.ts
```

For detailed deployment instructions, please refer to [scripts/README.md](scripts/README.md)

## Program ID

- **Devnet/Localnet**: `RXo7Ai9ugeBp9giAKqera2pg1xMj49exA5SgWdMBMuM`

## Testing

The test suite covers all major functionalities:

```bash
# Run all tests
anchor test

# Run specific test file
anchor test tests/pencil-solana.ts
```

## Configuration Parameters

### Fee Rates (in basis points, 1 bp = 0.01%)

- **Platform Fee**: Default 500 bp (5%)
- **Senior Early Exit Before Funding End**: Default 100 bp (1%)
- **Senior Early Exit After Funding End**: Default 200 bp (2%)
- **Junior Early Exit Before Funding End**: Default 300 bp (3%)
- **Minimum Junior Ratio**: Default 1000 bp (10%)

## Security Considerations

- All admin actions require proper authorization
- Immutable pool parameters after approval
- Time-locked operations prevent premature actions
- Automated ratio checks ensure pool balance
- Thorough testing with edge cases

## Development

### Project Structure

```
pencil-solana/
├── programs/
│   └── pencil-solana/
│       └── src/
│           ├── lib.rs              # Program entry point
│           ├── state.rs            # Account structures
│           ├── errors.rs           # Custom errors
│           ├── constants.rs        # Constants
│           └── instructions/       # Instruction handlers
│               ├── system_config.rs
│               ├── asset_pool.rs
│               ├── funding.rs
│               ├── repayment.rs
│               └── tokens.rs
├── scripts/                        # Deployment and configuration scripts
│   ├── deploy.ts                   # Main deployment script
│   ├── configure.ts                # Configuration management
│   └── README.md                   # Scripts documentation
├── tests/                          # Integration tests
├── docs/                           # Documentation
├── app/                            # Frontend application
└── migrations/                     # Anchor migrations
```

### Adding New Features

1. Define state structures in `state.rs`
2. Add custom errors in `errors.rs`
3. Implement instruction logic in `instructions/`
4. Export instructions in `lib.rs`
5. Write comprehensive tests
6. Update documentation

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

ISC License

## Resources

- [Anchor Framework Documentation](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [Solana Program Library](https://spl.solana.com/)

## Support

For questions and support, please open an issue in the repository.
