# Requirements Document

## Introduction

This specification defines the requirements for building a comprehensive end-to-end test suite for the Solana version of the Pencil protocol. The test suite SHALL mirror the functionality and coverage of the EVM MainFlow.test.js, ensuring complete business cycle validation from asset pool creation through final redemption.

## Glossary

- **System**: The Pencil Solana program
- **Asset Pool**: A lending pool account that manages fundraising and repayments
- **Senior Investor**: A user who subscribes to the senior tranche and receives GROW tokens
- **Junior Investor**: A user who subscribes to the junior tranche and receives NFTs
- **GROW Token**: SPL token representing senior tranche investment
- **Junior NFT**: NFT representing junior tranche investment with principal amount
- **Funding Period**: Time window between funding_start_time and funding_end_time
- **Repayment Period**: Duration of each repayment installment (in days)
- **Test Suite**: The comprehensive test file covering all business scenarios

## Requirements

### Requirement 1: Test Environment Setup

**User Story:** As a test developer, I want to set up a complete test environment with all necessary accounts and tokens, so that I can execute comprehensive integration tests.

#### Acceptance Criteria

1. WHEN the test suite initializes, THE System SHALL create test SPL token mints with configurable decimals
2. WHEN test accounts are created, THE System SHALL generate keypairs for all user roles (super admin, system admin, operation admin, treasury admin, borrowers, senior investors, junior investors)
3. WHEN token distribution occurs, THE System SHALL mint sufficient test tokens to all user accounts for testing
4. WHEN system configuration is initialized, THE System SHALL set up SystemConfig with all fee rates and admin roles
5. WHERE multiple asset pools are needed, THE System SHALL support creating pools with different token decimals (6 and 9 decimals)

### Requirement 2: Asset Pool Creation and Approval

**User Story:** As a borrower, I want to create and configure asset pools with specific parameters, so that I can raise funds for my lending needs.

#### Acceptance Criteria

1. WHEN a borrower creates an asset pool, THE System SHALL store all pool parameters (name, amounts, rates, periods, timestamps)
2. WHEN pool parameters need modification, THE System SHALL allow updates before approval
3. WHEN an operation admin approves a pool, THE System SHALL transition the pool status to Approved
4. WHEN related accounts are initialized, THE System SHALL create and link Funding, SeniorPool, FirstLossPool, JuniorInterestPool, GROW token mint, and Junior NFT mint accounts
5. WHERE asset whitelist validation is required, THE System SHALL verify the asset is supported before pool creation

### Requirement 3: Subscription Management

**User Story:** As an investor, I want to subscribe to senior or junior tranches during the funding period, so that I can participate in the lending pool.

#### Acceptance Criteria

1. WHEN a senior investor subscribes, THE System SHALL transfer tokens to the pool vault and record the subscription amount
2. WHEN a junior investor subscribes, THE System SHALL transfer tokens to the pool vault and record the subscription amount
3. WHEN subscription limits are reached, THE System SHALL reject subscriptions exceeding senior or junior caps
4. WHEN an investor exits before funding completion, THE System SHALL calculate and deduct early exit fees
5. WHERE funding period has not started, THE System SHALL reject subscription attempts
6. WHERE funding period has ended, THE System SHALL reject new subscription attempts

### Requirement 4: Subscription Withdrawal and Fees

**User Story:** As an investor, I want to withdraw my subscription before funding completes, so that I can exit if I change my mind.

#### Acceptance Criteria

1. WHEN a senior investor withdraws subscription, THE System SHALL calculate fee using senior_early_before_exit_fee_rate
2. WHEN a junior investor withdraws subscription, THE System SHALL calculate fee using junior_early_before_exit_fee_rate
3. WHEN withdrawal fee is calculated, THE System SHALL transfer the fee amount to treasury
4. WHEN withdrawal refund is processed, THE System SHALL transfer net amount (subscription minus fee) to user
5. WHERE subscription amount is insufficient, THE System SHALL reject withdrawal requests exceeding subscribed amount

### Requirement 5: Funding Completion and Token Distribution

**User Story:** As a pool admin, I want to complete funding and distribute tokens to investors, so that the lending pool can begin operations.

#### Acceptance Criteria

1. WHEN funding is completed, THE System SHALL transfer all raised funds to the borrower
2. WHEN GROW tokens are minted, THE System SHALL mint tokens proportional to each senior investor's subscription
3. WHEN Junior NFTs are minted, THE System SHALL create NFTs with principal amounts matching each junior investor's subscription
4. WHEN funding target is not met, THE System SHALL allow cancellation and refund processing
5. WHERE funding period has not ended, THE System SHALL reject complete_funding attempts

### Requirement 6: Repayment Processing

**User Story:** As a borrower, I want to make periodic repayments, so that I can fulfill my loan obligations and distribute returns to investors.

#### Acceptance Criteria

1. WHEN a repayment is made, THE System SHALL calculate and transfer platform fee to treasury
2. WHEN senior allocation is calculated, THE System SHALL transfer senior fixed rate amount to SeniorPool
3. WHEN junior allocation is calculated, THE System SHALL transfer remaining interest to JuniorInterestPool
4. WHEN principal is repaid, THE System SHALL transfer principal portion to FirstLossPool
5. WHERE repayment amount is insufficient, THE System SHALL reject the repayment transaction
6. WHERE all repayments are complete, THE System SHALL allow pool status transition to Ended

### Requirement 7: Senior Early Exit

**User Story:** As a senior investor, I want to exit my position early, so that I can access my funds before the pool ends.

#### Acceptance Criteria

1. WHEN early exit occurs before funding_end_time, THE System SHALL apply senior_early_before_exit_fee_rate
2. WHEN early exit occurs after funding_end_time, THE System SHALL apply senior_early_after_exit_fee_rate
3. WHEN GROW tokens are burned, THE System SHALL calculate redemption value based on current pool state
4. WHEN vault balance is insufficient, THE System SHALL request funds from FirstLossPool to cover shortfall
5. WHERE user has insufficient GROW tokens, THE System SHALL reject early exit requests

### Requirement 8: Junior Interest Claiming

**User Story:** As a junior investor, I want to claim accumulated interest, so that I can receive my investment returns.

#### Acceptance Criteria

1. WHEN interest is claimed, THE System SHALL calculate proportional share based on NFT principal amount
2. WHEN interest calculation is performed, THE System SHALL use formula: (total_interest × nft_principal) / junior_total_principal - claimed_interest
3. WHEN interest is transferred, THE System SHALL update JuniorInterestPool.distributed_interest
4. WHEN claim is recorded, THE System SHALL update JuniorNFTMetadata.claimed_interest
5. WHERE no claimable interest exists, THE System SHALL reject claim attempts

### Requirement 9: Junior Principal Withdrawal

**User Story:** As a junior investor, I want to withdraw my principal after the pool ends, so that I can recover my initial investment.

#### Acceptance Criteria

1. WHEN principal withdrawal occurs, THE System SHALL verify pool status is Ended
2. WHEN NFT ownership is verified, THE System SHALL confirm user owns the NFT
3. WHEN principal is transferred, THE System SHALL move funds from FirstLossPool to user
4. WHEN withdrawal is recorded, THE System SHALL set principal_withdrawn flag to true
5. WHERE principal was already withdrawn, THE System SHALL reject duplicate withdrawal attempts

### Requirement 10: System Pause Functionality

**User Story:** As a system admin, I want to pause system operations, so that I can halt activity during emergencies or maintenance.

#### Acceptance Criteria

1. WHEN system is paused, THE System SHALL reject pool creation attempts
2. WHEN system is paused, THE System SHALL reject subscription attempts
3. WHEN system is paused, THE System SHALL reject repayment attempts
4. WHEN system is paused, THE System SHALL reject early exit attempts
5. WHEN system is paused, THE System SHALL reject withdrawal and claim attempts

### Requirement 11: Multi-Asset Pool Support

**User Story:** As a test developer, I want to test pools with different token decimals, so that I can verify the system handles various SPL token configurations.

#### Acceptance Criteria

1. WHEN pools use 6-decimal tokens, THE System SHALL correctly handle all calculations and transfers
2. WHEN pools use 9-decimal tokens, THE System SHALL correctly handle all calculations and transfers
3. WHEN multiple pools exist simultaneously, THE System SHALL maintain separate state for each pool
4. WHERE token precision differs, THE System SHALL prevent precision loss in calculations

### Requirement 12: Complete Business Cycle Validation

**User Story:** As a test developer, I want to execute a complete end-to-end business cycle, so that I can verify all system components work together correctly.

#### Acceptance Criteria

1. WHEN full cycle test executes, THE System SHALL complete all phases: creation, approval, subscription, funding, repayment, and redemption
2. WHEN state transitions occur, THE System SHALL maintain data consistency across all accounts
3. WHEN funds flow through the system, THE System SHALL ensure no tokens are lost or stuck
4. WHEN final balances are checked, THE System SHALL verify all investors received correct amounts
5. WHERE any phase fails, THE System SHALL provide clear error messages for debugging

### Requirement 13: Concurrent Operations Testing

**User Story:** As a test developer, I want to test concurrent user operations, so that I can verify the system handles parallel transactions correctly.

#### Acceptance Criteria

1. WHEN multiple users subscribe simultaneously, THE System SHALL record all subscriptions accurately
2. WHEN multiple users claim interest simultaneously, THE System SHALL prevent double-spending
3. WHEN multiple users withdraw principal simultaneously, THE System SHALL maintain correct pool balances
4. WHERE race conditions could occur, THE System SHALL use proper account locking mechanisms

### Requirement 14: Edge Case and Error Handling

**User Story:** As a test developer, I want to test edge cases and error conditions, so that I can verify the system handles exceptional scenarios gracefully.

#### Acceptance Criteria

1. WHEN invalid parameters are provided, THE System SHALL reject transactions with descriptive errors
2. WHEN timing constraints are violated, THE System SHALL enforce funding and repayment schedules
3. WHEN insufficient balances exist, THE System SHALL reject operations requiring more funds
4. WHEN unauthorized users attempt restricted operations, THE System SHALL reject with permission errors
5. WHERE overflow conditions could occur, THE System SHALL use checked arithmetic to prevent exploits

### Requirement 15: Test Reporting and Logging

**User Story:** As a test developer, I want comprehensive test output and logging, so that I can understand test results and debug failures.

#### Acceptance Criteria

1. WHEN tests execute, THE System SHALL log all major operations with emoji indicators
2. WHEN state changes occur, THE System SHALL display before/after balances and amounts
3. WHEN calculations are performed, THE System SHALL show intermediate values for verification
4. WHEN tests complete, THE System SHALL generate a summary report with pass/fail statistics
5. WHERE tests fail, THE System SHALL provide detailed error information and stack traces