use crate::constants::*;
use crate::errors::PencilError;
use crate::state::{AssetPool, JuniorNFTMetadata};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};

/// GROW Token Mint 创建逻辑
/// PDA seeds: [b"grow_token_mint", asset_pool.key()]
/// Mint Authority: AssetPool PDA
/// Decimals: 与资产代币一致
///
/// 注意：GROW Token Mint 在 initialize_related_accounts 指令中创建
/// 本文件提供 mint_to 和 burn 操作

/// Mint GROW Token 给用户
/// 用于募资完成后分发代币
#[derive(Accounts)]
pub struct MintGrowToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [seeds::ASSET_POOL, asset_pool.creator.as_ref(), &asset_pool.name],
        bump,
        constraint = asset_pool.status == asset_pool_status::FUNDED @ PencilError::InvalidAssetPoolStatus
    )]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        seeds = [seeds::GROW_TOKEN_MINT, asset_pool.key().as_ref()],
        bump,
        constraint = grow_token_mint.key() == asset_pool.grow_token @ PencilError::InvalidAccount
    )]
    pub grow_token_mint: Account<'info, Mint>,

    /// CHECK: This is the recipient account
    pub recipient: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = grow_token_mint,
        associated_token::authority = recipient
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_grow_token(ctx: Context<MintGrowToken>, amount: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidSubscriptionAmount);

    let asset_pool = &ctx.accounts.asset_pool;

    // 获取 asset_pool 的 PDA seeds 用于签名
    let asset_pool_seeds = &[
        seeds::ASSET_POOL,
        asset_pool.creator.as_ref(),
        &asset_pool.name,
        &[ctx.bumps.asset_pool],
    ];
    let asset_pool_signer = &[&asset_pool_seeds[..]];

    // Mint GROW Token 使用 AssetPool PDA 作为 mint authority
    let cpi_accounts = MintTo {
        mint: ctx.accounts.grow_token_mint.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, asset_pool_signer);
    token::mint_to(cpi_ctx, amount)?;

    msg!(
        "GROW Token minted: {} tokens to {}",
        amount,
        ctx.accounts.recipient.key()
    );

    Ok(())
}

/// Burn GROW Token
/// 用于 Senior 早退时销毁代币
#[derive(Accounts)]
pub struct BurnGrowToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::GROW_TOKEN_MINT, asset_pool.key().as_ref()],
        bump
    )]
    pub grow_token_mint: Account<'info, Mint>,

    #[account(
        constraint = asset_pool.grow_token == grow_token_mint.key() @ PencilError::InvalidAccount
    )]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        mut,
        associated_token::mint = grow_token_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn burn_grow_token(ctx: Context<BurnGrowToken>, amount: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidSubscriptionAmount);

    // 验证用户有足够的代币
    require!(
        ctx.accounts.user_token_account.amount >= amount,
        PencilError::InsufficientBalance
    );

    // Burn GROW Token
    let cpi_accounts = Burn {
        mint: ctx.accounts.grow_token_mint.to_account_info(),
        from: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::burn(cpi_ctx, amount)?;

    msg!(
        "GROW Token burned: {} tokens from {}",
        amount,
        ctx.accounts.user.key()
    );

    Ok(())
}

/// Junior NFT Mint 创建逻辑
/// PDA seeds: [b"junior_nft_mint", asset_pool.key(), nft_id]
/// Mint Authority: AssetPool PDA
/// Supply: 1 (每个 NFT 唯一)
/// Decimals: 0 (NFT 不可分割)
///
/// 注意：每个 Junior NFT 都有独立的 Mint 账户
/// 在 distribute_junior_nft 指令中创建和铸造

/// Mint Junior NFT 给用户
/// 用于募资完成后分发 NFT
#[derive(Accounts)]
#[instruction(nft_id: u64)]
pub struct MintJuniorNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [seeds::ASSET_POOL, asset_pool.creator.as_ref(), &asset_pool.name],
        bump,
        constraint = asset_pool.status == asset_pool_status::FUNDED @ PencilError::InvalidAssetPoolStatus
    )]
    pub asset_pool: Account<'info, AssetPool>,

    /// Junior NFT Mint PDA - 每个 NFT 有独立的 Mint
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = asset_pool,
        seeds = [seeds::JUNIOR_NFT_MINT, asset_pool.key().as_ref(), nft_id.to_le_bytes().as_ref()],
        bump
    )]
    pub junior_nft_mint: Account<'info, Mint>,

    /// CHECK: This is the recipient account
    pub recipient: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = junior_nft_mint,
        associated_token::authority = recipient
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<JuniorNFTMetadata>(),
        seeds = [seeds::JUNIOR_NFT_METADATA, asset_pool.key().as_ref(), nft_id.to_le_bytes().as_ref()],
        bump
    )]
    pub nft_metadata: Account<'info, JuniorNFTMetadata>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_junior_nft(ctx: Context<MintJuniorNFT>, nft_id: u64, principal: u64) -> Result<()> {
    require!(principal > 0, PencilError::InvalidSubscriptionAmount);

    let asset_pool = &ctx.accounts.asset_pool;
    let clock = Clock::get()?;

    // 获取 asset_pool 的 PDA seeds 用于签名
    let asset_pool_seeds = &[
        seeds::ASSET_POOL,
        asset_pool.creator.as_ref(),
        &asset_pool.name,
        &[ctx.bumps.asset_pool],
    ];
    let asset_pool_signer = &[&asset_pool_seeds[..]];

    // Mint Junior NFT (supply = 1) 使用 AssetPool PDA 作为 mint authority
    let cpi_accounts = MintTo {
        mint: ctx.accounts.junior_nft_mint.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, asset_pool_signer);
    token::mint_to(cpi_ctx, 1)?; // NFT supply = 1

    // 初始化 NFT 元数据
    let nft_metadata = &mut ctx.accounts.nft_metadata;
    nft_metadata.nft_id = nft_id;
    nft_metadata.asset_pool = asset_pool.key();
    nft_metadata.owner = ctx.accounts.recipient.key();
    nft_metadata.principal = principal;
    nft_metadata.claimed_interest = 0;
    nft_metadata.principal_withdrawn = false;
    nft_metadata.created_at = clock.unix_timestamp;

    msg!(
        "Junior NFT minted: ID {}, Principal {} to {}",
        nft_id,
        principal,
        ctx.accounts.recipient.key()
    );

    Ok(())
}
