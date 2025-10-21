use anchor_lang::prelude::*;

declare_id!("RXo7Ai9ugeBp9giAKqera2pg1xMj49exA5SgWdMBMuM");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod pencil_solana {
    use super::*;

    // ==================== System Configuration ====================
    pub fn initialize_system_config(
        ctx: Context<InitializeSystemConfig>,
        platform_fee_rate: u16,
        senior_early_before_exit_fee_rate: u16,
        senior_early_after_exit_fee_rate: u16,
        junior_early_before_exit_fee_rate: u16,
        default_min_junior_ratio: u16,
    ) -> Result<()> {
        instructions::initialize_system_config(
            ctx,
            platform_fee_rate,
            senior_early_before_exit_fee_rate,
            senior_early_after_exit_fee_rate,
            junior_early_before_exit_fee_rate,
            default_min_junior_ratio,
        )
    }

    // ==================== Asset Pool ====================
    pub fn create_asset_pool(
        ctx: Context<CreateAssetPool>,
        name: String,
        platform_fee: u16,
        senior_early_before_exit_fee: u16,
        senior_early_after_exit_fee: u16,
        junior_early_before_exit_fee: u16,
        min_junior_ratio: u16,
        repayment_rate: u16,
        senior_fixed_rate: u16,
        repayment_period: u64,
        repayment_count: u64,
        total_amount: u64,
        min_amount: u64,
        funding_start_time: i64,
        funding_end_time: i64,
    ) -> Result<()> {
        instructions::create_asset_pool(
            ctx,
            name,
            platform_fee,
            senior_early_before_exit_fee,
            senior_early_after_exit_fee,
            junior_early_before_exit_fee,
            min_junior_ratio,
            repayment_rate,
            senior_fixed_rate,
            repayment_period,
            repayment_count,
            total_amount,
            min_amount,
            funding_start_time,
            funding_end_time,
        )
    }

    pub fn approve_asset_pool(ctx: Context<ApproveAssetPool>, creator: Pubkey) -> Result<()> {
        instructions::approve_asset_pool(ctx, creator)
    }

    // ==================== Funding ====================
    pub fn subscribe_senior(ctx: Context<SubscribeSenior>, amount: u64) -> Result<()> {
        instructions::subscribe_senior(ctx, amount)
    }

    pub fn subscribe_junior(ctx: Context<SubscribeJunior>, amount: u64) -> Result<()> {
        instructions::subscribe_junior(ctx, amount)
    }

    pub fn complete_funding(ctx: Context<CompleteFunding>) -> Result<()> {
        instructions::complete_funding(ctx)
    }

    pub fn refund_subscription(
        ctx: Context<RefundSubscription>,
        subscription_type: u8, // 0 for senior, 1 for junior
    ) -> Result<()> {
        instructions::refund_subscription(ctx, subscription_type)
    }

    // ==================== Repayment ====================
    pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
        instructions::repay(ctx, amount)
    }

    pub fn claim_interest(ctx: Context<ClaimInterest>, amount: u64) -> Result<()> {
        instructions::claim_interest(ctx, amount)
    }

    pub fn withdraw_principal(ctx: Context<WithdrawPrincipal>, nft_id: u64) -> Result<()> {
        instructions::withdraw_principal(ctx, nft_id)
    }

    pub fn early_exit_senior(ctx: Context<EarlyExitSenior>, amount: u64) -> Result<()> {
        instructions::early_exit_senior(ctx, amount)
    }

    // ==================== Token Management ====================
    pub fn mint_grow_token(ctx: Context<MintGrowToken>, amount: u64) -> Result<()> {
        instructions::mint_grow_token(ctx, amount)
    }

    pub fn burn_grow_token(ctx: Context<BurnGrowToken>, amount: u64) -> Result<()> {
        instructions::burn_grow_token(ctx, amount)
    }

    pub fn mint_junior_nft(ctx: Context<MintJuniorNFT>, nft_id: u64, principal: u64) -> Result<()> {
        instructions::mint_junior_nft(ctx, nft_id, principal)
    }
}
