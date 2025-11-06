use crate::constants::*;
use crate::errors::PencilError;
use crate::state::{AssetPool, Subscription};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct SubscribeSenior<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump
    )]
    pub system_config: Account<'info, crate::state::SystemConfig>,

    #[account(
        seeds = [seeds::ASSET_WHITELIST],
        bump
    )]
    pub asset_whitelist: Account<'info, crate::state::AssetWhitelist>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub asset_mint: Account<'info, anchor_spl::token::Mint>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<Subscription>(),
        seeds = [seeds::SUBSCRIPTION, asset_pool.key().as_ref(), user.key().as_ref(), b"senior"],
        bump
    )]
    pub subscription: Account<'info, Subscription>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn subscribe_senior(ctx: Context<SubscribeSenior>, amount: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidSubscriptionAmount);

    // 检查系统是否暂停
    require!(
        !ctx.accounts.system_config.paused,
        PencilError::SystemPaused
    );

    // 验证资产在白名单中
    let asset_address = ctx.accounts.asset_mint.key();
    require!(
        ctx.accounts.asset_whitelist.assets.contains(&asset_address),
        PencilError::AssetNotSupported
    );

    let asset_pool = &mut ctx.accounts.asset_pool;
    let clock = Clock::get()?;

    // 检查募资状态
    require!(
        clock.unix_timestamp >= asset_pool.funding_start_time,
        PencilError::FundingNotStarted
    );
    require!(
        clock.unix_timestamp <= asset_pool.funding_end_time,
        PencilError::FundingEnded
    );

    // 转账资产
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.pool_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // 记录订阅
    let subscription = &mut ctx.accounts.subscription;

    // 判断是否是第一次投资（通过检查 amount 是否为 0）
    if subscription.amount == 0 {
        // 第一次投资，初始化所有字段
        subscription.asset_pool = asset_pool.key();
        subscription.user = ctx.accounts.user.key();
        subscription.subscription_type = 0; // senior
        subscription.amount = amount;
        subscription.status = subscription_status::PENDING;
        subscription.subscribed_at = clock.unix_timestamp;
    } else {
        // 累加投资
        subscription.amount = subscription
            .amount
            .checked_add(amount)
            .ok_or(PencilError::ArithmeticOverflow)?;
    }

    // 更新资产池
    asset_pool.senior_amount = asset_pool
        .senior_amount
        .checked_add(amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    msg!("Senior subscription: {} tokens", amount);

    Ok(())
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct SubscribeJunior<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump
    )]
    pub system_config: Account<'info, crate::state::SystemConfig>,

    #[account(
        seeds = [seeds::ASSET_WHITELIST],
        bump
    )]
    pub asset_whitelist: Account<'info, crate::state::AssetWhitelist>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub asset_mint: Account<'info, anchor_spl::token::Mint>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<Subscription>(),
        seeds = [seeds::SUBSCRIPTION, asset_pool.key().as_ref(), user.key().as_ref(), b"junior"],
        bump
    )]
    pub subscription: Account<'info, Subscription>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn subscribe_junior(ctx: Context<SubscribeJunior>, amount: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidSubscriptionAmount);

    // 检查系统是否暂停
    require!(
        !ctx.accounts.system_config.paused,
        PencilError::SystemPaused
    );

    // 验证资产在白名单中
    let asset_address = ctx.accounts.asset_mint.key();
    require!(
        ctx.accounts.asset_whitelist.assets.contains(&asset_address),
        PencilError::AssetNotSupported
    );

    let asset_pool = &mut ctx.accounts.asset_pool;
    let clock = Clock::get()?;

    // 检查募资状态
    require!(
        clock.unix_timestamp >= asset_pool.funding_start_time,
        PencilError::FundingNotStarted
    );
    require!(
        clock.unix_timestamp <= asset_pool.funding_end_time,
        PencilError::FundingEnded
    );

    // 转账资产
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.pool_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // 记录订阅
    let subscription = &mut ctx.accounts.subscription;

    // 判断是否是第一次投资（通过检查 amount 是否为 0）
    if subscription.amount == 0 {
        // 第一次投资，初始化所有字段
        subscription.asset_pool = asset_pool.key();
        subscription.user = ctx.accounts.user.key();
        subscription.subscription_type = 1; // junior
        subscription.amount = amount;
        subscription.status = subscription_status::PENDING;
        subscription.subscribed_at = clock.unix_timestamp;
    } else {
        // 累加投资
        subscription.amount = subscription
            .amount
            .checked_add(amount)
            .ok_or(PencilError::ArithmeticOverflow)?;
    }

    // 更新资产池
    asset_pool.junior_amount = asset_pool
        .junior_amount
        .checked_add(amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    msg!("Junior subscription: {} tokens", amount);

    Ok(())
}

#[derive(Accounts)]
pub struct CompleteFunding<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        seeds = [seeds::SENIOR_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub senior_pool: Account<'info, crate::state::SeniorPool>,

    #[account(
        mut,
        seeds = [seeds::FIRST_LOSS_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub first_loss_pool: Account<'info, crate::state::FirstLossPool>,
}

pub fn complete_funding(ctx: Context<CompleteFunding>) -> Result<()> {
    let asset_pool = &mut ctx.accounts.asset_pool;
    let clock = Clock::get()?;

    // 检查募资是否已结束
    require!(
        clock.unix_timestamp > asset_pool.funding_end_time,
        PencilError::FundingNotCompleted
    );

    // 检查募资目标是否达成
    let total = asset_pool
        .senior_amount
        .checked_add(asset_pool.junior_amount)
        .ok_or(PencilError::ArithmeticOverflow)?;
    require!(
        total >= asset_pool.min_amount,
        PencilError::FundingMinimumNotMet
    );

    // 检查 Junior 占比
    let junior_ratio = (asset_pool.junior_amount * 10000) / total;
    require!(
        junior_ratio >= asset_pool.min_junior_ratio as u64,
        PencilError::InvalidJuniorRatio
    );

    // 更新 SeniorPool 和 FirstLossPool
    let senior_pool = &mut ctx.accounts.senior_pool;
    senior_pool.total_deposits = asset_pool.senior_amount;

    let first_loss_pool = &mut ctx.accounts.first_loss_pool;
    first_loss_pool.total_deposits = asset_pool.junior_amount;

    // 更新资产池状态
    asset_pool.status = asset_pool_status::FUNDED;

    msg!("Funding completed - ready for token distribution");
    msg!("Senior amount: {}", asset_pool.senior_amount);
    msg!("Junior amount: {}", asset_pool.junior_amount);
    msg!("Junior ratio: {}%", junior_ratio / 100);

    Ok(())
}

// 为 Senior 投资者分发 GROW Token
#[derive(Accounts)]
pub struct DistributeSeniorToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::ASSET_POOL, asset_pool.creator.as_ref(), asset_pool.name.as_bytes()],
        bump,
        constraint = asset_pool.status == asset_pool_status::FUNDED @ PencilError::InvalidAssetPoolStatus
    )]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        constraint = subscription.asset_pool == asset_pool.key() @ PencilError::InvalidAccount,
        constraint = subscription.subscription_type == 0 @ PencilError::InvalidSubscriptionStatus,
        constraint = subscription.status == subscription_status::PENDING @ PencilError::InvalidSubscriptionStatus
    )]
    pub subscription: Account<'info, Subscription>,

    #[account(
        mut,
        seeds = [seeds::GROW_TOKEN_MINT, asset_pool.key().as_ref()],
        bump
    )]
    pub grow_token_mint: Account<'info, anchor_spl::token::Mint>,

    /// CHECK: This account is validated by the subscription account
    pub user: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = grow_token_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn distribute_senior_token(ctx: Context<DistributeSeniorToken>) -> Result<()> {
    let subscription = &mut ctx.accounts.subscription;
    let asset_pool = &ctx.accounts.asset_pool;

    // 获取 asset_pool 的 PDA seeds 用于签名
    let asset_pool_seeds = &[
        seeds::ASSET_POOL,
        asset_pool.creator.as_ref(),
        asset_pool.name.as_bytes(),
        &[ctx.bumps.asset_pool],
    ];
    let asset_pool_signer = &[&asset_pool_seeds[..]];

    // 铸造 GROW Token
    let cpi_accounts = anchor_spl::token::MintTo {
        mint: ctx.accounts.grow_token_mint.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, asset_pool_signer);
    anchor_spl::token::mint_to(cpi_ctx, subscription.amount)?;

    // 更新订阅状态
    subscription.status = subscription_status::CONFIRMED;

    msg!(
        "Distributed {} GROW tokens to {}",
        subscription.amount,
        subscription.user
    );

    Ok(())
}

// 为 Junior 投资者分发 NFT
// 每个 Junior NFT 都有独立的 Mint 账户
// PDA seeds: [b"junior_nft_mint", asset_pool.key(), nft_id]
#[derive(Accounts)]
#[instruction(nft_id: u64)]
pub struct DistributeJuniorNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [seeds::ASSET_POOL, asset_pool.creator.as_ref(), asset_pool.name.as_bytes()],
        bump,
        constraint = asset_pool.status == asset_pool_status::FUNDED @ PencilError::InvalidAssetPoolStatus
    )]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        constraint = subscription.asset_pool == asset_pool.key() @ PencilError::InvalidAccount,
        constraint = subscription.subscription_type == 1 @ PencilError::InvalidSubscriptionStatus,
        constraint = subscription.status == subscription_status::PENDING @ PencilError::InvalidSubscriptionStatus
    )]
    pub subscription: Account<'info, Subscription>,

    /// Junior NFT Mint PDA - 每个 NFT 有独立的 Mint
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = asset_pool,
        seeds = [seeds::JUNIOR_NFT_MINT, asset_pool.key().as_ref(), nft_id.to_le_bytes().as_ref()],
        bump
    )]
    pub junior_nft_mint: Account<'info, anchor_spl::token::Mint>,

    /// CHECK: This account is validated by the subscription account
    pub user: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = junior_nft_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<crate::state::JuniorNFTMetadata>(),
        seeds = [seeds::JUNIOR_NFT_METADATA, asset_pool.key().as_ref(), nft_id.to_le_bytes().as_ref()],
        bump
    )]
    pub nft_metadata: Account<'info, crate::state::JuniorNFTMetadata>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn distribute_junior_nft(ctx: Context<DistributeJuniorNFT>, nft_id: u64) -> Result<()> {
    let subscription = &mut ctx.accounts.subscription;
    let asset_pool = &ctx.accounts.asset_pool;
    let clock = Clock::get()?;

    // 获取 asset_pool 的 PDA seeds 用于签名
    let asset_pool_seeds = &[
        seeds::ASSET_POOL,
        asset_pool.creator.as_ref(),
        asset_pool.name.as_bytes(),
        &[ctx.bumps.asset_pool],
    ];
    let asset_pool_signer = &[&asset_pool_seeds[..]];

    // 铸造 NFT (supply = 1) 使用 AssetPool PDA 作为 mint authority
    let cpi_accounts = anchor_spl::token::MintTo {
        mint: ctx.accounts.junior_nft_mint.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, asset_pool_signer);
    anchor_spl::token::mint_to(cpi_ctx, 1)?; // NFT supply = 1

    // 初始化 NFT 元数据
    let nft_metadata = &mut ctx.accounts.nft_metadata;
    nft_metadata.nft_id = nft_id;
    nft_metadata.asset_pool = asset_pool.key();
    nft_metadata.owner = subscription.user;
    nft_metadata.principal = subscription.amount;
    nft_metadata.claimed_interest = 0;
    nft_metadata.principal_withdrawn = false;
    nft_metadata.created_at = clock.unix_timestamp;

    // 更新订阅状态
    subscription.status = subscription_status::CONFIRMED;

    msg!(
        "Distributed Junior NFT {} to {} with principal {}",
        nft_id,
        subscription.user,
        subscription.amount
    );

    Ok(())
}

// 完成所有代币分发后发出事件
#[derive(Accounts)]
pub struct FinalizeTokenDistribution<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        constraint = asset_pool.status == asset_pool_status::FUNDED @ PencilError::InvalidAssetPoolStatus
    )]
    pub asset_pool: Account<'info, AssetPool>,
}

pub fn finalize_token_distribution(
    ctx: Context<FinalizeTokenDistribution>,
    senior_count: u64,
    junior_count: u64,
) -> Result<()> {
    let asset_pool = &ctx.accounts.asset_pool;
    let clock = Clock::get()?;

    // 发出事件
    emit!(crate::TokensDistributed {
        asset_pool: asset_pool.key(),
        senior_amount: asset_pool.senior_amount,
        junior_count,
        timestamp: clock.unix_timestamp,
    });

    msg!("Token distribution finalized");
    msg!("Senior investors: {}", senior_count);
    msg!("Junior investors: {}", junior_count);

    Ok(())
}

#[derive(Accounts)]
pub struct RefundSubscription<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(mut)]
    pub subscription: Account<'info, Subscription>,
}

pub fn refund_subscription(ctx: Context<RefundSubscription>, _subscription_type: u8) -> Result<()> {
    let subscription = &mut ctx.accounts.subscription;

    require!(
        subscription.status == subscription_status::PENDING,
        PencilError::RefundAlreadyProcessed
    );

    subscription.status = subscription_status::REFUNDED;

    msg!("Subscription refunded: {} tokens", subscription.amount);

    Ok(())
}

// 处理募资失败退款
#[derive(Accounts)]
pub struct ProcessRefund<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::ASSET_POOL, asset_pool.creator.as_ref(), asset_pool.name.as_bytes()],
        bump
    )]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        constraint = subscription.asset_pool == asset_pool.key() @ PencilError::InvalidAccount,
        constraint = subscription.user == user.key() @ PencilError::Unauthorized,
        constraint = subscription.status == subscription_status::PENDING @ PencilError::RefundAlreadyProcessed
    )]
    pub subscription: Account<'info, Subscription>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub pool_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub asset_mint: Account<'info, anchor_spl::token::Mint>,

    pub token_program: Program<'info, Token>,
}

pub fn process_refund(ctx: Context<ProcessRefund>) -> Result<()> {
    let clock = Clock::get()?;

    // 验证募资失败条件
    // 1. 募资时间已结束
    require!(
        clock.unix_timestamp > ctx.accounts.asset_pool.funding_end_time,
        PencilError::FundingNotCompleted
    );

    // 2. 检查是否达到最低募资目标或 Junior 占比不足
    let total = ctx
        .accounts
        .asset_pool
        .senior_amount
        .checked_add(ctx.accounts.asset_pool.junior_amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    let funding_failed = if total < ctx.accounts.asset_pool.min_amount {
        // 未达到最低募资目标
        true
    } else if ctx.accounts.asset_pool.junior_amount > 0 {
        // 检查 Junior 占比
        let junior_ratio = (ctx.accounts.asset_pool.junior_amount * 10000) / total;
        junior_ratio < ctx.accounts.asset_pool.min_junior_ratio as u64
    } else {
        // 没有 Junior 投资
        true
    };

    require!(funding_failed, PencilError::FundingTargetNotMet);

    // 验证退款金额
    let refund_amount = ctx.accounts.subscription.amount;
    require!(refund_amount > 0, PencilError::InvalidRefundAmount);

    // 验证 vault 余额充足
    require!(
        ctx.accounts.pool_vault.amount >= refund_amount,
        PencilError::InsufficientVaultBalance
    );

    // 从资产池 Vault 转账至用户 ATA
    let asset_pool_seeds = &[
        seeds::ASSET_POOL,
        ctx.accounts.asset_pool.creator.as_ref(),
        ctx.accounts.asset_pool.name.as_bytes(),
        &[ctx.bumps.asset_pool],
    ];
    let asset_pool_signer = &[&asset_pool_seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.pool_vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, asset_pool_signer);
    token::transfer(cpi_ctx, refund_amount)?;

    // 保存订阅类型和用户信息用于事件
    let subscription_type = ctx.accounts.subscription.subscription_type;
    let user = ctx.accounts.subscription.user;
    let asset_pool_key = ctx.accounts.asset_pool.key();

    // 更新 Subscription 状态为 REFUNDED
    ctx.accounts.subscription.status = subscription_status::REFUNDED;

    // 更新资产池的金额统计
    if subscription_type == 0 {
        // Senior
        ctx.accounts.asset_pool.senior_amount = ctx
            .accounts
            .asset_pool
            .senior_amount
            .checked_sub(refund_amount)
            .ok_or(PencilError::ArithmeticOverflow)?;
    } else {
        // Junior
        ctx.accounts.asset_pool.junior_amount = ctx
            .accounts
            .asset_pool
            .junior_amount
            .checked_sub(refund_amount)
            .ok_or(PencilError::ArithmeticOverflow)?;
    }

    // 发出 RefundProcessed 事件
    emit!(crate::RefundProcessed {
        asset_pool: asset_pool_key,
        user,
        amount: refund_amount,
        subscription_type,
        timestamp: clock.unix_timestamp,
    });

    msg!("Refund processed: {} tokens to {}", refund_amount, user);

    Ok(())
}

// 取消资产池（在所有认购都已退款后）
#[derive(Accounts)]
pub struct CancelAssetPool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::ASSET_POOL, asset_pool.creator.as_ref(), asset_pool.name.as_bytes()],
        bump,
        constraint = asset_pool.creator == authority.key() ||
                     asset_pool.system_config == authority.key() @ PencilError::Unauthorized
    )]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub pool_vault: Account<'info, TokenAccount>,

    pub asset_mint: Account<'info, anchor_spl::token::Mint>,
}

pub fn cancel_asset_pool(ctx: Context<CancelAssetPool>) -> Result<()> {
    let asset_pool = &mut ctx.accounts.asset_pool;
    let clock = Clock::get()?;

    // 验证募资时间已结束
    require!(
        clock.unix_timestamp > asset_pool.funding_end_time,
        PencilError::FundingNotCompleted
    );

    // 验证募资失败条件
    let total = asset_pool
        .senior_amount
        .checked_add(asset_pool.junior_amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    let funding_failed = if total < asset_pool.min_amount {
        true
    } else if asset_pool.junior_amount > 0 {
        let junior_ratio = (asset_pool.junior_amount * 10000) / total;
        junior_ratio < asset_pool.min_junior_ratio as u64
    } else {
        true
    };

    require!(funding_failed, PencilError::FundingTargetNotMet);

    // 验证所有认购都已退款（检查 vault 余额应该为 0 或接近 0）
    // 允许有微小的余额差异（由于精度问题）
    require!(
        ctx.accounts.pool_vault.amount == 0,
        PencilError::InsufficientVaultBalance
    );

    // 更新资产池状态为 CANCELLED
    asset_pool.status = asset_pool_status::CANCELLED;

    msg!("Asset pool cancelled: {}", asset_pool.name);
    msg!("All refunds have been processed");

    Ok(())
}

// 提前退出 - Senior 投资者
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct WithdrawSeniorSubscription<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump
    )]
    pub system_config: Account<'info, crate::state::SystemConfig>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        constraint = subscription.asset_pool == asset_pool.key() @ PencilError::InvalidAccount,
        constraint = subscription.subscription_type == 0 @ PencilError::InvalidSubscriptionStatus,
        constraint = subscription.user == user.key() @ PencilError::Unauthorized
    )]
    pub subscription: Account<'info, Subscription>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = treasury
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub asset_mint: Account<'info, anchor_spl::token::Mint>,

    /// CHECK: Treasury account from SystemConfig
    pub treasury: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw_senior_subscription(ctx: Context<WithdrawSeniorSubscription>, amount: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidSubscriptionAmount);
    require!(
        amount <= ctx.accounts.subscription.amount,
        PencilError::InsufficientBalance
    );

    // 检查系统是否暂停
    require!(
        !ctx.accounts.system_config.paused,
        PencilError::SystemPaused
    );

    // 检查募资状态（只能在募资期间提前退出）
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp <= ctx.accounts.asset_pool.funding_end_time,
        PencilError::FundingEnded
    );

    // 计算手续费
    let fee_rate = ctx.accounts.system_config.senior_early_before_exit_fee_rate;
    let fee = amount
        .checked_mul(fee_rate as u64)
        .ok_or(PencilError::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(PencilError::ArithmeticOverflow)?;
    let actual_amount = amount
        .checked_sub(fee)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 更新订阅金额
    ctx.accounts.subscription.amount = ctx.accounts.subscription.amount
        .checked_sub(amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 更新资产池 Senior 金额
    ctx.accounts.asset_pool.senior_amount = ctx.accounts.asset_pool.senior_amount
        .checked_sub(amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 转账手续费到金库
    if fee > 0 {
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_token_account.to_account_info(),
            to: ctx.accounts.treasury_ata.to_account_info(),
            authority: ctx.accounts.asset_pool.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, fee)?;
    }

    // 转账本金给用户
    if actual_amount > 0 {
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.asset_pool.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, actual_amount)?;
    }

    msg!("Senior subscription withdrawn: {} tokens, fee: {}", amount, fee);

    Ok(())
}

// 提前退出 - Junior 投资者
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct WithdrawJuniorSubscription<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump
    )]
    pub system_config: Account<'info, crate::state::SystemConfig>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        constraint = subscription.asset_pool == asset_pool.key() @ PencilError::InvalidAccount,
        constraint = subscription.subscription_type == 1 @ PencilError::InvalidSubscriptionStatus,
        constraint = subscription.user == user.key() @ PencilError::Unauthorized
    )]
    pub subscription: Account<'info, Subscription>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = treasury
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub asset_mint: Account<'info, anchor_spl::token::Mint>,

    /// CHECK: Treasury account from SystemConfig
    pub treasury: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw_junior_subscription(ctx: Context<WithdrawJuniorSubscription>, amount: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidSubscriptionAmount);
    require!(
        amount <= ctx.accounts.subscription.amount,
        PencilError::InsufficientBalance
    );

    // 检查系统是否暂停
    require!(
        !ctx.accounts.system_config.paused,
        PencilError::SystemPaused
    );

    // 检查募资状态（只能在募资期间提前退出）
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp <= ctx.accounts.asset_pool.funding_end_time,
        PencilError::FundingEnded
    );

    // 计算手续费
    let fee_rate = ctx.accounts.system_config.junior_early_before_exit_fee_rate;
    let fee = amount
        .checked_mul(fee_rate as u64)
        .ok_or(PencilError::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(PencilError::ArithmeticOverflow)?;
    let actual_amount = amount
        .checked_sub(fee)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 更新订阅金额
    ctx.accounts.subscription.amount = ctx.accounts.subscription.amount
        .checked_sub(amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 更新资产池 Junior 金额
    ctx.accounts.asset_pool.junior_amount = ctx.accounts.asset_pool.junior_amount
        .checked_sub(amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    // 转账手续费到金库
    if fee > 0 {
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_token_account.to_account_info(),
            to: ctx.accounts.treasury_ata.to_account_info(),
            authority: ctx.accounts.asset_pool.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, fee)?;
    }

    // 转账本金给用户
    if actual_amount > 0 {
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.asset_pool.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, actual_amount)?;
    }

    msg!("Junior subscription withdrawn: {} tokens, fee: {}", amount, fee);

    Ok(())
}
