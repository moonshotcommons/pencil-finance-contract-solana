use crate::constants::*;
use crate::errors::PencilError;
use crate::state::SystemConfig;
use anchor_lang::prelude::*;

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
        default_min_junior_ratio >= MIN_JUNIOR_RATIO
            && default_min_junior_ratio <= MAX_JUNIOR_RATIO,
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
    system_config.paused = false;
    system_config.initialized = true;

    msg!("System config initialized");
    msg!("Platform fee rate: {}", platform_fee_rate);
    msg!(
        "Senior early before exit fee rate: {}",
        senior_early_before_exit_fee_rate
    );
    msg!(
        "Senior early after exit fee rate: {}",
        senior_early_after_exit_fee_rate
    );
    msg!(
        "Junior early before exit fee rate: {}",
        junior_early_before_exit_fee_rate
    );
    msg!("Default min junior ratio: {}", default_min_junior_ratio);

    Ok(())
}

// ==================== Admin Role Management ====================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum AdminRole {
    SuperAdmin,
    SystemAdmin,
    TreasuryAdmin,
    OperationAdmin,
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    pub super_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = system_config.super_admin == super_admin.key() @ PencilError::Unauthorized
    )]
    pub system_config: Account<'info, SystemConfig>,
}

pub fn update_admin(ctx: Context<UpdateAdmin>, role: AdminRole, new_admin: Pubkey) -> Result<()> {
    let system_config = &mut ctx.accounts.system_config;
    let clock = Clock::get()?;

    let (role_u8, old_admin) = match role {
        AdminRole::SuperAdmin => {
            let old = system_config.super_admin;
            system_config.super_admin = new_admin;
            msg!("Super admin updated to: {}", new_admin);
            (0u8, old)
        }
        AdminRole::SystemAdmin => {
            let old = system_config.system_admin;
            system_config.system_admin = new_admin;
            msg!("System admin updated to: {}", new_admin);
            (1u8, old)
        }
        AdminRole::TreasuryAdmin => {
            let old = system_config.treasury_admin;
            system_config.treasury_admin = new_admin;
            msg!("Treasury admin updated to: {}", new_admin);
            (2u8, old)
        }
        AdminRole::OperationAdmin => {
            let old = system_config.operation_admin;
            system_config.operation_admin = new_admin;
            msg!("Operation admin updated to: {}", new_admin);
            (3u8, old)
        }
    };

    // Emit AdminUpdated event
    emit!(crate::AdminUpdated {
        role: role_u8,
        old_admin,
        new_admin,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== System Pause Management ====================

#[derive(Accounts)]
pub struct PauseSystem<'info> {
    #[account(mut)]
    pub super_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = system_config.super_admin == super_admin.key() @ PencilError::Unauthorized
    )]
    pub system_config: Account<'info, SystemConfig>,
}

pub fn pause_system(ctx: Context<PauseSystem>) -> Result<()> {
    let system_config = &mut ctx.accounts.system_config;
    let clock = Clock::get()?;

    system_config.paused = true;

    msg!("System paused at: {}", clock.unix_timestamp);

    // Emit SystemPaused event
    emit!(crate::SystemPaused {
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct UnpauseSystem<'info> {
    #[account(mut)]
    pub super_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = system_config.super_admin == super_admin.key() @ PencilError::Unauthorized
    )]
    pub system_config: Account<'info, SystemConfig>,
}

pub fn unpause_system(ctx: Context<UnpauseSystem>) -> Result<()> {
    let system_config = &mut ctx.accounts.system_config;
    let clock = Clock::get()?;

    system_config.paused = false;

    msg!("System unpaused at: {}", clock.unix_timestamp);

    // Emit SystemUnpaused event
    emit!(crate::SystemUnpaused {
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Fee Rate Management ====================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum FeeType {
    PlatformFee,
    SeniorEarlyBeforeExitFee,
    SeniorEarlyAfterExitFee,
    JuniorEarlyBeforeExitFee,
}

#[derive(Accounts)]
pub struct UpdateFeeRate<'info> {
    #[account(mut)]
    pub system_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = system_config.system_admin == system_admin.key() @ PencilError::Unauthorized
    )]
    pub system_config: Account<'info, SystemConfig>,
}

pub fn update_fee_rate(
    ctx: Context<UpdateFeeRate>,
    fee_type: FeeType,
    new_rate: u16,
) -> Result<()> {
    let system_config = &mut ctx.accounts.system_config;
    let clock = Clock::get()?;

    let (fee_type_u8, old_rate) = match fee_type {
        FeeType::PlatformFee => {
            require!(
                new_rate <= MAX_PLATFORM_FEE,
                PencilError::InvalidPlatformFee
            );
            let old = system_config.platform_fee_rate;
            system_config.platform_fee_rate = new_rate;
            msg!("Platform fee rate updated to: {}", new_rate);
            (0u8, old)
        }
        FeeType::SeniorEarlyBeforeExitFee => {
            require!(
                new_rate <= MAX_EARLY_EXIT_FEE,
                PencilError::InvalidEarlyExitFee
            );
            let old = system_config.senior_early_before_exit_fee_rate;
            system_config.senior_early_before_exit_fee_rate = new_rate;
            msg!("Senior early before exit fee rate updated to: {}", new_rate);
            (1u8, old)
        }
        FeeType::SeniorEarlyAfterExitFee => {
            require!(
                new_rate <= MAX_EARLY_EXIT_FEE,
                PencilError::InvalidEarlyExitFee
            );
            let old = system_config.senior_early_after_exit_fee_rate;
            system_config.senior_early_after_exit_fee_rate = new_rate;
            msg!("Senior early after exit fee rate updated to: {}", new_rate);
            (2u8, old)
        }
        FeeType::JuniorEarlyBeforeExitFee => {
            require!(
                new_rate <= MAX_EARLY_EXIT_FEE,
                PencilError::InvalidEarlyExitFee
            );
            let old = system_config.junior_early_before_exit_fee_rate;
            system_config.junior_early_before_exit_fee_rate = new_rate;
            msg!("Junior early before exit fee rate updated to: {}", new_rate);
            (3u8, old)
        }
    };

    // Emit FeeRateUpdated event
    emit!(crate::FeeRateUpdated {
        fee_type: fee_type_u8,
        old_rate,
        new_rate,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ==================== Treasury Management ====================

#[derive(Accounts)]
pub struct SetTreasury<'info> {
    #[account(mut)]
    pub system_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = system_config.system_admin == system_admin.key() @ PencilError::Unauthorized
    )]
    pub system_config: Account<'info, SystemConfig>,
}

pub fn set_treasury(ctx: Context<SetTreasury>, treasury: Pubkey) -> Result<()> {
    let system_config = &mut ctx.accounts.system_config;
    system_config.treasury = treasury;

    msg!("Treasury updated to: {}", treasury);

    Ok(())
}

// ==================== Asset Whitelist Management ====================

use crate::state::AssetWhitelist;

#[derive(Accounts)]
pub struct SetAssetSupported<'info> {
    #[account(mut)]
    pub system_admin: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = system_config.system_admin == system_admin.key() @ PencilError::Unauthorized
    )]
    pub system_config: Account<'info, SystemConfig>,

    #[account(
        init_if_needed,
        payer = system_admin,
        space = 8 + 32 + 4 + (32 * 100), // 账户头 + system_config + Vec长度 + 最多100个资产
        seeds = [seeds::ASSET_WHITELIST],
        bump
    )]
    pub asset_whitelist: Account<'info, AssetWhitelist>,

    pub system_program: Program<'info, System>,
}

pub fn set_asset_supported(
    ctx: Context<SetAssetSupported>,
    asset: Pubkey,
    supported: bool,
) -> Result<()> {
    let asset_whitelist = &mut ctx.accounts.asset_whitelist;
    let clock = Clock::get()?;

    // 如果是第一次初始化，设置 system_config
    if asset_whitelist.assets.is_empty() && asset_whitelist.system_config == Pubkey::default() {
        asset_whitelist.system_config = ctx.accounts.system_config.key();
    }

    // 检查资产是否已在列表中
    let asset_index = asset_whitelist.assets.iter().position(|&a| a == asset);

    if supported {
        // 添加到白名单
        if asset_index.is_none() {
            asset_whitelist.assets.push(asset);
            msg!("Asset added to whitelist: {}", asset);
        } else {
            msg!("Asset already in whitelist: {}", asset);
        }
    } else {
        // 从白名单移除
        if let Some(index) = asset_index {
            asset_whitelist.assets.remove(index);
            msg!("Asset removed from whitelist: {}", asset);
        } else {
            msg!("Asset not in whitelist: {}", asset);
        }
    }

    // Emit AssetSupportUpdated event
    emit!(crate::AssetSupportUpdated {
        asset,
        supported,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
