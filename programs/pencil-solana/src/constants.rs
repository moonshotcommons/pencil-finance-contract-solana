/// 常量定义
/// Constants for the Pencil Solana program

/// 基点单位 (10000 = 100%)
pub const BASIS_POINTS: u16 = 10000;

/// 最大平台费用 (50%)
pub const MAX_PLATFORM_FEE: u16 = 5000;

/// 最大提前退出费用 (20%)
pub const MAX_EARLY_EXIT_FEE: u16 = 2000;

/// 最小 Junior 占比 (5%)
pub const MIN_JUNIOR_RATIO: u16 = 500;

/// 最大 Junior 占比 (50%)
pub const MAX_JUNIOR_RATIO: u16 = 5000;

/// 最大年利率 (100%)
pub const MAX_ANNUAL_RATE: u16 = 10000;

/// 最小募资期限 (1 天)
pub const MIN_FUNDING_PERIOD: i64 = 86400;

/// 最大募资期限 (365 天)
pub const MAX_FUNDING_PERIOD: i64 = 31536000;

/// 最小还款期限 (1 天)
pub const MIN_REPAYMENT_PERIOD: u64 = 1;

/// 最大还款期限 (365 天)
pub const MAX_REPAYMENT_PERIOD: u64 = 365;

/// 最大还款期数 (120 期)
pub const MAX_REPAYMENT_COUNT: u64 = 120;

/// 小数精度 (6 位，与 USDC 一致)
pub const DECIMALS: u8 = 6;

/// 小数倍数
pub const DECIMAL_MULTIPLIER: u64 = 1_000_000;

/// 资产池状态
pub mod asset_pool_status {
    pub const CREATED: u8 = 0;
    pub const APPROVED: u8 = 1;
    pub const FUNDING: u8 = 2;
    pub const FUNDED: u8 = 3;
    pub const REPAYING: u8 = 4;
    pub const COMPLETED: u8 = 5;
    pub const CANCELLED: u8 = 6;
}

/// 订阅状态
pub mod subscription_status {
    pub const PENDING: u8 = 0;
    pub const CONFIRMED: u8 = 1;
    pub const REFUNDED: u8 = 2;
}

/// 还款状态
pub mod repayment_status {
    pub const PENDING: u8 = 0;
    pub const COMPLETED: u8 = 1;
    pub const PARTIAL: u8 = 2;
}

/// PDA 种子
pub mod seeds {
    pub const SYSTEM_CONFIG: &[u8] = b"system_config";
    pub const ASSET_POOL: &[u8] = b"asset_pool";
    pub const FUNDING: &[u8] = b"funding";
    pub const SENIOR_POOL: &[u8] = b"senior_pool";
    pub const FIRST_LOSS_POOL: &[u8] = b"first_loss_pool";
    pub const JUNIOR_INTEREST_POOL: &[u8] = b"junior_interest_pool";
    pub const TREASURY: &[u8] = b"treasury";
    pub const GROW_TOKEN_MINT: &[u8] = b"grow_token_mint";
    pub const JUNIOR_NFT_MINT: &[u8] = b"junior_nft_mint";
    pub const SUBSCRIPTION: &[u8] = b"subscription";
    pub const REPAYMENT_RECORD: &[u8] = b"repayment_record";
    pub const JUNIOR_NFT_METADATA: &[u8] = b"junior_nft_metadata";
}

