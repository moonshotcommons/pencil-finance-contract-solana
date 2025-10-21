use anchor_lang::prelude::*;
use crate::state::SystemConfig;
use crate::errors::PencilError;
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeSystemConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<SystemConfig>(),
        seeds = [seeds::SYSTEM_CONFIG],
        bump
    )]
    pub system_config: Account<'info, SystemConfig>,

    /// 金库账户
    pub treasury: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_system_config(
    ctx: Context<InitializeSystemConfig>,
    platform_fee_rate: u16,
    senior_early_before_exit_fee_rate: u16,
    senior_early_after_exit_fee_rate: u16,
    junior_early_before_exit_fee_rate: u16,
    default_min_junior_ratio: u16,
) -> Result<()> {
    // 验证参数
    require!(
        platform_fee_rate <= MAX_PLATFORM_FEE,
        PencilError::InvalidPlatformFee
    );
    require!(
        senior_early_before_exit_fee_rate <= MAX_EARLY_EXIT_FEE,
        PencilError::InvalidEarlyExitFee
    );
    require!(
        senior_early_after_exit_fee_rate <= MAX_EARLY_EXIT_FEE,
        PencilError::InvalidEarlyExitFee
    );
    require!(
        junior_early_before_exit_fee_rate <= MAX_EARLY_EXIT_FEE,
        PencilError::InvalidEarlyExitFee
    );
    require!(
        default_min_junior_ratio >= MIN_JUNIOR_RATIO && default_min_junior_ratio <= MAX_JUNIOR_RATIO,
        PencilError::InvalidMinJuniorRatio
    );

    let system_config = &mut ctx.accounts.system_config;
    system_config.super_admin = ctx.accounts.payer.key();
    system_config.system_admin = ctx.accounts.payer.key();
    system_config.treasury_admin = ctx.accounts.payer.key();
    system_config.operation_admin = ctx.accounts.payer.key();
    system_config.treasury = ctx.accounts.treasury.key();
    system_config.platform_fee_rate = platform_fee_rate;
    system_config.senior_early_before_exit_fee_rate = senior_early_before_exit_fee_rate;
    system_config.senior_early_after_exit_fee_rate = senior_early_after_exit_fee_rate;
    system_config.junior_early_before_exit_fee_rate = junior_early_before_exit_fee_rate;
    system_config.default_min_junior_ratio = default_min_junior_ratio;
    system_config.initialized = true;

    msg!("System config initialized");
    msg!("Platform fee rate: {}", platform_fee_rate);
    msg!("Senior early before exit fee rate: {}", senior_early_before_exit_fee_rate);
    msg!("Senior early after exit fee rate: {}", senior_early_after_exit_fee_rate);
    msg!("Junior early before exit fee rate: {}", junior_early_before_exit_fee_rate);
    msg!("Default min junior ratio: {}", default_min_junior_ratio);

    Ok(())
}

