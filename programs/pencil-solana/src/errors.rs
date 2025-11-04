use anchor_lang::prelude::*;

#[error_code]
pub enum PencilError {
    #[msg("Invalid system configuration")]
    InvalidSystemConfig,

    #[msg("Invalid asset pool")]
    InvalidAssetPool,

    #[msg("Invalid funding parameters")]
    InvalidFundingParams,

    #[msg("Funding not started")]
    FundingNotStarted,

    #[msg("Funding already ended")]
    FundingEnded,

    #[msg("Funding not completed")]
    FundingNotCompleted,

    #[msg("Invalid subscription amount")]
    InvalidSubscriptionAmount,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Insufficient senior amount")]
    InsufficientSeniorAmount,

    #[msg("Insufficient junior amount")]
    InsufficientJuniorAmount,

    #[msg("Invalid junior ratio")]
    InvalidJuniorRatio,

    #[msg("Invalid repayment amount")]
    InvalidRepaymentAmount,

    #[msg("Invalid repayment period")]
    InvalidRepaymentPeriod,

    #[msg("Invalid repayment count")]
    InvalidRepaymentCount,

    #[msg("Repayment not due")]
    RepaymentNotDue,

    #[msg("Repayment already completed")]
    RepaymentAlreadyCompleted,

    #[msg("Invalid early exit fee")]
    InvalidEarlyExitFee,

    #[msg("Invalid platform fee")]
    InvalidPlatformFee,

    #[msg("Invalid senior fixed rate")]
    InvalidSeniorFixedRate,

    #[msg("Invalid repayment rate")]
    InvalidRepaymentRate,

    #[msg("Invalid min junior ratio")]
    InvalidMinJuniorRatio,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid account")]
    InvalidAccount,

    #[msg("Invalid token account")]
    InvalidTokenAccount,

    #[msg("Invalid mint")]
    InvalidMint,

    #[msg("Transfer failed")]
    TransferFailed,

    #[msg("Mint failed")]
    MintFailed,

    #[msg("Burn failed")]
    BurnFailed,

    #[msg("Invalid NFT")]
    InvalidNFT,

    #[msg("NFT already claimed")]
    NFTAlreadyClaimed,

    #[msg("Invalid subscription status")]
    InvalidSubscriptionStatus,

    #[msg("Invalid asset pool status")]
    InvalidAssetPoolStatus,

    #[msg("Asset pool already approved")]
    AssetPoolAlreadyApproved,

    #[msg("Asset pool not approved")]
    AssetPoolNotApproved,

    #[msg("Funding target not met")]
    FundingTargetNotMet,

    #[msg("Funding minimum not met")]
    FundingMinimumNotMet,

    #[msg("Invalid time parameters")]
    InvalidTimeParameters,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Invalid pool address")]
    InvalidPoolAddress,

    #[msg("Invalid treasury address")]
    InvalidTreasuryAddress,

    #[msg("Insufficient funds in pool")]
    InsufficientPoolFunds,

    #[msg("Invalid interest calculation")]
    InvalidInterestCalculation,

    #[msg("Invalid principal calculation")]
    InvalidPrincipalCalculation,

    #[msg("Subscription not found")]
    SubscriptionNotFound,

    #[msg("Repayment record not found")]
    RepaymentRecordNotFound,

    #[msg("Invalid string length")]
    InvalidStringLength,

    #[msg("Duplicate subscription")]
    DuplicateSubscription,

    #[msg("Invalid refund amount")]
    InvalidRefundAmount,

    #[msg("Refund already processed")]
    RefundAlreadyProcessed,

    #[msg("Invalid claim amount")]
    InvalidClaimAmount,

    #[msg("No interest to claim")]
    NoInterestToClaim,

    #[msg("No principal to withdraw")]
    NoPrincipalToWithdraw,

    #[msg("Invalid early exit amount")]
    InvalidEarlyExitAmount,

    #[msg("Early exit not allowed")]
    EarlyExitNotAllowed,

    #[msg("Invalid token decimals")]
    InvalidTokenDecimals,

    #[msg("Precision loss in calculation")]
    PrecisionLoss,

    #[msg("System is paused")]
    SystemPaused,

    #[msg("Invalid admin role")]
    InvalidAdminRole,

    #[msg("Invalid fee type")]
    InvalidFeeType,

    #[msg("Asset not in whitelist")]
    AssetNotSupported,

    #[msg("Related accounts already initialized")]
    RelatedAccountsAlreadyInitialized,

    #[msg("Related accounts not initialized")]
    RelatedAccountsNotInitialized,

    #[msg("Insufficient vault balance")]
    InsufficientVaultBalance,

    #[msg("Invalid early exit timing")]
    InvalidEarlyExitTiming,

    #[msg("NFT not owned by user")]
    NFTNotOwnedByUser,

    #[msg("Principal already withdrawn")]
    PrincipalAlreadyWithdrawn,

    #[msg("Pool not ended")]
    PoolNotEnded,

    #[msg("Invalid period calculation")]
    InvalidPeriodCalculation,
}
