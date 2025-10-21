use crate::constants::*;
use crate::errors::PencilError;
use crate::state::{AssetPool, JuniorNFTMetadata, RepaymentRecord};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Repay<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = payer
    )]
    pub payer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_pool
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub asset_mint: Account<'info, anchor_spl::token::Mint>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<RepaymentRecord>(),
        seeds = [seeds::REPAYMENT_RECORD, asset_pool.key().as_ref()],
        bump
    )]
    pub repayment_record: Account<'info, RepaymentRecord>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidRepaymentAmount);

    let asset_pool = &mut ctx.accounts.asset_pool;

    // 检查资产池状态
    require!(
        asset_pool.status == asset_pool_status::FUNDED,
        PencilError::InvalidAssetPoolStatus
    );

    // 转账资产
    let cpi_accounts = Transfer {
        from: ctx.accounts.payer_token_account.to_account_info(),
        to: ctx.accounts.pool_token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // 更新还款记录
    let repayment_record = &mut ctx.accounts.repayment_record;
    repayment_record.asset_pool = asset_pool.key();
    repayment_record.amount = amount;
    repayment_record.repaid_at = Clock::get()?.unix_timestamp;
    repayment_record.status = repayment_status::COMPLETED;

    // 更新资产池
    asset_pool.repaid_amount = asset_pool
        .repaid_amount
        .checked_add(amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    msg!("Repayment received: {} tokens", amount);

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimInterest<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(mut)]
    pub nft_metadata: Account<'info, JuniorNFTMetadata>,

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

    pub token_program: Program<'info, Token>,
}

pub fn claim_interest(ctx: Context<ClaimInterest>, amount: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidClaimAmount);

    let nft_metadata = &mut ctx.accounts.nft_metadata;

    // 检查 NFT 所有者
    require!(
        nft_metadata.owner == ctx.accounts.user.key(),
        PencilError::Unauthorized
    );

    // 计算可领取的利息
    let total_interest = calculate_junior_interest(
        nft_metadata.principal,
        ctx.accounts.asset_pool.senior_fixed_rate,
        ctx.accounts.asset_pool.repayment_count,
    );
    let available_interest = total_interest
        .checked_sub(nft_metadata.claimed_interest)
        .ok_or(PencilError::ArithmeticOverflow)?;

    require!(
        amount <= available_interest,
        PencilError::InvalidClaimAmount
    );

    // 转账利息
    let cpi_accounts = Transfer {
        from: ctx.accounts.pool_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // 更新已领取利息
    nft_metadata.claimed_interest = nft_metadata
        .claimed_interest
        .checked_add(amount)
        .ok_or(PencilError::ArithmeticOverflow)?;

    msg!("Interest claimed: {} tokens", amount);

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawPrincipal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(mut)]
    pub nft_metadata: Account<'info, JuniorNFTMetadata>,

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

    pub token_program: Program<'info, Token>,
}

pub fn withdraw_principal(ctx: Context<WithdrawPrincipal>, _nft_id: u64) -> Result<()> {
    let nft_metadata = &mut ctx.accounts.nft_metadata;

    // 检查 NFT 所有者
    require!(
        nft_metadata.owner == ctx.accounts.user.key(),
        PencilError::Unauthorized
    );

    // 检查本金是否已提取
    require!(
        !nft_metadata.principal_withdrawn,
        PencilError::NoPrincipalToWithdraw
    );

    // 转账本金
    let cpi_accounts = Transfer {
        from: ctx.accounts.pool_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, nft_metadata.principal)?;

    // 标记本金已提取
    nft_metadata.principal_withdrawn = true;

    msg!("Principal withdrawn: {} tokens", nft_metadata.principal);

    Ok(())
}

#[derive(Accounts)]
pub struct EarlyExitSenior<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    pub system_program: Program<'info, System>,
}

pub fn early_exit_senior(ctx: Context<EarlyExitSenior>, _amount: u64) -> Result<()> {
    let _asset_pool = &ctx.accounts.asset_pool;

    // 这是一个占位符实现
    // 实际实现需要与 GROW Token 交互

    msg!("Early exit initiated");

    Ok(())
}

/// 计算 Junior 利息
fn calculate_junior_interest(principal: u64, rate: u16, periods: u64) -> u64 {
    let rate_decimal = (rate as u128) / 10000u128;
    let interest = ((principal as u128) * rate_decimal * (periods as u128)) / 365u128;
    interest as u64
}
