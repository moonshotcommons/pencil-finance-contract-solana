use crate::constants::*;
use crate::errors::PencilError;
use crate::state::{AssetPool, SystemConfig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateAssetPool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump
    )]
    pub system_config: Account<'info, SystemConfig>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<AssetPool>() + 64,
        seeds = [seeds::ASSET_POOL, payer.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub asset_pool: Account<'info, AssetPool>,

    /// 资产代币地址
    pub asset_address: Signer<'info>,

    pub system_program: Program<'info, System>,
}

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
    // 验证参数
    require!(
        !name.is_empty() && name.len() <= 64,
        PencilError::InvalidStringLength
    );
    require!(
        platform_fee <= MAX_PLATFORM_FEE,
        PencilError::InvalidPlatformFee
    );
    require!(
        senior_early_before_exit_fee <= MAX_EARLY_EXIT_FEE,
        PencilError::InvalidEarlyExitFee
    );
    require!(
        senior_early_after_exit_fee <= MAX_EARLY_EXIT_FEE,
        PencilError::InvalidEarlyExitFee
    );
    require!(
        junior_early_before_exit_fee <= MAX_EARLY_EXIT_FEE,
        PencilError::InvalidEarlyExitFee
    );
    require!(
        min_junior_ratio >= MIN_JUNIOR_RATIO && min_junior_ratio <= MAX_JUNIOR_RATIO,
        PencilError::InvalidMinJuniorRatio
    );
    require!(
        repayment_rate > 0 && repayment_rate <= MAX_ANNUAL_RATE,
        PencilError::InvalidRepaymentRate
    );
    require!(
        senior_fixed_rate > 0 && senior_fixed_rate <= MAX_ANNUAL_RATE,
        PencilError::InvalidSeniorFixedRate
    );
    require!(
        repayment_period >= MIN_REPAYMENT_PERIOD && repayment_period <= MAX_REPAYMENT_PERIOD,
        PencilError::InvalidRepaymentPeriod
    );
    require!(
        repayment_count > 0 && repayment_count <= MAX_REPAYMENT_COUNT,
        PencilError::InvalidRepaymentCount
    );
    require!(total_amount > 0, PencilError::InvalidFundingParams);
    require!(
        min_amount > 0 && min_amount <= total_amount,
        PencilError::InvalidFundingParams
    );
    require!(
        funding_start_time > 0 && funding_end_time > funding_start_time,
        PencilError::InvalidTimeParameters
    );
    require!(
        funding_end_time - funding_start_time >= MIN_FUNDING_PERIOD
            && funding_end_time - funding_start_time <= MAX_FUNDING_PERIOD,
        PencilError::InvalidTimeParameters
    );

    let asset_pool = &mut ctx.accounts.asset_pool;
    asset_pool.name = name;
    asset_pool.status = asset_pool_status::CREATED;
    asset_pool.asset_address = ctx.accounts.asset_address.key();
    asset_pool.system_config = ctx.accounts.system_config.key();
    asset_pool.platform_fee = platform_fee;
    asset_pool.senior_early_before_exit_fee = senior_early_before_exit_fee;
    asset_pool.senior_early_after_exit_fee = senior_early_after_exit_fee;
    asset_pool.junior_early_before_exit_fee = junior_early_before_exit_fee;
    asset_pool.min_junior_ratio = min_junior_ratio;
    asset_pool.repayment_rate = repayment_rate;
    asset_pool.senior_fixed_rate = senior_fixed_rate;
    asset_pool.repayment_period = repayment_period;
    asset_pool.repayment_count = repayment_count;
    asset_pool.total_amount = total_amount;
    asset_pool.min_amount = min_amount;
    asset_pool.funding_start_time = funding_start_time;
    asset_pool.funding_end_time = funding_end_time;
    asset_pool.creator = ctx.accounts.payer.key();
    asset_pool.created_at = Clock::get()?.unix_timestamp;

    msg!("Asset pool created: {}", asset_pool.name);
    msg!("Total amount: {}", total_amount);
    msg!("Min amount: {}", min_amount);

    Ok(())
}

#[derive(Accounts)]
#[instruction(creator: Pubkey, name: String)]
pub struct ApproveAssetPool<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = system_config.super_admin == admin.key() @ PencilError::Unauthorized
    )]
    pub system_config: Account<'info, SystemConfig>,

    #[account(
        mut,
        seeds = [seeds::ASSET_POOL, creator.as_ref(), name.as_bytes()],
        bump
    )]
    pub asset_pool: Account<'info, AssetPool>,
}

pub fn approve_asset_pool(
    ctx: Context<ApproveAssetPool>,
    _creator: Pubkey,
    _name: String,
) -> Result<()> {
    let asset_pool = &mut ctx.accounts.asset_pool;

    require!(
        asset_pool.status == asset_pool_status::CREATED,
        PencilError::AssetPoolAlreadyApproved
    );

    asset_pool.status = asset_pool_status::APPROVED;

    msg!("Asset pool approved: {}", asset_pool.name);

    Ok(())
}
