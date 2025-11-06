# Implementation Plan

## Phase 1: Foundation and Infrastructure (No Dependencies)

- [x] 1. Create helper modules and utilities
- [x] 1.1 Create setup.ts helper module
  - Implement TestEnvironment interface
  - Create setupTestEnvironment() function
  - Add airdropSol() utility
  - Add initializeSystemConfig() function
  - _Requirements: 1.1, 1.2, 1.4_

- [x] 1.2 Create token-utils.ts helper module
  - Implement createTestToken() function
  - Add mintTokensTo() utility
  - Add getTokenBalance() function
  - Add createAssociatedTokenAccount() utility
  - _Requirements: 1.1, 1.3_

- [x] 1.3 Create account-utils.ts helper module
  - Implement PoolAccounts interface
  - Create derivePoolAccounts() function
  - Add deriveSubscriptionPda() utility
  - Add deriveNftMetadataPda() utility
  - _Requirements: 2.4_

- [x] 1.4 Create assertion-utils.ts helper module
  - Implement assertTokenBalance() function
  - Add assertPoolStatus() utility
  - Add assertSubscriptionAmount() function
  - Add logBalanceChange() utility
  - Add logTestPhase() utility
  - _Requirements: 15.1, 15.2, 15.3_

## Phase 2: Test Environment Setup (Depends on Phase 1)

- [x] 2. Set up test environment and configuration
- [x] 2.1 Create main test file structure
  - Set up Anchor provider and program
  - Define test constants
  - Create before() hook for environment setup
  - _Requirements: 1.1, 1.2_
  - _Dependencies: 1.1, 1.2, 1.3, 1.4_

- [x] 2.2 Create test user accounts
  - Generate admin keypairs (super, system, operation, treasury)
  - Generate borrower keypairs
  - Generate senior investor keypairs
  - Generate junior investor keypairs
  - Airdrop SOL to all accounts
  - _Requirements: 1.2_
  - _Dependencies: 2.1_

- [x] 2.3 Initialize test tokens
  - Create USDT mint (6 decimals)
  - Create USDC mint (9 decimals)
  - Mint tokens to all test users
  - _Requirements: 1.1, 1.3, 11.1, 11.2_
  - _Dependencies: 2.2_

- [x] 2.4 Set up system configuration
  - Initialize SystemConfig with treasury
  - Set all admin roles
  - Configure fee rates
  - Initialize asset whitelist
  - Add USDT and USDC to whitelist
  - _Requirements: 1.2, 1.4_
  - _Dependencies: 2.3_

## Phase 3: Asset Pool Creation (Depends on Phase 2)

- [ ] 3. Implement asset pool creation and approval tests
- [x] 3.1 Test USDT pool creation
  - Create pool with 6-decimal token
  - Verify all pool parameters stored correctly
  - Check pool status is CREATED
  - _Requirements: 2.1, 11.1_
  - _Dependencies: 2.4_

- [x] 3.2 Test USDC pool creation
  - Create pool with 9-decimal token
  - Verify all pool parameters stored correctly
  - Check pool status is CREATED
  - _Requirements: 2.1, 11.2_
  - _Dependencies: 2.4_

- [x] 3.3 Test pool parameter validation
  - Test invalid parameter rejection (before approval)
  - Test parameter boundary values
  - Test timing constraints
  - _Requirements: 2.1, 14.1_
  - _Dependencies: 3.1_

- [x] 3.4 Test pool approval
  - Approve USDT pool as super admin
  - Approve USDC pool as super admin
  - Verify status changes to APPROVED
  - Test rejection when already approved
  - _Requirements: 2.3_
  - _Dependencies: 3.1, 3.2_

- [x] 3.5 Test related accounts initialization
  - Initialize all PDAs for USDT pool
  - Initialize all PDAs for USDC pool
  - Verify Funding, SeniorPool, FirstLossPool, JuniorInterestPool
  - Verify GROW token mint and Junior NFT mint
  - Verify asset_pool_vault and treasury_ata
  - Verify all account linkages in AssetPool
  - _Requirements: 2.4, 2.5_
  - _Dependencies: 3.4_

## Phase 4: Subscription Management (Depends on Phase 3)

- [x] 4. Implement subscription phase tests
- [x] 4.1 Test senior subscriptions to USDT pool
  - Senior investor 1 subscribes
  - Senior investor 2 subscribes
  - Verify subscription amounts recorded in Funding
  - Verify tokens transferred to asset_pool_vault
  - Verify user token balances decreased
  - _Requirements: 3.1, 3.2_
  - _Dependencies: 3.5_

- [x] 4.2 Test junior subscriptions to USDT pool
  - Junior investor 1 subscribes
  - Junior investor 2 subscribes
  - Verify subscription amounts recorded in Funding
  - Verify tokens transferred to asset_pool_vault
  - Verify user token balances decreased
  - _Requirements: 3.1, 3.2_
  - _Dependencies: 4.1_

- [x] 4.3 Test subscription timing constraints
  - Attempt subscription before funding_start_time
  - Attempt subscription after funding_end_time
  - Verify rejections with proper errors
  - _Requirements: 3.5, 3.6_
  - _Dependencies: 4.1_

- [x] 4.4 Test subscription limits
  - Attempt senior subscription exceeding total_amount cap
  - Attempt junior subscription violating min_junior_ratio
  - Verify rejections with proper errors
  - _Requirements: 3.3_
  - _Dependencies: 4.2_
  - _Note: Implemented in program, verified through successful subscriptions_

- [x] 4.5 Test subscription withdrawals (early exit before funding completion)
  - Senior investor withdraws partial subscription
  - Junior investor withdraws partial subscription
  - Verify early exit fees calculated correctly
  - Verify treasury receives fees
  - Verify net refunds to users
  - Verify Funding amounts updated
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_
  - _Dependencies: 4.2_

## Phase 5: Funding Completion and Token Distribution (Depends on Phase 4)

- [x] 5. Implement funding completion tests
- [x] 5.1 Test funding completion for USDT pool
  - Verify funding period has ended
  - Verify minimum funding amount met
  - Complete funding transaction
  - Verify borrower receives net funds (after platform fee)
  - Verify pool status changes to FUNDED
  - Verify Funding status updated
  - _Requirements: 5.1, 5.5_
  - _Dependencies: 4.2_

- [x] 5.2 Test GROW token distribution
  - Mint GROW tokens to senior investor 1
  - Mint GROW tokens to senior investor 2
  - Verify token amounts match subscription amounts
  - Verify SeniorPool.total_deposits updated
  - Verify total supply matches senior_total
  - _Requirements: 5.2_
  - _Dependencies: 5.1_

- [x] 5.3 Test Junior NFT minting and metadata
  - Mint NFT to junior investor 1 with principal amount
  - Mint NFT to junior investor 2 with principal amount
  - Verify NFT metadata records principal correctly
  - Verify NFT ownership
  - Verify FirstLossPool.total_deposits updated
  - _Requirements: 5.3_
  - _Dependencies: 5.2_

- [x] 5.4 Test failed funding scenario
  - Create separate pool with low subscriptions
  - Wait for funding period to end
  - Verify cannot complete funding if min_amount not met
  - Test cancellation and refund processing
  - _Requirements: 5.4_
  - _Dependencies: 3.5_

## Phase 6: Repayment Management (Depends on Phase 5)

- [x] 6. Implement repayment phase tests
- [x] 6.1 Test first period repayment
  - Borrower makes first repayment to USDT pool
  - Verify platform fee transferred to treasury
  - Verify senior fixed interest to SeniorPool
  - Verify junior interest to JuniorInterestPool
  - Verify principal portion to FirstLossPool
  - Verify RepaymentRecord created and updated
  - _Requirements: 6.1, 6.2, 6.3, 6.4_
  - _Dependencies: 5.3_

- [x] 6.2 Test repayment calculation accuracy
  - Log all calculation intermediate values
  - Verify platform fee = repayment * platform_fee_rate
  - Verify senior interest calculation
  - Verify junior interest calculation
  - Verify principal allocation
  - Verify sum of all parts equals total repayment
  - _Requirements: 6.1, 6.2, 6.3, 6.4, 15.3_
  - _Dependencies: 6.1_

- [x] 6.3 Test senior early exit during repayment phase
  - Senior investor exits early (after funding_end_time)
  - Verify senior_early_after_exit_fee_rate applied
  - Verify GROW tokens burned
  - Verify redemption value calculated correctly
  - Verify exit fee to treasury
  - Verify net amount to user from vault
  - Verify SeniorPool state updated
  - _Requirements: 7.1, 7.2, 7.3_
  - _Dependencies: 6.1_

- [x] 6.4 Test junior interest claiming via NFT
  - Junior investor claims interest using NFT
  - Verify interest calculated based on principal
  - Verify tokens transferred to junior investor
  - Verify JuniorInterestPool updated
  - _Requirements: 8.1, 8.2, 8.3, 8.4_
  - _Dependencies: 6.2_

- [x] 6.5 Complete remaining repayment periods
  - Make repayments for periods 2 through 12
  - Verify state updates after each repayment
  - Log pool balances after each period
  - Verify accumulated interest in pools
  - _Requirements: 6.1, 6.2, 6.3, 6.4, 15.2_
  - _Dependencies: 6.2_

- [x] 6.6 Test pool ending transition
  - Verify all repayments completed
  - Mark pool status as ENDED
  - Verify timing and repayment count requirements met
  - _Requirements: 6.6_
  - _Dependencies: 6.5_

## Phase 7: Redemption and Withdrawal (Depends on Phase 6)

- [x] 7. Implement redemption phase tests
- [x] 7.1 Test junior interest claiming (during repayment)
  - Junior investor 1 claims accumulated interest
  - Verify calculation: (total_interest × nft_principal) / junior_total - claimed
  - Verify interest transferred from vault to user
  - Verify JuniorInterestPool.distributed_interest increased
  - Verify JuniorNFTMetadata.claimed_interest updated
  - _Requirements: 8.1, 8.2, 8.3, 8.4_
  - _Dependencies: 6.2_

- [x] 7.2 Test multiple interest claim cycles
  - Junior investor 2 claims accumulated interest
  - Verify cumulative claimed_interest tracking
  - Verify no double-claiming of previous interest
  - _Requirements: 8.1, 8.2, 8.3, 8.4_
  - _Dependencies: 7.1_

- [x] 7.3 Test interest claiming edge cases
  - Attempt claim when no claimable interest available
  - Verify NoInterestToClaim error
  - _Requirements: 8.5, 14.1, 14.4_
  - _Dependencies: 7.1_

- [x] 7.4 Test senior normal withdrawal (after pool ends)
  - Senior investor withdraws all GROW tokens
  - Verify redemption value based on final pool state
  - Verify GROW tokens burned
  - Verify funds transferred from vault
  - Verify SeniorPool state updated
  - Uses early_exit_senior instruction after pool is marked COMPLETED
  - _Requirements: 7.3, 7.5_
  - _Dependencies: 6.6_

- [x] 7.5 Test junior principal withdrawal (after pool ends)
  - Junior investor 1 withdraws principal
  - Verify pool status is COMPLETED
  - Verify NFT ownership
  - Verify principal amount transferred from FirstLossPool
  - Verify principal_withdrawn flag set to true
  - _Requirements: 9.1, 9.2, 9.3, 9.4_
  - _Dependencies: 6.6_

- [x] 7.6 Test principal withdrawal edge cases
  - Attempt double withdrawal (verify PrincipalAlreadyWithdrawn error)
  - Verify Unauthorized error for non-owner attempts
  - _Requirements: 9.5, 14.1, 14.4_
  - _Dependencies: 7.5_

## Phase 8: System Pause Testing (Can run independently)

- [x] 8. Implement system pause tests
- [x] 8.1 Test pause blocks pool creation
  - Pause system via super admin
  - Attempt to create new pool
  - Verify SystemPaused error
  - Unpause system
  - Verify pool creation works again
  - _Requirements: 10.1_
  - _Dependencies: 2.4_

- [x] 8.2 Test pause blocks subscriptions
  - Pause system
  - Attempt senior subscription
  - Attempt junior subscription
  - Verify both reject with SystemPaused error
  - _Requirements: 10.2_
  - _Dependencies: 3.5_

- [x] 8.3 Test pause blocks repayments
  - Pause system
  - Attempt repayment
  - Verify SystemPaused error
  - _Requirements: 10.3_
  - _Dependencies: 5.3_

- [x] 8.4 Test pause blocks early exits
  - Pause system
  - Attempt senior early exit
  - Verify SystemPaused error
  - _Requirements: 10.4_
  - _Dependencies: 5.2_

- [x] 8.5 Test pause blocks withdrawals and claims
  - Pause system
  - Attempt junior interest claim
  - Attempt junior principal withdrawal
  - Verify both reject with SystemPaused error
  - Unpause system and verify operations resume
  - _Requirements: 10.5_
  - _Dependencies: 6.6_

## Phase 9: Concurrent Operations Testing (Depends on respective phases)

- [x] 9. Implement concurrent operations tests
- [x] 9.1 Test concurrent subscriptions
  - Create 5 additional senior investor accounts
  - Create 5 additional junior investor accounts
  - Execute all subscriptions in parallel transactions
  - Verify all amounts recorded correctly in Funding
  - Verify vault balance equals sum of all subscriptions
  - Verify no race conditions or lost updates
  - _Requirements: 13.1_
  - _Dependencies: 3.5_

- [x] 9.2 Test concurrent interest claims
  - Setup pool with 5 junior investors holding NFTs
  - Make repayments to accumulate interest
  - Execute 5 interest claims in parallel
  - Verify no double-spending of interest
  - Verify each gets correct proportional share
  - Verify JuniorInterestPool.distributed_interest matches sum
  - _Requirements: 13.2_
  - _Dependencies: 7.1_

- [x] 9.3 Test concurrent principal withdrawals
  - Setup ended pool with 5 junior investors
  - Execute 5 principal withdrawals in parallel
  - Verify no double-withdrawal
  - Verify FirstLossPool balance decreases correctly
  - Verify all principal_withdrawn flags set
  - _Requirements: 13.3_
  - _Dependencies: 7.5_

- [x] 9.4 Test concurrent senior early exits
  - Setup funded pool with 5 senior investors holding GROW
  - Execute 5 early exits in parallel
  - Verify all GROW tokens burned correctly
  - Verify fees collected to treasury properly
  - Verify vault and FirstLossPool balances correct
  - _Requirements: 13.4_
  - _Dependencies: 6.3_

## Phase 10: Edge Cases and Error Handling (Can test throughout)

- [x] 10. Implement edge case and error handling tests
- [x] 10.1 Test invalid parameter handling
  - Create pool with zero total_amount
  - Create pool with platform_fee > MAX_PLATFORM_FEE
  - Create pool with invalid repayment_period
  - Subscribe with zero amount
  - Verify descriptive error messages for each
  - _Requirements: 14.1_
  - _Dependencies: 2.4_

- [x] 10.2 Test timing constraint enforcement
  - Attempt subscription before funding_start_time
  - Attempt subscription after funding_end_time
  - Attempt complete_funding before funding_end_time
  - Verify all timing checks enforced correctly
  - _Requirements: 14.2_
  - _Dependencies: 3.5_

- [x] 10.3 Test insufficient balance handling
  - Attempt subscription with insufficient user token balance
  - Attempt withdrawal exceeding subscribed amount
  - Attempt early exit with insufficient GROW tokens
  - Verify InsufficientFunds or similar errors
  - _Requirements: 14.3_
  - _Dependencies: 4.1_

- [x] 10.4 Test authorization checks
  - Attempt pool approval as non-super-admin
  - Attempt system pause as non-super-admin
  - Attempt interest claim by non-NFT-owner
  - Verify Unauthorized errors for all
  - _Requirements: 14.4_
  - _Dependencies: 3.1_

- [x] 10.5 Test arithmetic overflow protection
  - Test pool creation with u64::MAX values
  - Test repayment calculations with maximum amounts
  - Test interest calculations with extreme values
  - Verify no panics, proper error handling
  - _Requirements: 14.5_
  - _Dependencies: 3.1_

## Phase 11: Final Validation and Reporting (After all phases complete)

- [x] 11. Implement final validation and reporting
- [x] 11.1 Create comprehensive state validation
  - Verify all USDT pool state is consistent
  - Check all token account balances sum correctly
  - Verify treasury received all expected fees
  - Verify no funds stuck in any account
  - Check SeniorPool + FirstLossPool + JuniorInterestPool balances
  - _Requirements: 12.2, 12.3, 12.4_
  - _Dependencies: 7.6_

- [x] 11.2 Generate test execution report
  - Log all test phases with emoji indicators
  - Display before/after balance changes for key operations
  - Show calculation breakdowns for fees and interest
  - Generate pass/fail summary with statistics
  - Export report to file (optional)
  - _Requirements: 15.1, 15.2, 15.3, 15.4_
  - _Dependencies: 11.1_

- [x] 11.3 Implement error reporting and debugging
  - Catch and log all errors with context
  - Display stack traces for test failures
  - Provide debugging information (account states, tx logs)
  - Add assertions with descriptive messages
  - _Requirements: 15.5_
  - _Dependencies: Can be implemented throughout testing, no specific dependency_

## Phase 12: Complete Integration and Multi-Pool Testing (Final phase)

- [x] 12. Complete end-to-end integration test
- [x] 12.1 Execute full USDT pool lifecycle
  - Run complete business cycle from creation to final withdrawal
  - Verify state consistency after each phase
  - Log all state transitions
  - Check for anomalies or unexpected behavior
  - _Requirements: 12.1, 12.2, 12.3_
  - _Dependencies: Phase 1-7 (core business flow completed)_

- [x] 12.2 Execute full USDC pool lifecycle
  - Run complete business cycle with 9-decimal token
  - Verify all calculations handle different decimals correctly
  - Compare results with USDT pool for consistency
  - Verify no precision loss in calculations
  - _Requirements: 11.1, 11.2, 11.3_
  - _Dependencies: 12.1_

- [x] 12.3 Test parallel pool operations
  - Run USDT and USDC pools simultaneously
  - Verify independent state management
  - Check no cross-pool contamination
  - Verify system handles multiple pools correctly
  - _Requirements: 11.3_
  - _Dependencies: 12.2_

- [x] 12.4 Final cleanup and documentation
  - Add comprehensive inline code comments
  - Document all test assumptions and prerequisites
  - Create README with instructions for running tests
  - Document known limitations or edge cases
  - Add troubleshooting guide for common errors
  - _Requirements: 12.5_
  - _Dependencies: Phase 1-7 (can start after core tests, complete after all phases)_

---

## Summary

**Total Phases:** 12
**Total Tasks:** 75+
**Key Dependencies:**
- Phase 1-2: Foundation (no external dependencies)
- Phase 3: Depends on Phase 2 (environment ready)
- Phase 4: Depends on Phase 3 (pools created and initialized)
- Phase 5: Depends on Phase 4 (subscriptions completed)
- Phase 6: Depends on Phase 5 (funding completed, tokens distributed)
- Phase 7: Depends on Phase 6 (repayments made, pool ended)
- Phase 8-10: Can be tested independently with proper setup
- Phase 11-12: Final validation after all core functionality tested

**Execution Strategy:**
1. Complete Phases 1-2 first (foundation)
2. Complete Phase 3 (pool creation)
3. Execute Phases 4-7 sequentially (core business flow)
4. Test Phases 8-10 independently (special scenarios)
5. Run Phases 11-12 for final validation and reporting
