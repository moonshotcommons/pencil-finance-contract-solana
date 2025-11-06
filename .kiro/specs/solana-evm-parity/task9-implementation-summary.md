# Task 9 Implementation Summary: Junior 本金提取

## Overview
Implemented the `withdraw_principal` instruction to allow Junior investors to withdraw their principal after the asset pool has ended.

## Changes Made

### 1. Updated `programs/pencil-solana/src/instructions/repayment.rs`

#### Added PrincipalWithdrawn Event
```rust
#[event]
pub struct PrincipalWithdrawn {
    pub asset_pool: Pubkey,
    pub user: Pubkey,
    pub nft_id: u64,
    pub amount: u64,
    pub timestamp: i64,
}
```

#### Enhanced WithdrawPrincipal Context
- Added proper PDA derivation with seeds for all accounts
- Added constraint to verify AssetPool status is COMPLETED (ENDED)
- Added constraint to verify user owns the Junior NFT
- Added constraint to verify principal hasn't been withdrawn yet
- Added FirstLossPool account for principal source
- Added proper token account validations

#### Implemented Complete withdraw_principal Function
The function now:
1. ✅ Verifies AssetPool status is ENDED (COMPLETED) - via constraint
2. ✅ Verifies user holds Junior NFT - via constraint checking NFT ownership and amount == 1
3. ✅ Verifies user hasn't withdrawn principal yet - via constraint on principal_withdrawn field
4. ✅ Reads principal amount from JuniorNFTMetadata
5. ✅ Validates FirstLossPool has sufficient balance
6. ✅ Transfers principal from asset_pool_vault to user ATA (funds tracked by FirstLossPool)
7. ✅ Updates FirstLossPool.repaid_amount to track principal withdrawal
8. ✅ Marks JuniorNFTMetadata.principal_withdrawn = true
9. ✅ Emits PrincipalWithdrawn event with all relevant details

### 2. Updated `programs/pencil-solana/src/constants.rs`
- Added comment clarifying that COMPLETED status (value 5) represents ENDED state
- This is when Junior investors can withdraw their principal

## Key Implementation Details

### Account Structure
```rust
#[derive(Accounts)]
#[instruction(nft_id: u64)]
pub struct WithdrawPrincipal<'info> {
    pub user: Signer<'info>,
    pub system_config: Account<'info, SystemConfig>,
    pub asset_pool: Account<'info, AssetPool>,
    pub first_loss_pool: Account<'info, FirstLossPool>,
    pub nft_metadata: Account<'info, JuniorNFTMetadata>,
    pub user_nft_account: Account<'info, TokenAccount>,
    pub junior_nft_mint: Account<'info, Mint>,
    pub user_asset_account: Account<'info, TokenAccount>,
    pub asset_pool_vault: Account<'info, TokenAccount>,
    pub asset_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}
```

### Security Features
1. **System Pause Check**: Respects system-wide pause state
2. **Status Validation**: Only allows withdrawal when pool is COMPLETED
3. **Ownership Verification**: Validates NFT ownership through multiple constraints
4. **Double-Withdrawal Prevention**: Checks principal_withdrawn flag
5. **Balance Validation**: Ensures FirstLossPool has sufficient funds
6. **PDA Signing**: Uses proper PDA seeds for secure token transfers

### Fund Flow
```
FirstLossPool (tracked balance)
    ↓
asset_pool_vault (actual tokens)
    ↓
user_asset_account (Junior investor)
```

The FirstLossPool tracks Junior principal deposits. When withdrawing:
- Tokens transfer from asset_pool_vault to user
- FirstLossPool.repaid_amount increases to track withdrawal
- Available balance = total_deposits - repaid_amount

## Requirements Satisfied

All requirements from Requirement 10 are satisfied:

- ✅ 10.1: Verifies AssetPool status is ENDED (COMPLETED)
- ✅ 10.2: Verifies user holds Junior NFT
- ✅ 10.3: Verifies user hasn't withdrawn principal yet
- ✅ 10.4: Reads principal amount from JuniorNFTMetadata
- ✅ 10.5: Transfers principal to user ATA
- ✅ 10.6: Marks principal as withdrawn

## Testing Recommendations

To test this functionality:

1. **Setup Phase**:
   - Initialize system config
   - Create and approve asset pool
   - Initialize related accounts
   - Complete funding with Junior subscriptions
   - Distribute Junior NFTs

2. **Repayment Phase**:
   - Process all repayments
   - Update AssetPool status to COMPLETED

3. **Withdrawal Phase**:
   - Call withdraw_principal with NFT ID
   - Verify principal transferred to user
   - Verify principal_withdrawn flag set
   - Verify event emitted
   - Attempt duplicate withdrawal (should fail)

4. **Edge Cases**:
   - Try withdrawal before pool ends (should fail)
   - Try withdrawal without NFT ownership (should fail)
   - Try withdrawal with insufficient FirstLossPool balance (should fail)
   - Try withdrawal when system is paused (should fail)

## Build Status

✅ Code compiles successfully with `anchor build`
- Note: Pre-existing stack size warnings are present but don't affect functionality

## Next Steps

This completes Task 9. The next task in the implementation plan is:
- Task 10: 完善 SPL 代币与 NFT 实现
