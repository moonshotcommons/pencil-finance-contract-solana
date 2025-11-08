use anchor_lang::prelude::*;

declare_id!("G7XrbiCSkpBRyys2US19NLNf256Bm3BQ7okzDCWeuxzs");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

// ==================== Events ====================

#[event]
pub struct AdminUpdated {
    pub role: u8, // 0: SuperAdmin, 1: SystemAdmin, 2: TreasuryAdmin, 3: OperationAdmin
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct SystemPaused {
    pub timestamp: i64,
}

#[event]
pub struct SystemUnpaused {
    pub timestamp: i64,
}

#[event]
pub struct FeeRateUpdated {
    pub fee_type: u8, // 0: PlatformFee, 1: SeniorEarlyBeforeExitFee, 2: SeniorEarlyAfterExitFee, 3: JuniorEarlyBeforeExitFee
    pub old_rate: u16,
    pub new_rate: u16,
    pub timestamp: i64,
}

#[event]
pub struct AssetSupportUpdated {
    pub asset: Pubkey,
    pub supported: bool,
    pub timestamp: i64,
}

#[event]
pub struct RelatedAccountsInitialized {
    pub asset_pool: Pubkey,
    pub funding: Pubkey,
    pub senior_pool: Pubkey,
    pub first_loss_pool: Pubkey,
    pub junior_interest_pool: Pubkey,
    pub grow_token: Pubkey,
    pub asset_pool_vault: Pubkey,
    pub treasury_ata: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TokensDistributed {
    pub asset_pool: Pubkey,
    pub senior_amount: u64,
    pub junior_count: u64,
    pub timestamp: i64,
}

#[event]
pub struct RefundProcessed {
    pub asset_pool: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub subscription_type: u8,
    pub timestamp: i64,
}

#[event]
pub struct RepaymentDistributed {
    pub asset_pool: Pubkey,
    pub period: u64,
    pub total_amount: u64,
    pub platform_fee: u64,
    pub senior_amount: u64,
    pub junior_interest: u64,
    pub timestamp: i64,
}

#[event]
pub struct EarlyExitProcessed {
    pub asset_pool: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub fee: u64,
    pub net_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct InterestClaimed {
    pub asset_pool: Pubkey,
    pub user: Pubkey,
    pub nft_id: u64,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct PrincipalWithdrawn {
    pub asset_pool: Pubkey,
    pub user: Pubkey,
    pub nft_id: u64,
    pub amount: u64,
    pub timestamp: i64,
}

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

    pub fn update_admin(
        ctx: Context<UpdateAdmin>,
        role: instructions::AdminRole,
        new_admin: Pubkey,
    ) -> Result<()> {
        instructions::update_admin(ctx, role, new_admin)
    }

    pub fn pause_system(ctx: Context<PauseSystem>) -> Result<()> {
        instructions::pause_system(ctx)
    }

    pub fn unpause_system(ctx: Context<UnpauseSystem>) -> Result<()> {
        instructions::unpause_system(ctx)
    }

    pub fn update_fee_rate(
        ctx: Context<UpdateFeeRate>,
        fee_type: instructions::FeeType,
        new_rate: u16,
    ) -> Result<()> {
        instructions::update_fee_rate(ctx, fee_type, new_rate)
    }

    pub fn set_treasury(ctx: Context<SetTreasury>, treasury: Pubkey) -> Result<()> {
        instructions::set_treasury(ctx, treasury)
    }

    pub fn set_asset_supported(
        ctx: Context<SetAssetSupported>,
        asset: Pubkey,
        supported: bool,
    ) -> Result<()> {
        instructions::set_asset_supported(ctx, asset, supported)
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

    pub fn approve_asset_pool(
        ctx: Context<ApproveAssetPool>,
        creator: Pubkey,
        name: String,
    ) -> Result<()> {
        instructions::approve_asset_pool(ctx, creator, name)
    }

    pub fn initialize_related_accounts(ctx: Context<InitializeRelatedAccounts>) -> Result<()> {
        instructions::initialize_related_accounts(ctx)
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

    pub fn distribute_senior_token(ctx: Context<DistributeSeniorToken>) -> Result<()> {
        instructions::distribute_senior_token(ctx)
    }

    pub fn distribute_junior_nft(ctx: Context<DistributeJuniorNFT>, nft_id: u64) -> Result<()> {
        instructions::distribute_junior_nft(ctx, nft_id)
    }

    pub fn finalize_token_distribution(
        ctx: Context<FinalizeTokenDistribution>,
        senior_count: u64,
        junior_count: u64,
    ) -> Result<()> {
        instructions::finalize_token_distribution(ctx, senior_count, junior_count)
    }

    pub fn refund_subscription(
        ctx: Context<RefundSubscription>,
        subscription_type: u8, // 0 for senior, 1 for junior
    ) -> Result<()> {
        instructions::refund_subscription(ctx, subscription_type)
    }

    pub fn process_refund(ctx: Context<ProcessRefund>) -> Result<()> {
        instructions::process_refund(ctx)
    }

    pub fn cancel_asset_pool(ctx: Context<CancelAssetPool>) -> Result<()> {
        instructions::cancel_asset_pool(ctx)
    }

    pub fn withdraw_senior_subscription(ctx: Context<WithdrawSeniorSubscription>, amount: u64) -> Result<()> {
        instructions::withdraw_senior_subscription(ctx, amount)
    }

    pub fn withdraw_junior_subscription(ctx: Context<WithdrawJuniorSubscription>, amount: u64) -> Result<()> {
        instructions::withdraw_junior_subscription(ctx, amount)
    }

    // ==================== Repayment ====================
    pub fn repay(ctx: Context<Repay>, amount: u64, period: u64) -> Result<()> {
        instructions::repay(ctx, amount, period)
    }

    pub fn claim_junior_interest(ctx: Context<ClaimJuniorInterest>, nft_id: u64) -> Result<()> {
        instructions::claim_junior_interest(ctx, nft_id)
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
