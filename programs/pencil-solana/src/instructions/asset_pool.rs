use crate::constants::*;
use crate::errors::PencilError;
use crate::state::{
    AssetPool, AssetWhitelist, FirstLossPool, Funding, JuniorInterestPool, SeniorPool, SystemConfig,
};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

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
        seeds = [seeds::ASSET_WHITELIST],
        bump
    )]
    pub asset_whitelist: Account<'info, AssetWhitelist>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<AssetPool>() + 64,
        seeds = [seeds::ASSET_POOL, payer.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub asset_pool: Account<'info, AssetPool>,

    /// 资产代币地址
    pub asset_address: Account<'info, Mint>,

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
    // 检查系统是否暂停
    require!(
        !ctx.accounts.system_config.paused,
        PencilError::SystemPaused
    );

    // 验证资产在白名单中
    let asset_address = ctx.accounts.asset_address.key();
    require!(
        ctx.accounts.asset_whitelist.assets.contains(&asset_address),
        PencilError::AssetNotSupported
    );

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
    asset_pool.name = name.as_bytes().to_vec();
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

    let name_str = String::from_utf8_lossy(&asset_pool.name);
    msg!("Asset pool created: {}", name_str);
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

    let name_str = String::from_utf8_lossy(&asset_pool.name);
    msg!("Asset pool approved: {}", name_str);

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeRelatedAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [seeds::SYSTEM_CONFIG],
        bump
    )]
    pub system_config: Box<Account<'info, SystemConfig>>,

    #[account(
        mut,
        constraint = asset_pool.status == asset_pool_status::APPROVED @ PencilError::AssetPoolNotApproved,
        constraint = !asset_pool.related_accounts_initialized @ PencilError::RelatedAccountsAlreadyInitialized
    )]
    pub asset_pool: Box<Account<'info, AssetPool>>,

    /// 资产代币 Mint
    pub asset_mint: Box<Account<'info, Mint>>,

    /// Funding PDA 账户
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Funding>(),
        seeds = [seeds::FUNDING, asset_pool.key().as_ref()],
        bump
    )]
    pub funding: Box<Account<'info, Funding>>,

    /// SeniorPool PDA 账户
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<SeniorPool>(),
        seeds = [seeds::SENIOR_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub senior_pool: Box<Account<'info, SeniorPool>>,

    /// FirstLossPool PDA 账户
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<FirstLossPool>(),
        seeds = [seeds::FIRST_LOSS_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub first_loss_pool: Box<Account<'info, FirstLossPool>>,

    /// JuniorInterestPool PDA 账户
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<JuniorInterestPool>(),
        seeds = [seeds::JUNIOR_INTEREST_POOL, asset_pool.key().as_ref()],
        bump
    )]
    pub junior_interest_pool: Box<Account<'info, JuniorInterestPool>>,

    /// GROW Token Mint PDA
    #[account(
        init,
        payer = payer,
        mint::decimals = asset_mint.decimals,
        mint::authority = asset_pool,
        seeds = [seeds::GROW_TOKEN_MINT, asset_pool.key().as_ref()],
        bump
    )]
    pub grow_token_mint: Box<Account<'info, Mint>>,

    /// Junior NFT Mint PDA (用作基础 mint，实际 NFT 会有独立的 mint)
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = asset_pool,
        seeds = [seeds::JUNIOR_NFT_MINT, asset_pool.key().as_ref()],
        bump
    )]
    pub junior_nft_mint: Box<Account<'info, Mint>>,

    /// 资产池 Token Vault ATA
    #[account(
        init,
        payer = payer,
        associated_token::mint = asset_mint,
        associated_token::authority = asset_pool
    )]
    pub asset_pool_vault: Box<Account<'info, TokenAccount>>,

    /// 金库账户 (从 SystemConfig 读取)
    /// CHECK: This is the treasury account from SystemConfig
    #[account(
        constraint = treasury.key() == system_config.treasury @ PencilError::InvalidTreasuryAddress
    )]
    pub treasury: AccountInfo<'info>,

    /// 金库 ATA
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = asset_mint,
        associated_token::authority = treasury
    )]
    pub treasury_ata: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_related_accounts(ctx: Context<InitializeRelatedAccounts>) -> Result<()> {
    let asset_pool = &mut ctx.accounts.asset_pool;
    let funding = &mut ctx.accounts.funding;
    let senior_pool = &mut ctx.accounts.senior_pool;
    let first_loss_pool = &mut ctx.accounts.first_loss_pool;
    let junior_interest_pool = &mut ctx.accounts.junior_interest_pool;

    // 初始化 Funding 账户
    funding.asset_pool = asset_pool.key();
    funding.asset_address = ctx.accounts.asset_mint.key();
    funding.senior_total = 0;
    funding.junior_total = 0;
    funding.status = 0; // PENDING

    // 初始化 SeniorPool 账户
    senior_pool.asset_pool = asset_pool.key();
    senior_pool.grow_token = ctx.accounts.grow_token_mint.key();
    senior_pool.total_deposits = 0;
    senior_pool.repaid_amount = 0;

    // 初始化 FirstLossPool 账户
    first_loss_pool.asset_pool = asset_pool.key();
    first_loss_pool.junior_nft = ctx.accounts.junior_nft_mint.key();
    first_loss_pool.total_deposits = 0;
    first_loss_pool.repaid_amount = 0;

    // 初始化 JuniorInterestPool 账户
    junior_interest_pool.asset_pool = asset_pool.key();
    junior_interest_pool.junior_nft = ctx.accounts.junior_nft_mint.key();
    junior_interest_pool.total_interest = 0;
    junior_interest_pool.distributed_interest = 0;

    // 将所有账户地址写入 AssetPool
    asset_pool.funding = ctx.accounts.funding.key();
    asset_pool.senior_pool = ctx.accounts.senior_pool.key();
    asset_pool.first_loss_pool = ctx.accounts.first_loss_pool.key();
    asset_pool.junior_interest_pool = ctx.accounts.junior_interest_pool.key();
    asset_pool.grow_token = ctx.accounts.grow_token_mint.key();
    asset_pool.junior_nft = ctx.accounts.junior_nft_mint.key();
    asset_pool.treasury = ctx.accounts.system_config.treasury;
    asset_pool.asset_pool_vault = ctx.accounts.asset_pool_vault.key();
    asset_pool.treasury_ata = ctx.accounts.treasury_ata.key();
    asset_pool.related_accounts_initialized = true;

    let clock = Clock::get()?;

    // 发出事件
    emit!(crate::RelatedAccountsInitialized {
        asset_pool: asset_pool.key(),
        funding: ctx.accounts.funding.key(),
        senior_pool: ctx.accounts.senior_pool.key(),
        first_loss_pool: ctx.accounts.first_loss_pool.key(),
        junior_interest_pool: ctx.accounts.junior_interest_pool.key(),
        grow_token: ctx.accounts.grow_token_mint.key(),
        asset_pool_vault: ctx.accounts.asset_pool_vault.key(),
        treasury_ata: ctx.accounts.treasury_ata.key(),
        timestamp: clock.unix_timestamp,
    });

    let name_str = String::from_utf8_lossy(&asset_pool.name);
    msg!(
        "Related accounts initialized for asset pool: {}",
        name_str
    );
    msg!("Funding: {}", ctx.accounts.funding.key());
    msg!("Senior Pool: {}", ctx.accounts.senior_pool.key());
    msg!("First Loss Pool: {}", ctx.accounts.first_loss_pool.key());
    msg!(
        "Junior Interest Pool: {}",
        ctx.accounts.junior_interest_pool.key()
    );
    msg!("GROW Token: {}", ctx.accounts.grow_token_mint.key());
    msg!("Junior NFT: {}", ctx.accounts.junior_nft_mint.key());
    msg!("Asset Pool Vault: {}", ctx.accounts.asset_pool_vault.key());
    msg!("Treasury ATA: {}", ctx.accounts.treasury_ata.key());

    Ok(())
}
