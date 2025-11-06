# Design Document

## Overview

This document outlines the technical design for a comprehensive end-to-end test suite for the Pencil Solana protocol. The test suite will mirror the EVM MainFlow.test.js structure while adapting to Solana's account model and transaction patterns.

## Architecture

### Test File Structure

```
tests/
├── main-flow.test.ts          # Main comprehensive test suite
├── helpers/
│   ├── setup.ts               # Test environment setup utilities
│   ├── token-utils.ts         # SPL token creation and management
│   ├── account-utils.ts       # Account creation and PDA derivation
│   └── assertion-utils.ts     # Custom assertion helpers
└── fixtures/
    ├── pool-configs.ts        # Predefined pool configurations
    └── user-profiles.ts       # Test user account configurations
```

### Test Organization

The main test file will be organized into 8 major describe blocks:

1. **Test Environment Setup** - Initialize all accounts, tokens, and system config
2. **Asset Pool Creation and Approval** - Pool lifecycle management
3. **Subscription Phase** - Senior and junior subscriptions with withdrawals
4. **Funding Completion** - Token/NFT distribution
5. **Repayment Phase** - Multiple repayment periods with interest distribution
6. **Redemption Phase** - Interest claims and principal withdrawals
7. **System Pause Scenarios** - Emergency pause functionality
8. **Final State Validation** - Complete system state verification

## Components and Interfaces

### Helper Modules

#### 1. Setup Module (`helpers/setup.ts`)

```typescript
interface TestEnvironment {
  provider: AnchorProvider;
  program: Program<PencilSolana>;
  systemConfig: PublicKey;
  treasury: Keypair;

  // Admin accounts
  superAdmin: Keypair;
  systemAdmin: Keypair;
  operationAdmin: Keypair;
  treasuryAdmin: Keypair;

  // Test tokens
  usdtMint: PublicKey;  // 6 decimals
  usdcMint: PublicKey;  // 9 decimals

  // User accounts
  borrower1: Keypair;
  borrower2: Keypair;
  seniorInvestor1: Keypair;
  seniorInvestor2: Keypair;
  juniorInvestor1: Keypair;
  juniorInvestor2: Keypair;
}

async function setupTestEnvironment(): Promise<TestEnvironment>
async function airdropSol(pubkey: PublicKey, amount: number): Promise<void>
async function initializeSystemConfig(env: TestEnvironment): Promise<void>
```

#### 2. Token Utils Module (`helpers/token-utils.ts`)

```typescript
async function createTestToken(
  provider: AnchorProvider,
  decimals: number,
  name: string,
  symbol: string
): Promise<PublicKey>

async function mintTokensTo(
  provider: AnchorProvider,
  mint: PublicKey,
  destination: PublicKey,
  amount: BN
): Promise<void>

async function getTokenBalance(
  provider: AnchorProvider,
  tokenAccount: PublicKey
): Promise<BN>

async function createAssociatedTokenAccount(
  provider: AnchorProvider,
  mint: PublicKey,
  owner: PublicKey
): Promise<PublicKey>
```

#### 3. Account Utils Module (`helpers/account-utils.ts`)

```typescript
interface PoolAccounts {
  assetPool: PublicKey;
  funding: PublicKey;
  seniorPool: PublicKey;
  firstLossPool: PublicKey;
  juniorInterestPool: PublicKey;
  growTokenMint: PublicKey;
  juniorNftMint: PublicKey;
  assetPoolVault: PublicKey;
  treasuryAta: PublicKey;
}

async function derivePoolAccounts(
  program: Program<PencilSolana>,
  creator: PublicKey,
  poolName: string,
  assetMint: PublicKey
): Promise<PoolAccounts>

async function deriveSubscriptionPda(
  program: Program<PencilSolana>,
  assetPool: PublicKey,
  user: PublicKey,
  tranche: "senior" | "junior"
): Promise<PublicKey>

async function deriveNftMetadataPda(
  program: Program<PencilSolana>,
  assetPool: PublicKey,
  nftId: BN
): Promise<PublicKey>
```

#### 4. Assertion Utils Module (`helpers/assertion-utils.ts`)

```typescript
async function assertTokenBalance(
  provider: AnchorProvider,
  tokenAccount: PublicKey,
  expectedAmount: BN,
  message?: string
): Promise<void>

async function assertPoolStatus(
  program: Program<PencilSolana>,
  assetPool: PublicKey,
  expectedStatus: number
): Promise<void>

async function assertSubscriptionAmount(
  program: Program<PencilSolana>,
  subscription: PublicKey,
  expectedAmount: BN
): Promise<void>

function logBalanceChange(
  label: string,
  before: BN,
  after: BN,
  decimals: number
): void

function logTestPhase(phase: string, emoji: string): void
```

## Data Models

### Test Configuration

```typescript
interface PoolConfig {
  name: string;
  assetMint: PublicKey;
  totalAmount: BN;
  minAmount: BN;
  minJuniorRatio: number;
  repaymentRate: number;
  seniorFixedRate: number;
  repaymentPeriod: BN;
  repaymentCount: BN;
  fundingStartTime: BN;
  fundingEndTime: BN;
}

interface UserAllocation {
  user: Keypair;
  seniorAmount?: BN;
  juniorAmount?: BN;
}
```

### Test State Tracking

```typescript
interface TestState {
  pools: Map<string, PoolAccounts>;
  subscriptions: Map<string, PublicKey>;
  nftIds: Map<string, BN>;
  initialBalances: Map<string, BN>;
  currentPhase: string;
}
```

## Error Handling

### Error Categories

1. **Setup Errors** - Token creation, account initialization failures
2. **Transaction Errors** - Instruction execution failures
3. **Assertion Errors** - Expected vs actual value mismatches
4. **Timing Errors** - Clock manipulation issues

### Error Handling Strategy

```typescript
try {
  // Test operation
  await program.methods.someInstruction().rpc();

  // Verify expected outcome
  const account = await program.account.someAccount.fetch(pda);
  assert.equal(account.value, expectedValue);

} catch (error) {
  if (error instanceof AnchorError) {
    console.error(`❌ Anchor error: ${error.error.errorCode.code}`);
    console.error(`   Message: ${error.error.errorMessage}`);
  } else {
    console.error(`❌ Unexpected error: ${error.message}`);
  }
  throw error;
}
```

## Testing Strategy

### Test Phases

#### Phase 1: Environment Setup (before hook)
- Create test SPL tokens (USDT 6 decimals, USDC 9 decimals)
- Generate all user keypairs
- Airdrop SOL to all accounts
- Initialize SystemConfig
- Set up asset whitelist
- Mint test tokens to users

#### Phase 2: Pool Creation Tests
- Create USDT pool (6 decimals)
- Create USDC pool (9 decimals)
- Test parameter updates before approval
- Approve pools
- Initialize related accounts
- Verify all PDAs created correctly

#### Phase 3: Subscription Tests
- Senior investors subscribe to both pools
- Junior investors subscribe to both pools
- Test subscription limits (senior cap, junior cap)
- Test early withdrawal with fees
- Verify treasury receives fees
- Test concurrent subscriptions

#### Phase 4: Funding Completion Tests
- Advance clock past funding period
- Complete funding for both pools
- Mint GROW tokens to senior investors
- Mint Junior NFTs to junior investors
- Verify token/NFT amounts match subscriptions
- Verify borrower receives funds

#### Phase 5: Repayment Tests
- Make first period repayment
- Verify platform fee to treasury
- Verify senior interest to SeniorPool
- Verify junior interest to JuniorInterestPool
- Verify principal to FirstLossPool
- Test early exit during repayment period
- Test FirstLossPool補足 mechanism
- Complete remaining repayment periods
- Mark pool as Ended

#### Phase 6: Redemption Tests
- Junior investors claim interest
- Verify interest calculations
- Senior investors withdraw (normal exit)
- Junior investors withdraw principal
- Verify all funds distributed correctly
- Check for stuck funds

#### Phase 7: System Pause Tests
- Pause system
- Verify all operations blocked
- Unpause system
- Verify operations resume

#### Phase 8: Final Validation
- Check all pool statuses
- Verify all token balances
- Verify treasury collected all fees
- Generate comprehensive report
- Validate no funds lost

### Test Data

```typescript
const TEST_CONSTANTS = {
  USDT_DECIMALS: 6,
  USDC_DECIMALS: 9,

  POOL_TOTAL_AMOUNT: new BN(1_000_000), // 1M tokens
  POOL_MIN_AMOUNT: new BN(100_000),     // 100K tokens
  MIN_JUNIOR_RATIO: 2000,                // 20%
  REPAYMENT_RATE: 75,                    // 0.75%
  SENIOR_FIXED_RATE: 35,                 // 0.35%
  REPAYMENT_PERIOD: new BN(30),          // 30 days
  REPAYMENT_COUNT: new BN(12),           // 12 periods

  SENIOR_EARLY_BEFORE_FEE: 100,          // 1%
  SENIOR_EARLY_AFTER_FEE: 200,           // 2%
  JUNIOR_EARLY_BEFORE_FEE: 300,          // 3%
  PLATFORM_FEE: 500,                     // 5%
};
```

## Implementation Notes

### Solana-Specific Considerations

1. **Account Size** - Pre-calculate and allocate sufficient space for all accounts
2. **Rent Exemption** - Ensure all accounts are rent-exempt
3. **Transaction Size** - Split large operations into multiple transactions if needed
4. **Compute Units** - Monitor and optimize compute unit usage
5. **Clock Manipulation** - Use `solana-test-validator` clock advancement for time-based tests

### Token Decimal Handling

```typescript
function toTokenAmount(amount: number, decimals: number): BN {
  return new BN(amount * Math.pow(10, decimals));
}

function fromTokenAmount(amount: BN, decimals: number): number {
  return amount.toNumber() / Math.pow(10, decimals);
}
```

### Logging Strategy

```typescript
function logPhase(phase: string) {
  console.log(`\n${"=".repeat(60)}`);
  console.log(`📋 ${phase}`);
  console.log(`${"=".repeat(60)}\n`);
}

function logOperation(operation: string, success: boolean) {
  const icon = success ? "✅" : "❌";
  console.log(`${icon} ${operation}`);
}

function logBalance(label: string, amount: BN, decimals: number) {
  console.log(`   ${label}: ${fromTokenAmount(amount, decimals).toLocaleString()}`);
}
```

## Performance Considerations

1. **Parallel Test Execution** - Use `Promise.all()` for independent operations
2. **Account Caching** - Cache frequently accessed account data
3. **Transaction Batching** - Batch related operations when possible
4. **Selective Logging** - Use environment variables to control log verbosity

## Security Considerations

1. **Test Isolation** - Each test should be independent and not affect others
2. **Cleanup** - Properly close accounts and return rent when possible
3. **Error Propagation** - Ensure errors don't leave system in inconsistent state
4. **Sensitive Data** - Never commit real keypairs or sensitive data

## Future Enhancements

1. **Snapshot Testing** - Save and compare account states
2. **Performance Benchmarking** - Track transaction costs and execution time
3. **Fuzz Testing** - Generate random valid inputs to test edge cases
4. **Coverage Reporting** - Track instruction and branch coverage
5. **Visual Reports** - Generate HTML reports with charts and graphs
