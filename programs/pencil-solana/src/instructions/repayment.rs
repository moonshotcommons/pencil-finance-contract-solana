use crate::constants::*;
use crate::errors::PencilError;
use crate::state::{
    AssetPool, AssetWhitelist, FirstLossPool, JuniorInterestPool, JuniorNFTMetadata,
    RepaymentRecord, SeniorPool, SystemConfig,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Transfer};

// RepaymentDistributed event is now defined in lib.rs

#[derive(Accounts)]
#[instruction(amount: u64, period: u64)]
pub struct Repay<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = !system_config.paused @ PencilError::SystemPaused
    )]
    pub system_config: Box<Account<'info, SystemConfig>>,

    #[account(
        seeds = [seeds::ASSET_WHITELIST],
        bump
    )]
    pub asset_whitelist: Box<Account<'info, AssetWhitelist>>,

    #[account(
        mut,
        seeds = [seeds::ASSET_POOL, asset_pool.creator.as_ref(), &asset_pool.name],
        bump
    )]
    pub asset_pool: Box<Account<'info, AssetPool>>,

    #[account(
        mut,
        seeds = [seeds::SENIOR_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub senior_pool: Box<Account<'info, SeniorPool>>,

    #[account(
        mut,
        seeds = [seeds::FIRST_LOSS_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub first_loss_pool: Box<Account<'info, FirstLossPool>>,

    #[account(
        mut,
        seeds = [seeds::JUNIOR_INTEREST_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub junior_interest_pool: Box<Account<'info, JuniorInterestPool>>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = payer
    )]
    pub payer_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub asset_pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = asset_mint
    )]
    pub treasury_ata: Box<Account<'info, TokenAccount>>,

    pub asset_mint: Box<Account<'info, anchor_spl::token::Mint>>,

    /// Repayment record - one per period
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<RepaymentRecord>(),
        seeds = [seeds::REPAYMENT_RECORD, asset_pool.key().as_ref(), &period.to_le_bytes()],
        bump
    )]
    pub repayment_record: Box<Account<'info, RepaymentRecord>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn repay(ctx: Context<Repay>, amount: u64, period: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidRepaymentAmount);

    let asset_pool = &mut ctx.accounts.asset_pool;
    let clock = Clock::get()?;

    // 验证资产在白名单中
    let asset_address = asset_pool.asset_address;
    require!(
        ctx.accounts.asset_whitelist.assets.contains(&asset_address),
        PencilError::AssetNotSupported
    );

    // 检查资产池状态
    require!(
        asset_pool.status == asset_pool_status::FUNDED
            || asset_pool.status == asset_pool_status::REPAYING,
        PencilError::InvalidAssetPoolStatus
    );

    // 验证相关账户已初始化
    require!(
        asset_pool.related_accounts_initialized,
        PencilError::RelatedAccountsNotInitialized
    );

    // 1. 验证期数参数
    require!(
        period > 0 && period <= asset_pool.repayment_count,
        PencilError::InvalidRepaymentPeriod
    );

    // 验证当前时间是否已到该期数
    let calculated_period =
        calculate_current_period(asset_pool.funding_end_time, asset_pool.repayment_period)?;
    require!(period <= calculated_period, PencilError::RepaymentNotDue);

    // 2. 接收还款到资产池 Vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.payer_token_account.to_account_info(),
        to: ctx.accounts.asset_pool_vault.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // 3. 计算每期应还总额
    let per_period_total = calculate_per_period_amount(
        asset_pool.total_amount,
        asset_pool.repayment_count,
        asset_pool.repayment_rate,
    )?;

    // 验证还款金额是否足够
    require!(
        amount >= per_period_total,
        PencilError::InvalidRepaymentAmount
    );

    // 4. 计算并转账平台费至金库
    let platform_fee = calculate_platform_fee(per_period_total, asset_pool.platform_fee)?;

    // 准备 PDA 签名种子（在修改 asset_pool 之前）
    let asset_pool_bump = ctx.bumps.asset_pool;
    let asset_pool_creator = asset_pool.creator;
    let asset_pool_name = asset_pool.name.clone();

    if platform_fee > 0 {
        let asset_pool_seeds = &[
            seeds::ASSET_POOL,
            asset_pool_creator.as_ref(),
            &asset_pool_name,
            &[asset_pool_bump],
        ];
        let signer_seeds = &[&asset_pool_seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.asset_pool_vault.to_account_info(),
            to: ctx.accounts.treasury_ata.to_account_info(),
            authority: asset_pool.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, platform_fee)?;
    }

    // 5. 计算 Senior 应得本息
    let senior_amount = calculate_senior_amount(
        asset_pool.senior_amount,
        asset_pool.repayment_count,
        asset_pool.senior_fixed_rate,
    )?;

    // 6. 分配 Senior 应得本息（记录在 SeniorPool 状态中）
    let mut actual_senior_amount = senior_amount;

    // 计算当前 Vault 中可用于 Senior 的余额（扣除平台费后）
    let available_for_senior = ctx
        .accounts
        .asset_pool_vault
        .amount
        .checked_sub(platform_fee)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 7. 如果可用余额不足，从 FirstLossPool 补足
    let mut first_loss_used = 0u64;
    if available_for_senior < senior_amount {
        let shortfall = senior_amount
            .checked_sub(available_for_senior)
            .ok_or(PencilError::ArithmeticOverflow)?;

        // 检查 FirstLossPool 是否有足够余额（通过 total_deposits - repaid_amount 计算）
        let first_loss_pool = &ctx.accounts.first_loss_pool;
        let first_loss_available = first_loss_pool
            .total_deposits
            .checked_sub(first_loss_pool.repaid_amount)
            .ok_or(PencilError::ArithmeticOverflow)?;

        if first_loss_available >= shortfall {
            // FirstLossPool 有足够余额，记录使用金额
            first_loss_used = shortfall;
            msg!("FirstLossPool补足差额: {} tokens", shortfall);
        } else {
            // FirstLossPool 余额不足，只能分配可用金额
            first_loss_used = first_loss_available;
            actual_senior_amount = available_for_senior
                .checked_add(first_loss_used)
                .ok_or(PencilError::ArithmeticOverflow)?;
            msg!(
                "警告: FirstLossPool 余额不足，Senior 实际获得: {} tokens",
                actual_senior_amount
            );
        }
    }

    // 更新 SeniorPool 已还款金额（资金留在 asset_pool_vault 中）
    let senior_pool = &mut ctx.accounts.senior_pool;
    senior_pool.repaid_amount = senior_pool
        .repaid_amount
        .checked_add(actual_senior_amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 更新 FirstLossPool 已使用金额
    if first_loss_used > 0 {
        let first_loss_pool = &mut ctx.accounts.first_loss_pool;
        first_loss_pool.repaid_amount = first_loss_pool
            .repaid_amount
            .checked_add(first_loss_used)
            .ok_or(PencilError::ArithmeticOverflow)?;
    }

    // 8. 计算剩余金额并分配至 JuniorInterestPool（记录在状态中）
    let junior_interest = amount
        .checked_sub(platform_fee)
        .ok_or(PencilError::ArithmeticOverflow)?
        .checked_sub(actual_senior_amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    if junior_interest > 0 {
        // 更新 JuniorInterestPool（资金留在 asset_pool_vault 中）
        let junior_interest_pool = &mut ctx.accounts.junior_interest_pool;
        junior_interest_pool.total_interest = junior_interest_pool
            .total_interest
            .checked_add(junior_interest)
            .ok_or(PencilError::ArithmeticOverflow)?;
    }

    // 9. 创建 RepaymentRecord 并记录期数
    let repayment_record = &mut ctx.accounts.repayment_record;
    repayment_record.asset_pool = asset_pool.key();
    repayment_record.period = period;
    repayment_record.amount = amount;
    repayment_record.repaid_at = clock.unix_timestamp;
    repayment_record.status = repayment_status::COMPLETED;

    // 更新资产池
    asset_pool.repaid_amount = asset_pool
        .repaid_amount
        .checked_add(amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 更新资产池状态为 REPAYING
    if asset_pool.status == asset_pool_status::FUNDED {
        asset_pool.status = asset_pool_status::REPAYING;
    }

    // 检查是否是最后一期还款，如果是则将状态设置为 COMPLETED
    if period == asset_pool.repayment_count {
        asset_pool.status = asset_pool_status::COMPLETED;
        msg!("All repayments completed. Pool status set to COMPLETED.");
    }

    // 10. 发出 RepaymentDistributed 事件
    emit!(crate::RepaymentDistributed {
        asset_pool: asset_pool.key(),
        period,
        total_amount: amount,
        platform_fee,
        senior_amount: actual_senior_amount,
        junior_interest,
        timestamp: clock.unix_timestamp,
    });

    msg!(
        "还款分配完成 - 期数: {}, 总额: {}, 平台费: {}, Senior: {}, Junior利息: {}",
        period,
        amount,
        platform_fee,
        actual_senior_amount,
        junior_interest
    );

    Ok(())
}

/// 计算当前应还期数
/// repayment_period: 还款周期（秒数）
fn calculate_current_period(funding_end_time: i64, repayment_period: u64) -> Result<u64> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;

    // 如果还未到募资结束时间，返回 0
    if current_time < funding_end_time {
        return Ok(0);
    }

    // 计算已过去的时间（秒）
    let elapsed_seconds = current_time
        .checked_sub(funding_end_time)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 计算当前期数（直接用秒数计算，支持任意时间单位）
    let count = if repayment_period > 0 {
        (elapsed_seconds as u64) / repayment_period
    } else {
        0
    };

    // 与EVM一致：如果还没到第一个还款期，也允许还第一期
    // return count > 0 ? count : 1
    let period = if count > 0 { count } else { 1 };

    Ok(period)
}

/// 计算每期应还总额 = 本金/期数 + 本金 × 还款利率
fn calculate_per_period_amount(
    total_amount: u64,
    repayment_count: u64,
    repayment_rate: u16,
) -> Result<u64> {
    require!(repayment_count > 0, PencilError::InvalidRepaymentCount);

    // 每期本金
    let principal_per_period = total_amount
        .checked_div(repayment_count)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 每期利息 = 本金 × 还款利率 / 10000
    let interest_per_period = ((total_amount as u128)
        .checked_mul(repayment_rate as u128)
        .ok_or(PencilError::ArithmeticOverflow)?)
    .checked_div(BASIS_POINTS as u128)
    .ok_or(PencilError::ArithmeticOverflow)? as u64;

    // 每期总额
    let per_period_total = principal_per_period
        .checked_add(interest_per_period)
        .ok_or(PencilError::ArithmeticOverflow)?;

    Ok(per_period_total)
}

/// 计算平台费 = 每期应还金额 × 平台费率
fn calculate_platform_fee(per_period_amount: u64, platform_fee_rate: u16) -> Result<u64> {
    let platform_fee = ((per_period_amount as u128)
        .checked_mul(platform_fee_rate as u128)
        .ok_or(PencilError::ArithmeticOverflow)?)
    .checked_div(BASIS_POINTS as u128)
    .ok_or(PencilError::ArithmeticOverflow)? as u64;

    Ok(platform_fee)
}

/// 计算 Senior 应得本息 = Senior本金/期数 + Senior本金 × Senior固定利率
fn calculate_senior_amount(
    senior_total: u64,
    repayment_count: u64,
    senior_fixed_rate: u16,
) -> Result<u64> {
    require!(repayment_count > 0, PencilError::InvalidRepaymentCount);

    // 每期 Senior 本金
    let senior_principal_per_period = senior_total
        .checked_div(repayment_count)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 每期 Senior 利息 = Senior本金 × Senior固定利率 / 10000
    let senior_interest_per_period = ((senior_total as u128)
        .checked_mul(senior_fixed_rate as u128)
        .ok_or(PencilError::ArithmeticOverflow)?)
    .checked_div(BASIS_POINTS as u128)
    .ok_or(PencilError::ArithmeticOverflow)? as u64;

    // 每期 Senior 总额
    let senior_per_period = senior_principal_per_period
        .checked_add(senior_interest_per_period)
        .ok_or(PencilError::ArithmeticOverflow)?;

    Ok(senior_per_period)
}

// InterestClaimed event is now defined in lib.rs

#[derive(Accounts)]
#[instruction(nft_id: u64)]
pub struct ClaimJuniorInterest<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = !system_config.paused @ PencilError::SystemPaused
    )]
    pub system_config: Box<Account<'info, SystemConfig>>,

    #[account(
        mut,
        seeds = [seeds::ASSET_POOL, asset_pool.creator.as_ref(), &asset_pool.name],
        bump
    )]
    pub asset_pool: Box<Account<'info, AssetPool>>,

    #[account(
        mut,
        seeds = [seeds::FIRST_LOSS_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub first_loss_pool: Box<Account<'info, FirstLossPool>>,

    #[account(
        mut,
        seeds = [seeds::JUNIOR_INTEREST_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub junior_interest_pool: Box<Account<'info, JuniorInterestPool>>,

    #[account(
        mut,
        seeds = [seeds::JUNIOR_NFT_METADATA, asset_pool.key().as_ref(), &nft_id.to_le_bytes()],
        bump,
        constraint = nft_metadata.owner == user.key() @ PencilError::Unauthorized
    )]
    pub nft_metadata: Box<Account<'info, JuniorNFTMetadata>>,

    #[account(
        mut,
        token::mint = junior_nft_mint,
        token::authority = user,
        constraint = user_nft_account.amount == 1 @ PencilError::Unauthorized
    )]
    pub user_nft_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [seeds::JUNIOR_NFT_MINT, asset_pool.key().as_ref(), &nft_id.to_le_bytes()],
        bump
    )]
    pub junior_nft_mint: Box<Account<'info, anchor_spl::token::Mint>>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = user
    )]
    pub user_asset_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub asset_pool_vault: Box<Account<'info, TokenAccount>>,

    pub asset_mint: Box<Account<'info, anchor_spl::token::Mint>>,

    pub token_program: Program<'info, Token>,
}

pub fn claim_junior_interest(ctx: Context<ClaimJuniorInterest>, nft_id: u64) -> Result<()> {
    let asset_pool = &ctx.accounts.asset_pool;
    let junior_interest_pool = &mut ctx.accounts.junior_interest_pool;
    let first_loss_pool = &ctx.accounts.first_loss_pool;
    let nft_metadata = &mut ctx.accounts.nft_metadata;
    let clock = Clock::get()?;

    // 1. 验证用户持有 Junior NFT（已在 constraint 中验证）
    // 2. 从 JuniorInterestPool 计算可领取利息
    // 公式: 可领取 = (JuniorInterestPool 总额 × NFT 本金) / Junior 总本金 - 已领取

    // 获取 Junior 总本金
    let junior_total_principal = first_loss_pool.total_deposits;
    require!(
        junior_total_principal > 0,
        PencilError::InvalidPrincipalCalculation
    );

    // 计算该 NFT 应得的总利息份额
    let nft_share = ((junior_interest_pool.total_interest as u128)
        .checked_mul(nft_metadata.principal as u128)
        .ok_or(PencilError::ArithmeticOverflow)?)
    .checked_div(junior_total_principal as u128)
    .ok_or(PencilError::ArithmeticOverflow)? as u64;

    // 计算可领取利息（总份额 - 已领取）
    let claimable_interest = nft_share
        .checked_sub(nft_metadata.claimed_interest)
        .ok_or(PencilError::ArithmeticOverflow)?;

    require!(claimable_interest > 0, PencilError::NoInterestToClaim);

    // 验证 JuniorInterestPool 有足够的未分配利息
    let undistributed_interest = junior_interest_pool
        .total_interest
        .checked_sub(junior_interest_pool.distributed_interest)
        .ok_or(PencilError::ArithmeticOverflow)?;

    require!(
        undistributed_interest >= claimable_interest,
        PencilError::InsufficientPoolFunds
    );

    // 3. 转账利息至用户 ATA
    let asset_pool_bump = ctx.bumps.asset_pool;
    let asset_pool_creator = asset_pool.creator;
    let asset_pool_name = asset_pool.name.clone();
    let asset_pool_seeds = &[
        seeds::ASSET_POOL,
        asset_pool_creator.as_ref(),
        &asset_pool_name,
        &[asset_pool_bump],
    ];
    let signer_seeds = &[&asset_pool_seeds[..]];

    let transfer_cpi_accounts = Transfer {
        from: ctx.accounts.asset_pool_vault.to_account_info(),
        to: ctx.accounts.user_asset_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let transfer_cpi_program = ctx.accounts.token_program.to_account_info();
    let transfer_cpi_ctx =
        CpiContext::new_with_signer(transfer_cpi_program, transfer_cpi_accounts, signer_seeds);
    token::transfer(transfer_cpi_ctx, claimable_interest)?;

    // 4. 更新 JuniorInterestPool 的已分配金额
    junior_interest_pool.distributed_interest = junior_interest_pool
        .distributed_interest
        .checked_add(claimable_interest)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 5. 更新 JuniorNFTMetadata 的 claimed_interest 字段
    nft_metadata.claimed_interest = nft_metadata
        .claimed_interest
        .checked_add(claimable_interest)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 6. 发出 InterestClaimed 事件
    emit!(crate::InterestClaimed {
        asset_pool: asset_pool.key(),
        user: ctx.accounts.user.key(),
        nft_id,
        amount: claimable_interest,
        timestamp: clock.unix_timestamp,
    });

    msg!(
        "Junior 利息领取完成 - NFT ID: {}, 用户: {}, 金额: {}",
        nft_id,
        ctx.accounts.user.key(),
        claimable_interest
    );

    Ok(())
}

// PrincipalWithdrawn event is now defined in lib.rs

#[derive(Accounts)]
#[instruction(nft_id: u64)]
pub struct WithdrawPrincipal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = !system_config.paused @ PencilError::SystemPaused
    )]
    pub system_config: Box<Account<'info, SystemConfig>>,

    #[account(
        mut,
        seeds = [seeds::ASSET_POOL, asset_pool.creator.as_ref(), &asset_pool.name],
        bump,
        constraint = asset_pool.status == asset_pool_status::COMPLETED @ PencilError::InvalidAssetPoolStatus
    )]
    pub asset_pool: Box<Account<'info, AssetPool>>,

    #[account(
        mut,
        seeds = [seeds::FIRST_LOSS_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub first_loss_pool: Box<Account<'info, FirstLossPool>>,

    #[account(
        mut,
        seeds = [seeds::JUNIOR_NFT_METADATA, asset_pool.key().as_ref(), &nft_id.to_le_bytes()],
        bump,
        constraint = nft_metadata.owner == user.key() @ PencilError::Unauthorized,
        constraint = !nft_metadata.principal_withdrawn @ PencilError::NoPrincipalToWithdraw
    )]
    pub nft_metadata: Box<Account<'info, JuniorNFTMetadata>>,

    #[account(
        mut,
        token::mint = junior_nft_mint,
        token::authority = user,
        constraint = user_nft_account.amount == 1 @ PencilError::Unauthorized
    )]
    pub user_nft_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [seeds::JUNIOR_NFT_MINT, asset_pool.key().as_ref(), &nft_id.to_le_bytes()],
        bump
    )]
    pub junior_nft_mint: Box<Account<'info, anchor_spl::token::Mint>>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = user
    )]
    pub user_asset_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub asset_pool_vault: Box<Account<'info, TokenAccount>>,

    pub asset_mint: Box<Account<'info, anchor_spl::token::Mint>>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw_principal(ctx: Context<WithdrawPrincipal>, nft_id: u64) -> Result<()> {
    let asset_pool = &ctx.accounts.asset_pool;
    let first_loss_pool = &mut ctx.accounts.first_loss_pool;
    let nft_metadata = &mut ctx.accounts.nft_metadata;
    let clock = Clock::get()?;

    // 1. 验证 AssetPool 状态为 ENDED (COMPLETED)
    // (已在 constraint 中验证)

    // 2. 验证用户持有 Junior NFT
    // (已在 constraint 中验证)

    // 3. 验证用户未提取过本金
    // (已在 constraint 中验证)

    // 4. 从 JuniorNFTMetadata 读取用户份额
    let user_shares = nft_metadata.principal;
    require!(user_shares > 0, PencilError::InvalidPrincipalCalculation);

    // 5. 计算按比例分配的金额（首损机制）
    // 按照EVM逻辑：assetAmount = vaultBalance * userShares / totalRemainingShares

    // 获取vault当前余额
    let vault_balance = ctx.accounts.asset_pool_vault.amount;

    // 计算剩余未提取的总份额
    let total_remaining_shares = first_loss_pool
        .total_deposits
        .checked_sub(first_loss_pool.repaid_amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    require!(
        total_remaining_shares > 0,
        PencilError::InvalidPrincipalCalculation
    );

    // 按比例计算用户应得金额
    // actual_amount = vault_balance * user_shares / total_remaining_shares
    let actual_amount = (vault_balance as u128)
        .checked_mul(user_shares as u128)
        .ok_or(PencilError::ArithmeticOverflow)?
        .checked_div(total_remaining_shares as u128)
        .ok_or(PencilError::ArithmeticOverflow)? as u64;

    msg!(
        "Junior本金提取计算 - Vault余额: {}, 用户份额: {}, 剩余总份额: {}, 实际金额: {}",
        vault_balance,
        user_shares,
        total_remaining_shares,
        actual_amount
    );

    // 6. 从 FirstLossPool 转账按比例计算的金额至用户 ATA
    // 准备 PDA 签名种子
    let asset_pool_bump = ctx.bumps.asset_pool;
    let asset_pool_creator = asset_pool.creator;
    let asset_pool_name = asset_pool.name.clone();
    let asset_pool_seeds = &[
        seeds::ASSET_POOL,
        asset_pool_creator.as_ref(),
        &asset_pool_name,
        &[asset_pool_bump],
    ];
    let signer_seeds = &[&asset_pool_seeds[..]];

    let transfer_cpi_accounts = Transfer {
        from: ctx.accounts.asset_pool_vault.to_account_info(),
        to: ctx.accounts.user_asset_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let transfer_cpi_program = ctx.accounts.token_program.to_account_info();
    let transfer_cpi_ctx =
        CpiContext::new_with_signer(transfer_cpi_program, transfer_cpi_accounts, signer_seeds);
    token::transfer(transfer_cpi_ctx, actual_amount)?;

    // 更新 FirstLossPool repaid_amount（记录已处理的份额）
    // 注意：这里记录的是用户份额，不是实际转账金额（按比例分配机制）
    first_loss_pool.repaid_amount = first_loss_pool
        .repaid_amount
        .checked_add(user_shares)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 7. 标记 JuniorNFTMetadata.principal_withdrawn = true
    nft_metadata.principal_withdrawn = true;

    // 8. 发出 PrincipalWithdrawn 事件
    emit!(crate::PrincipalWithdrawn {
        asset_pool: asset_pool.key(),
        user: ctx.accounts.user.key(),
        nft_id,
        amount: actual_amount,
        timestamp: clock.unix_timestamp,
    });

    msg!(
        "Junior 本金提取完成 - NFT ID: {}, 用户: {}, 份额: {}, 实际金额: {}",
        nft_id,
        ctx.accounts.user.key(),
        user_shares,
        actual_amount
    );

    Ok(())
}

// EarlyExitProcessed event is now defined in lib.rs

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct EarlyExitSenior<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump,
        constraint = !system_config.paused @ PencilError::SystemPaused
    )]
    pub system_config: Box<Account<'info, SystemConfig>>,

    #[account(
        mut,
        seeds = [seeds::ASSET_POOL, asset_pool.creator.as_ref(), &asset_pool.name],
        bump
    )]
    pub asset_pool: Box<Account<'info, AssetPool>>,

    #[account(
        mut,
        seeds = [seeds::SENIOR_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub senior_pool: Box<Account<'info, SeniorPool>>,

    #[account(
        mut,
        seeds = [seeds::FIRST_LOSS_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub first_loss_pool: Box<Account<'info, FirstLossPool>>,

    #[account(
        mut,
        seeds = [seeds::GROW_TOKEN_MINT, asset_pool.key().as_ref()],
        bump
    )]
    pub grow_token_mint: Box<Account<'info, anchor_spl::token::Mint>>,

    #[account(
        mut,
        token::mint = grow_token_mint,
        token::authority = user
    )]
    pub user_grow_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = user
    )]
    pub user_asset_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub asset_pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = asset_mint
    )]
    pub treasury_ata: Box<Account<'info, TokenAccount>>,

    pub asset_mint: Box<Account<'info, anchor_spl::token::Mint>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn early_exit_senior(ctx: Context<EarlyExitSenior>, amount: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidEarlyExitAmount);

    let asset_pool = &ctx.accounts.asset_pool;
    let clock = Clock::get()?;

    // 1. 验证募资已完成（AssetPool 状态为 FUNDED, REPAYING 或 COMPLETED）
    // COMPLETED状态允许Senior按比例提取剩余资金（类似EVM的withdraw方法）
    require!(
        asset_pool.status == asset_pool_status::FUNDED
            || asset_pool.status == asset_pool_status::REPAYING
            || asset_pool.status == asset_pool_status::COMPLETED,
        PencilError::InvalidAssetPoolStatus
    );

    // 验证相关账户已初始化
    require!(
        asset_pool.related_accounts_initialized,
        PencilError::RelatedAccountsNotInitialized
    );

    // 验证用户持有足够的 GROW Token
    require!(
        ctx.accounts.user_grow_token_account.amount >= amount,
        PencilError::InsufficientBalance
    );

    // 如果池已完成（COMPLETED状态），使用按比例分配的逻辑（类似EVM的withdraw方法）
    if asset_pool.status == asset_pool_status::COMPLETED {
        return handle_senior_withdraw_after_completion(ctx, amount);
    }

    // 以下是early exit逻辑（FUNDED和REPAYING状态）
    // 2. 根据时间计算早退费率（募资结束前/后使用不同费率）
    let early_exit_fee_rate = if clock.unix_timestamp < asset_pool.funding_end_time {
        asset_pool.senior_early_before_exit_fee
    } else {
        asset_pool.senior_early_after_exit_fee
    };

    // 3. 计算早退费用和净退款金额
    let exit_fee = ((amount as u128)
        .checked_mul(early_exit_fee_rate as u128)
        .ok_or(PencilError::ArithmeticOverflow)?)
    .checked_div(BASIS_POINTS as u128)
    .ok_or(PencilError::ArithmeticOverflow)? as u64;

    let net_refund_amount = amount
        .checked_sub(exit_fee)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 4. 销毁用户的 GROW Token
    let burn_cpi_accounts = Burn {
        mint: ctx.accounts.grow_token_mint.to_account_info(),
        from: ctx.accounts.user_grow_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let burn_cpi_program = ctx.accounts.token_program.to_account_info();
    let burn_cpi_ctx = CpiContext::new(burn_cpi_program, burn_cpi_accounts);
    token::burn(burn_cpi_ctx, amount)?;

    msg!("GROW Token burned: {} tokens", amount);

    // 准备 PDA 签名种子
    let asset_pool_bump = ctx.bumps.asset_pool;
    let asset_pool_creator = asset_pool.creator;
    let asset_pool_name = asset_pool.name.clone();
    let asset_pool_seeds = &[
        seeds::ASSET_POOL,
        asset_pool_creator.as_ref(),
        &asset_pool_name,
        &[asset_pool_bump],
    ];
    let signer_seeds = &[&asset_pool_seeds[..]];

    // 5. 转账早退费用至金库 ATA
    if exit_fee > 0 {
        let fee_transfer_cpi_accounts = Transfer {
            from: ctx.accounts.asset_pool_vault.to_account_info(),
            to: ctx.accounts.treasury_ata.to_account_info(),
            authority: ctx.accounts.asset_pool.to_account_info(),
        };
        let fee_transfer_cpi_program = ctx.accounts.token_program.to_account_info();
        let fee_transfer_cpi_ctx = CpiContext::new_with_signer(
            fee_transfer_cpi_program,
            fee_transfer_cpi_accounts,
            signer_seeds,
        );
        token::transfer(fee_transfer_cpi_ctx, exit_fee)?;

        msg!(
            "Early exit fee transferred to treasury: {} tokens",
            exit_fee
        );
    }

    // 6. 检查资产池 Vault 余额是否足够
    let vault_balance = ctx.accounts.asset_pool_vault.amount;
    let mut actual_refund = net_refund_amount;
    let mut first_loss_used = 0u64;

    // 7. 如果 Vault 余额不足，从 FirstLossPool 请求补足
    if vault_balance < net_refund_amount {
        let shortfall = net_refund_amount
            .checked_sub(vault_balance)
            .ok_or(PencilError::ArithmeticOverflow)?;

        // 检查 FirstLossPool 可用余额
        let first_loss_pool = &ctx.accounts.first_loss_pool;
        let first_loss_available = first_loss_pool
            .total_deposits
            .checked_sub(first_loss_pool.repaid_amount)
            .ok_or(PencilError::ArithmeticOverflow)?;

        if first_loss_available >= shortfall {
            // FirstLossPool 有足够余额
            first_loss_used = shortfall;
            msg!("FirstLossPool 补足早退差额: {} tokens", first_loss_used);
        } else {
            // FirstLossPool 余额不足，只能退还可用金额
            first_loss_used = first_loss_available;
            actual_refund = vault_balance
                .checked_add(first_loss_used)
                .ok_or(PencilError::ArithmeticOverflow)?;
            msg!(
                "警告: 资金不足，实际退款: {} tokens (Vault: {}, FirstLoss: {})",
                actual_refund,
                vault_balance,
                first_loss_used
            );
        }
    }

    // 8. 转账净退款金额至用户 ATA
    if actual_refund > 0 {
        let refund_transfer_cpi_accounts = Transfer {
            from: ctx.accounts.asset_pool_vault.to_account_info(),
            to: ctx.accounts.user_asset_account.to_account_info(),
            authority: ctx.accounts.asset_pool.to_account_info(),
        };
        let refund_transfer_cpi_program = ctx.accounts.token_program.to_account_info();
        let refund_transfer_cpi_ctx = CpiContext::new_with_signer(
            refund_transfer_cpi_program,
            refund_transfer_cpi_accounts,
            signer_seeds,
        );
        token::transfer(refund_transfer_cpi_ctx, actual_refund)?;

        msg!("Net refund transferred to user: {} tokens", actual_refund);
    }

    // 更新 FirstLossPool 已使用金额
    if first_loss_used > 0 {
        let first_loss_pool = &mut ctx.accounts.first_loss_pool;
        first_loss_pool.repaid_amount = first_loss_pool
            .repaid_amount
            .checked_add(first_loss_used)
            .ok_or(PencilError::ArithmeticOverflow)?;
    }

    // 更新 SeniorPool 总存款（减少早退金额）
    let senior_pool = &mut ctx.accounts.senior_pool;
    senior_pool.total_deposits = senior_pool
        .total_deposits
        .checked_sub(amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 9. 发出 EarlyExitProcessed 事件
    emit!(crate::EarlyExitProcessed {
        asset_pool: asset_pool.key(),
        user: ctx.accounts.user.key(),
        amount,
        fee: exit_fee,
        net_amount: actual_refund,
        timestamp: clock.unix_timestamp,
    });

    msg!(
        "Senior 早退完成 - 用户: {}, 金额: {}, 费用: {}, 净退款: {}",
        ctx.accounts.user.key(),
        amount,
        exit_fee,
        actual_refund
    );

    Ok(())
}

/// 处理池完成后的Senior按比例提取（类似EVM的withdraw方法）
fn handle_senior_withdraw_after_completion(
    ctx: Context<EarlyExitSenior>,
    amount: u64,
) -> Result<()> {
    // 1. 获取GROW token总供应量
    let grow_total_supply = ctx.accounts.grow_token_mint.supply;
    require!(
        grow_total_supply > 0,
        PencilError::InvalidPrincipalCalculation
    );

    // 2. 获取vault当前余额
    let vault_balance = ctx.accounts.asset_pool_vault.amount;

    // 3. 按比例计算用户应得金额（类似Junior的按比例分配）
    // actual_amount = vault_balance * user_grow_tokens / total_grow_supply
    let actual_amount = (vault_balance as u128)
        .checked_mul(amount as u128)
        .ok_or(PencilError::ArithmeticOverflow)?
        .checked_div(grow_total_supply as u128)
        .ok_or(PencilError::ArithmeticOverflow)? as u64;

    msg!(
        "Senior正常提取计算 - Vault余额: {}, GROW销毁: {}, GROW总量: {}, 实际金额: {}",
        vault_balance,
        amount,
        grow_total_supply,
        actual_amount
    );

    // 4. 销毁用户的 GROW Token
    let burn_cpi_accounts = Burn {
        mint: ctx.accounts.grow_token_mint.to_account_info(),
        from: ctx.accounts.user_grow_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let burn_cpi_program = ctx.accounts.token_program.to_account_info();
    let burn_cpi_ctx = CpiContext::new(burn_cpi_program, burn_cpi_accounts);
    token::burn(burn_cpi_ctx, amount)?;

    // 5. 准备 PDA 签名种子
    let asset_pool = &ctx.accounts.asset_pool;
    let asset_pool_seeds = &[
        seeds::ASSET_POOL,
        asset_pool.creator.as_ref(),
        &asset_pool.name,
        &[ctx.bumps.asset_pool],
    ];
    let signer_seeds = &[&asset_pool_seeds[..]];

    // 6. 转账按比例计算的金额给用户
    let transfer_cpi_accounts = Transfer {
        from: ctx.accounts.asset_pool_vault.to_account_info(),
        to: ctx.accounts.user_asset_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let transfer_cpi_program = ctx.accounts.token_program.to_account_info();
    let transfer_cpi_ctx =
        CpiContext::new_with_signer(transfer_cpi_program, transfer_cpi_accounts, signer_seeds);
    token::transfer(transfer_cpi_ctx, actual_amount)?;

    // 7. 更新SeniorPool的total_deposits
    let senior_pool = &mut ctx.accounts.senior_pool;
    senior_pool.total_deposits = senior_pool
        .total_deposits
        .checked_sub(actual_amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    msg!(
        "Senior 正常提取完成 - 用户: {}, GROW销毁: {}, 实际金额: {}",
        ctx.accounts.user.key(),
        amount,
        actual_amount
    );

    Ok(())
}
