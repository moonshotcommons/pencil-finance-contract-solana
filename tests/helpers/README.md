# Test Helper Modules

This directory contains helper utilities for comprehensive end-to-end testing of the Pencil Solana protocol.

## Modules

### setup.ts
Environment setup and initialization utilities.

**Exports:**
- `TestEnvironment` - Interface containing all test accounts and configuration
- `setupTestEnvironment()` - Creates and initializes all test accounts
- `airdropSol()` - Airdrops SOL to accounts
- `initializeSystemConfig()` - Initializes system configuration with fee rates and admin roles

### token-utils.ts
SPL token creation and management utilities.

**Exports:**
- `createTestToken()` - Creates a test SPL token mint with specified decimals
- `mintTokensTo()` - Mints tokens to a destination account
- `getTokenBalance()` - Gets the token balance of an account
- `createAssociatedTokenAccount()` - Creates an associated token account
- `toTokenAmount()` - Converts human-readable amount to base units
- `fromTokenAmount()` - Converts base units to human-readable amount

### account-utils.ts
Account creation and PDA derivation utilities.

**Exports:**
- `PoolAccounts` - Interface containing all pool-related account addresses
- `derivePoolAccounts()` - Derives all pool-related account PDAs
- `deriveSubscriptionPda()` - Derives subscription PDA for a user
- `deriveNftMetadataPda()` - Derives NFT metadata PDA
- `deriveJuniorNftMintPda()` - Derives junior NFT mint PDA
- `deriveAssetWhitelistPda()` - Derives asset whitelist PDA
- `deriveSystemConfigPda()` - Derives system config PDA

### assertion-utils.ts
Custom assertion helpers and logging utilities.

**Exports:**
- `assertTokenBalance()` - Asserts token account balance
- `assertPoolStatus()` - Asserts asset pool status
- `assertSubscriptionAmount()` - Asserts subscription amount
- `assertApproximately()` - Asserts values are approximately equal within tolerance
- `logBalanceChange()` - Logs balance changes with formatting
- `logTestPhase()` - Logs test phase with emoji
- `logSuccess()` - Logs successful operation
- `logWarning()` - Logs warning
- `logError()` - Logs error
- `logInfo()` - Logs detailed information
- `logTransaction()` - Logs transaction signature
- `logBalances()` - Logs account balances in formatted table
- `logCalculation()` - Logs calculation breakdown for debugging

## Usage

Import all helpers from the index file:

```typescript
import {
  setupTestEnvironment,
  createTestToken,
  derivePoolAccounts,
  assertTokenBalance,
  logTestPhase,
} from "./helpers";
```

Or import from specific modules:

```typescript
import { setupTestEnvironment } from "./helpers/setup";
import { createTestToken } from "./helpers/token-utils";
```

## Example

```typescript
import * as anchor from "@coral-xyz/anchor";
import {
  setupTestEnvironment,
  createTestToken,
  derivePoolAccounts,
  logTestPhase,
  toTokenAmount,
} from "./helpers";

describe("My Test Suite", () => {
  let env;

  before(async () => {
    logTestPhase("Environment Setup", "🚀");

    // Setup test environment
    env = await setupTestEnvironment();

    // Create test tokens
    env.usdtMint = await createTestToken(
      env.provider,
      6,
      "Test USDT",
      "USDT"
    );

    // Initialize system config
    await initializeSystemConfig(env);
  });

  it("Creates a pool", async () => {
    const poolAccounts = await derivePoolAccounts(
      env.program,
      env.borrower1.publicKey,
      "Test Pool",
      env.usdtMint,
      env.treasury.publicKey
    );

    // Use poolAccounts for testing...
  });
});
```

## Requirements Coverage

These helper modules satisfy the following requirements from the spec:

- **Requirement 1.1**: Test environment setup with SPL tokens
- **Requirement 1.2**: Test account generation for all user roles
- **Requirement 1.3**: Token distribution to test accounts
- **Requirement 1.4**: SystemConfig initialization
- **Requirement 2.4**: PDA derivation for pool accounts
- **Requirement 15.1**: Test phase logging with emojis
- **Requirement 15.2**: Balance change logging
- **Requirement 15.3**: Calculation breakdown logging
