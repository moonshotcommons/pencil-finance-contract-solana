use crate::constants::*;
use crate::errors::PencilError;
use crate::state::{AssetPool, JuniorNFTMetadata};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
pub struct MintGrowToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(mut)]
    pub grow_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = grow_token_mint,
        associated_token::authority = payer
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn mint_grow_token(ctx: Context<MintGrowToken>, amount: u64) -> Result<()> {
    require!(amount > 0, PencilError::InvalidSubscriptionAmount);

    let asset_pool = &ctx.accounts.asset_pool;

    // 检查资产池状态
    require!(
        asset_pool.status == asset_pool_status::FUNDED,
        PencilError::InvalidAssetPoolStatus
    );

    // Mint GROW Token
    let cpi_accounts = MintTo {
        mint: ctx.accounts.grow_token_mint.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.asset_pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, amount)?;

    msg!("GROW Token minted: {} tokens", amount);

    Ok(())
}

#[derive(Accounts)]
pub struct BurnGrowToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub grow_token_mint: Account<'info, Mint>,

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

    // Burn GROW Token
    let cpi_accounts = Burn {
        mint: ctx.accounts.grow_token_mint.to_account_info(),
        from: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::burn(cpi_ctx, amount)?;

    msg!("GROW Token burned: {} tokens", amount);

    Ok(())
}

#[derive(Accounts)]
#[instruction(nft_id: u64, principal: u64)]
pub struct MintJuniorNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub asset_pool: Account<'info, AssetPool>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<JuniorNFTMetadata>(),
        seeds = [seeds::JUNIOR_NFT_METADATA, asset_pool.key().as_ref()],
        bump
    )]
    pub nft_metadata: Account<'info, JuniorNFTMetadata>,

    pub system_program: Program<'info, System>,
}

pub fn mint_junior_nft(ctx: Context<MintJuniorNFT>, nft_id: u64, principal: u64) -> Result<()> {
    require!(principal > 0, PencilError::InvalidSubscriptionAmount);

    let nft_metadata = &mut ctx.accounts.nft_metadata;
    nft_metadata.nft_id = nft_id;
    nft_metadata.asset_pool = ctx.accounts.asset_pool.key();
    nft_metadata.owner = ctx.accounts.payer.key();
    nft_metadata.principal = principal;
    nft_metadata.claimed_interest = 0;
    nft_metadata.principal_withdrawn = false;
    nft_metadata.created_at = Clock::get()?.unix_timestamp;

    msg!("Junior NFT minted: ID {}, Principal {}", nft_id, principal);

    Ok(())
}
