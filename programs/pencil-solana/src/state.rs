use anchor_lang::prelude::*;

/// 系统配置账户
#[account]
pub struct SystemConfig {
    /// 超级管理员
    pub super_admin: Pubkey,
    /// 系统管理员
    pub system_admin: Pubkey,
    /// 金库管理员
    pub treasury_admin: Pubkey,
    /// 运营管理员
    pub operation_admin: Pubkey,
    /// 金库地址
    pub treasury: Pubkey,
    /// 平台手续费率 (基点)
    pub platform_fee_rate: u16,
    /// 优先份额募资结束前提前退出手续费率
    pub senior_early_before_exit_fee_rate: u16,
    /// 优先份额募资结束后提前退出手续费率
    pub senior_early_after_exit_fee_rate: u16,
    /// 次级份额募资结束前提前退出手续费率
    pub junior_early_before_exit_fee_rate: u16,
    /// 默认最低 Junior 占比
    pub default_min_junior_ratio: u16,
    /// 是否已初始化
    pub initialized: bool,
    /// 预留空间
    pub _reserved: [u8; 128],
}

/// 资产池账户
#[account]
pub struct AssetPool {
    /// 资产池名称
    pub name: String,
    /// 资产池状态
    pub status: u8,
    /// 资产代币地址
    pub asset_address: Pubkey,
    /// 系统配置地址
    pub system_config: Pubkey,
    /// 募资合约地址
    pub funding: Pubkey,
    /// 优先池地址
    pub senior_pool: Pubkey,
    /// 首损池地址
    pub first_loss_pool: Pubkey,
    /// 利息池地址
    pub junior_interest_pool: Pubkey,
    /// GROW Token 地址
    pub grow_token: Pubkey,
    /// Junior NFT 地址
    pub junior_nft: Pubkey,
    /// 金库地址
    pub treasury: Pubkey,
    /// 平台手续费 (基点)
    pub platform_fee: u16,
    /// 优先份额募资结束前提前退出手续费
    pub senior_early_before_exit_fee: u16,
    /// 优先份额募资结束后提前退出手续费
    pub senior_early_after_exit_fee: u16,
    /// 次级份额募资结束前提前退出手续费
    pub junior_early_before_exit_fee: u16,
    /// 最低 Junior 占比
    pub min_junior_ratio: u16,
    /// 还款利率 (基点)
    pub repayment_rate: u16,
    /// 优先份额固定利率 (基点)
    pub senior_fixed_rate: u16,
    /// 还款周期 (天)
    pub repayment_period: u64,
    /// 还款期数
    pub repayment_count: u64,
    /// 预期募资总金额
    pub total_amount: u64,
    /// 最少募资目标
    pub min_amount: u64,
    /// 募资开始时间
    pub funding_start_time: i64,
    /// 募资结束时间
    pub funding_end_time: i64,
    /// 优先份额金额
    pub senior_amount: u64,
    /// 次级份额金额
    pub junior_amount: u64,
    /// 已还款金额
    pub repaid_amount: u64,
    /// 创建者
    pub creator: Pubkey,
    /// 创建时间
    pub created_at: i64,
    /// 预留空间
    pub _reserved: [u8; 128],
}

/// 募资账户
#[account]
pub struct Funding {
    /// 资产池地址
    pub asset_pool: Pubkey,
    /// 资产代币地址
    pub asset_address: Pubkey,
    /// 优先份额总认购金额
    pub senior_total: u64,
    /// 次级份额总认购金额
    pub junior_total: u64,
    /// 募资状态
    pub status: u8,
    /// 预留空间
    pub _reserved: [u8; 128],
}

/// 用户订阅记录
#[account]
pub struct Subscription {
    /// 资产池地址
    pub asset_pool: Pubkey,
    /// 用户地址
    pub user: Pubkey,
    /// 订阅类型 (0: senior, 1: junior)
    pub subscription_type: u8,
    /// 订阅金额
    pub amount: u64,
    /// 订阅状态
    pub status: u8,
    /// 订阅时间
    pub subscribed_at: i64,
    /// 预留空间
    pub _reserved: [u8; 128],
}

/// 优先池账户
#[account]
pub struct SeniorPool {
    /// 资产池地址
    pub asset_pool: Pubkey,
    /// GROW Token 地址
    pub grow_token: Pubkey,
    /// 总存款金额
    pub total_deposits: u64,
    /// 已还款金额
    pub repaid_amount: u64,
    /// 预留空间
    pub _reserved: [u8; 128],
}

/// 首损池账户
#[account]
pub struct FirstLossPool {
    /// 资产池地址
    pub asset_pool: Pubkey,
    /// Junior NFT 地址
    pub junior_nft: Pubkey,
    /// 总存款金额
    pub total_deposits: u64,
    /// 已还款金额
    pub repaid_amount: u64,
    /// 预留空间
    pub _reserved: [u8; 128],
}

/// 利息池账户
#[account]
pub struct JuniorInterestPool {
    /// 资产池地址
    pub asset_pool: Pubkey,
    /// Junior NFT 地址
    pub junior_nft: Pubkey,
    /// 总利息金额
    pub total_interest: u64,
    /// 已分配利息
    pub distributed_interest: u64,
    /// 预留空间
    pub _reserved: [u8; 128],
}

/// 还款记录
#[account]
pub struct RepaymentRecord {
    /// 资产池地址
    pub asset_pool: Pubkey,
    /// 还款期数
    pub period: u64,
    /// 还款金额
    pub amount: u64,
    /// 还款时间
    pub repaid_at: i64,
    /// 还款状态
    pub status: u8,
    /// 预留空间
    pub _reserved: [u8; 128],
}

/// Junior NFT 元数据
#[account]
pub struct JuniorNFTMetadata {
    /// NFT ID
    pub nft_id: u64,
    /// 资产池地址
    pub asset_pool: Pubkey,
    /// 所有者地址
    pub owner: Pubkey,
    /// 本金金额
    pub principal: u64,
    /// 已领取利息
    pub claimed_interest: u64,
    /// 是否已提取本金
    pub principal_withdrawn: bool,
    /// 创建时间
    pub created_at: i64,
    /// 预留空间
    pub _reserved: [u8; 128],
}

