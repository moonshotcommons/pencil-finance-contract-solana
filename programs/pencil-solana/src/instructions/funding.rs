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

    asset_pool.status = asset_pool_status::FUNDED;

    msg!("Funding completed");
    msg!("Senior amount: {}", asset_pool.senior_amount);
    msg!("Junior amount: {}", asset_pool.junior_amount);
    msg!("Junior ratio: {}%", junior_ratio / 100);

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
