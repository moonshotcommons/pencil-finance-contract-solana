# Solana-EVM Parity 设计文档

## 概述

本设计文档描述了如何在 Solana 程序中实现与 EVM 合约功能对等的系统。当前 Solana 程序已经实现了基础的资产池创建、认购和还款功能，但缺少以下关键功能：

1. 系统治理与权限管理（管理员角色更新、系统暂停/恢复）
2. 资产白名单管理
3. 工厂化账户初始化（一次性创建所有相关账户）
4. 募资完成后的代币分发（GROW Token 和 Junior NFT）
5. 募资失败退款机制
6. 还款资金的正确分配（平台费、Senior 本息、Junior 利息）
7. Senior 早退机制
8. Junior 利息领取和本金提取
9. 完整的 SPL 代币和 NFT 实现
10. 事件日志记录

## 架构设计

### 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                      SystemConfig (PDA)                      │
│  - 管理员角色管理                                              │
│  - 系统参数配置                                                │
│  - 资产白名单                                                  │
│  - 暂停状态                                                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ 引用
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      AssetPool (PDA)                         │
│  - 资产池核心状态                                              │
│  - 关联所有子账户                                              │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Funding    │    │  SeniorPool  │    │ FirstLossPool│
│    (PDA)     │    │    (PDA)     │    │    (PDA)     │
└──────────────┘    └──────────────┘    └──────────────┘
        │                     │                     │
        │                     │                     │
        ▼                     ▼                     ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│ GROW Token   │    │  Junior NFT  │    │JuniorInterest│
│  Mint (PDA)  │    │  Mint (PDA)  │    │  Pool (PDA)  │
└──────────────┘    └──────────────┘    └──────────────┘
```

### 账户关系

1. **SystemConfig**: 全局单例 PDA，管理系统级配置
2. **AssetPool**: 每个资产池一个 PDA，是所有子账户的中心
3. **Funding**: 管理募资流程
4. **SeniorPool**: 管理 Senior 投资者资金
5. **FirstLossPool**: 管理 Junior 投资者本金（首亏池）
6. **JuniorInterestPool**: 管理 Junior 投资者利息
7. **GROW Token Mint**: SPL Token Mint，代表 Senior 份额
8. **Junior NFT Mint**: SPL Token Mint（supply=1），代表 Junior 份额

## 组件设计

### 1. SystemConfig 增强

#### 新增状态字段

```rust
#[account]
pub struct SystemConfig {
    // 现有字段...

    /// 系统暂停状态
    pub paused: bool,

    /// 资产白名单（使用 Vec 存储支持的资产地址）
    pub supported_assets: Vec<Pubkey>,
}
```

#### 新增指令

1. **update_admin**: 更新管理员角色
   - 参数: `role: AdminRole`, `new_admin: Pubkey`
   - 权限: 仅超级管理员

2. **pause_system**: 暂停系统
   - 权限: 仅超级管理员

3. **unpause_system**: 恢复系统
   - 权限: 仅超级管理员

4. **update_fee_rate**: 更新费率参数
   - 参数: `fee_type: FeeType`, `new_rate: u16`
   - 权限: 仅系统管理员

5. **set_treasury**: 设置金库地址
   - 参数: `treasury: Pubkey`
   - 权限: 仅系统管理员

6. **set_asset_supported**: 设置资产支持状态
   - 参数: `asset: Pubkey`, `supported: bool`
   - 权限: 仅系统管理员

### 2. 工厂化初始化

#### 新增指令: initialize_related_accounts

一次性创建并初始化所有相关账户：

```rust
pub fn initialize_related_accounts(
    ctx: Context<InitializeRelatedAccounts>,
    asset_pool_name: String,
) -> Result<()>
```

**账户列表**:
- `asset_pool`: 已存在的 AssetPool 账户
- `funding`: 待创建的 Funding PDA
- `senior_pool`: 待创建的 SeniorPool PDA
- `first_loss_pool`: 待创建的 FirstLossPool PDA
- `junior_interest_pool`: 待创建的 JuniorInterestPool PDA
- `grow_token_mint`: 待创建的 GROW Token Mint PDA
- `junior_nft_mint`: 待创建的 Junior NFT Mint PDA
- `asset_pool_vault`: 待创建的资产池 Token Vault ATA
- `treasury_ata`: 待创建的金库 ATA

**执行流程**:
1. 验证 AssetPool 状态为 APPROVED
2. 创建所有 PDA 账户
3. 初始化 SPL Token Mints
4. 创建必要的 ATA
5. 将所有账户地址写入 AssetPool
6. 发出 RelatedAccountsInitialized 事件

### 3. 募资完成与代币分发

#### 增强 complete_funding 指令

```rust
pub fn complete_funding(
    ctx: Context<CompleteFunding>,
) -> Result<()>
```

**新增逻辑**:
1. 验证募资目标达成
2. 为所有 Senior 投资者铸造 GROW Token
3. 为所有 Junior 投资者铸造 NFT
4. 创建 JuniorNFTMetadata PDA 记录本金
5. 将代币转入投资者 ATA
6. 更新 AssetPool 状态为 FUNDED

**账户需求**:
- 需要遍历所有 Subscription 账户
- 使用 CPI 调用 SPL Token Program 铸造代币
- 使用 PDA 签名授权铸造

### 4. 募资失败退款

#### 新增指令: process_refund

```rust
pub fn process_refund(
    ctx: Context<ProcessRefund>,
) -> Result<()>
```

**执行流程**:
1. 验证募资失败（未达到 min_amount 或 Junior 占比不足）
2. 从资产池 Vault 转账至用户 ATA
3. 更新 Subscription 状态为 REFUNDED
4. 发出 RefundProcessed 事件

### 5. 还款资金分配

#### 增强 repay 指令

```rust
pub fn repay(
    ctx: Context<Repay>,
    amount: u64,
) -> Result<()>
```

**分配逻辑**:

```
还款金额
  │
  ├─> 1. 计算当前应还期数
  │
  ├─> 2. 计算并转账平台费至金库
  │      平台费 = (本期应还金额 × 平台费率)
  │
  ├─> 3. 计算并分配 Senior 应得本息
  │      Senior 本息 = Senior 本金 / 期数 + Senior 本金 × Senior 固定利率
  │      │
  │      ├─> 如果 SeniorPool 余额不足
  │      │   └─> 从 FirstLossPool 请求补足
  │      │
  │      └─> 转账至 SeniorPool
  │
  └─> 4. 剩余金额转至 JuniorInterestPool
         Junior 利息 = 还款金额 - 平台费 - Senior 本息
```

**关键计算**:
- 当前应还期数 = `(当前时间 - 募资结束时间) / 还款周期`
- 每期应还总额 = `总金额 / 期数 + 总金额 × 还款利率`
- 平台费 = `每期应还总额 × 平台费率`
- Senior 每期应得 = `Senior 金额 / 期数 + Senior 金额 × Senior 固定利率`

### 6. Senior 早退机制

#### 新增指令: early_exit_senior

```rust
pub fn early_exit_senior(
    ctx: Context<EarlyExitSenior>,
    amount: u64,
) -> Result<()>
```

**执行流程**:
1. 验证募资已完成
2. 根据时间计算早退费率
   - 募资结束前: `senior_early_before_exit_fee_rate`
   - 募资结束后: `senior_early_after_exit_fee_rate`
3. 销毁用户的 GROW Token
4. 计算净退款金额 = `amount - (amount × 早退费率)`
5. 转账早退费用至金库
6. 转账净退款金额至用户
7. 如果资产池 Vault 余额不足，从 FirstLossPool 请求补足
8. 发出 EarlyExitProcessed 事件

### 7. Junior 利息领取

#### 新增指令: claim_junior_interest

```rust
pub fn claim_junior_interest(
    ctx: Context<ClaimJuniorInterest>,
    nft_id: u64,
) -> Result<()>
```

**执行流程**:
1. 验证用户持有 Junior NFT
2. 从 JuniorInterestPool 计算可领取利息
3. 计算公式: `可领取 = (JuniorInterestPool 总额 × NFT 本金) / Junior 总本金 - 已领取`
4. 转账利息至用户 ATA
5. 更新 JuniorNFTMetadata 的已领取金额
6. 发出 InterestClaimed 事件

### 8. Junior 本金提取

#### 新增指令: withdraw_junior_principal

```rust
pub fn withdraw_junior_principal(
    ctx: Context<WithdrawJuniorPrincipal>,
    nft_id: u64,
) -> Result<()>
```

**执行流程**:
1. 验证 AssetPool 状态为 ENDED
2. 验证用户持有 Junior NFT
3. 验证用户未提取过本金
4. 从 JuniorNFTMetadata 读取本金金额
5. 从 FirstLossPool 转账本金至用户 ATA
6. 标记 JuniorNFTMetadata.principal_withdrawn = true
7. 发出 PrincipalWithdrawn 事件

### 9. SPL 代币与 NFT 实现

#### GROW Token (SPL Token)

```rust
// PDA Seeds: [b"grow_token_mint", asset_pool.key().as_ref()]
// Mint Authority: AssetPool PDA
// Decimals: 与资产代币一致（如 USDC 为 6）
```

**操作**:
- `mint_to`: 募资完成时铸造给 Senior 投资者
- `burn`: 早退时销毁

#### Junior NFT (SPL Token with supply=1)

```rust
// PDA Seeds: [b"junior_nft_mint", asset_pool.key().as_ref(), nft_id.to_le_bytes().as_ref()]
// Mint Authority: AssetPool PDA
// Supply: 1 (每个 NFT 唯一)
```

**元数据存储**:
```rust
#[account]
pub struct JuniorNFTMetadata {
    pub nft_id: u64,
    pub asset_pool: Pubkey,
    pub owner: Pubkey,
    pub principal: u64,           // 本金金额
    pub claimed_interest: u64,    // 已领取利息
    pub principal_withdrawn: bool, // 本金是否已提取
    pub created_at: i64,
}
```

### 10. 事件日志

#### 新增事件

```rust
#[event]
pub struct AdminUpdated {
    pub role: AdminRole,
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
}

#[event]
pub struct SystemPaused {
    pub timestamp: i64,
}

#[event]
pub struct SystemUnpaused {
    pub timestamp: i64,
}

#[event]
pub struct FeeRateUpdated {
    pub fee_type: FeeType,
    pub old_rate: u16,
    pub new_rate: u16,
}

#[event]
pub struct AssetSupportUpdated {
    pub asset: Pubkey,
    pub supported: bool,
}

#[event]
pub struct RelatedAccountsInitialized {
    pub asset_pool: Pubkey,
    pub funding: Pubkey,
    pub senior_pool: Pubkey,
    pub first_loss_pool: Pubkey,
    pub junior_interest_pool: Pubkey,
    pub grow_token: Pubkey,
    pub junior_nft: Pubkey,
}

#[event]
pub struct TokensDistributed {
    pub asset_pool: Pubkey,
    pub senior_amount: u64,
    pub junior_count: u64,
}

#[event]
pub struct RefundProcessed {
    pub user: Pubkey,
    pub amount: u64,
    pub subscription_type: u8,
}

#[event]
pub struct RepaymentDistributed {
    pub asset_pool: Pubkey,
    pub period: u64,
    pub platform_fee: u64,
    pub senior_amount: u64,
    pub junior_interest: u64,
}

#[event]
pub struct EarlyExitProcessed {
    pub user: Pubkey,
    pub amount: u64,
    pub fee: u64,
    pub net_amount: u64,
}

#[event]
pub struct InterestClaimed {
    pub user: Pubkey,
    pub nft_id: u64,
    pub amount: u64,
}

#[event]
pub struct PrincipalWithdrawn {
    pub user: Pubkey,
    pub nft_id: u64,
    pub amount: u64,
}
```

## 数据模型

### 新增/修改的账户结构

#### SystemConfig (修改)

```rust
#[account]
pub struct SystemConfig {
    pub super_admin: Pubkey,
    pub system_admin: Pubkey,
    pub treasury_admin: Pubkey,
    pub operation_admin: Pubkey,
    pub treasury: Pubkey,
    pub platform_fee_rate: u16,
    pub senior_early_before_exit_fee_rate: u16,
    pub senior_early_after_exit_fee_rate: u16,
    pub junior_early_before_exit_fee_rate: u16,
    pub default_min_junior_ratio: u16,
    pub paused: bool,                    // 新增
    pub initialized: bool,
    pub _reserved: [u8; 127],            // 调整预留空间
}
```

#### AssetWhitelist (新增)

```rust
#[account]
pub struct AssetWhitelist {
    pub system_config: Pubkey,
    pub assets: Vec<Pubkey>,  // 支持的资产列表
}
```

#### AssetPool (修改)

```rust
#[account]
pub struct AssetPool {
    // 现有字段...

    // 新增关联账户地址
    pub asset_pool_vault: Pubkey,  // 资产池 Token Vault
    pub treasury_ata: Pubkey,      // 金库 ATA

    // 新增状态字段
    pub related_accounts_initialized: bool,
}
```

## 错误处理

### 新增错误类型

```rust
#[error_code]
pub enum PencilError {
    // 现有错误...

    #[msg("System is paused")]
    SystemPaused,

    #[msg("Asset not in whitelist")]
    AssetNotSupported,

    #[msg("Related accounts already initialized")]
    RelatedAccountsAlreadyInitialized,

    #[msg("Related accounts not initialized")]
    RelatedAccountsNotInitialized,

    #[msg("Funding target not met")]
    FundingTargetNotMet,

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
```

## 安全考虑

### 1. 权限控制

- 所有管理员操作需要验证调用者角色
- 使用 PDA 作为程序权限，避免私钥泄露
- 关键操作需要多重签名（如更新超级管理员）

### 2. 资金安全

- 所有代币转账使用 CPI 调用 SPL Token Program
- 使用 PDA 签名授权，避免直接持有私钥
- 实现 ReentrancyGuard 防止重入攻击
- 金额计算使用 checked_add/checked_sub 防止溢出

### 3. 状态一致性

- 使用原子操作确保状态转换的一致性
- 关键操作失败时回滚状态
- 使用事件日志记录所有状态变更

### 4. 暂停机制

- 系统级暂停：阻止所有关键操作
- 资产池级暂停：仅阻止特定资产池操作
- 紧急情况下可快速暂停系统

## 测试策略

### 单元测试

1. **SystemConfig 测试**
   - 管理员角色更新
   - 系统暂停/恢复
   - 费率参数更新
   - 资产白名单管理

2. **工厂化初始化测试**
   - 一次性创建所有账户
   - PDA 地址正确性
   - 账户关联正确性

3. **募资流程测试**
   - 募资成功场景
   - 募资失败场景
   - 代币分发正确性
   - 退款流程

4. **还款分配测试**
   - 平台费计算
   - Senior 本息分配
   - Junior 利息分配
   - FirstLossPool 补足机制

5. **早退机制测试**
   - 早退费率计算
   - GROW Token 销毁
   - 资金转账
   - FirstLossPool 补足

6. **利息和本金提取测试**
   - 利息计算正确性
   - 本金提取时机
   - NFT 所有权验证

### 集成测试

1. **完整流程测试**
   - 创建资产池 → 初始化账户 → 募资 → 还款 → 提取

2. **异常场景测试**
   - 募资失败退款
   - 早退场景
   - 系统暂停场景

3. **并发测试**
   - 多用户同时认购
   - 多用户同时提取

### 性能测试

1. **Gas 消耗测试**
   - 各指令的计算单元消耗
   - 优化高频操作

2. **账户大小测试**
   - 验证账户大小足够
   - 预留空间合理性

## 部署计划

### 阶段 1: 基础设施（1-2 周）

- 实现 SystemConfig 增强功能
- 实现资产白名单管理
- 实现系统暂停机制
- 单元测试

### 阶段 2: 工厂化初始化（1 周）

- 实现 initialize_related_accounts 指令
- 实现 SPL Token Mint 创建
- 实现 ATA 创建
- 单元测试

### 阶段 3: 募资与分发（2 周）

- 增强 complete_funding 指令
- 实现 GROW Token 铸造
- 实现 Junior NFT 铸造
- 实现募资失败退款
- 单元测试

### 阶段 4: 还款分配（2 周）

- 增强 repay 指令
- 实现资金分配逻辑
- 实现 FirstLossPool 补足机制
- 单元测试

### 阶段 5: 早退与提取（2 周）

- 实现 Senior 早退机制
- 实现 Junior 利息领取
- 实现 Junior 本金提取
- 单元测试

### 阶段 6: 集成测试与优化（1-2 周）

- 完整流程集成测试
- 性能优化
- 安全审计
- 文档完善

## 与 EVM 版本的对比

| 功能 | EVM 实现 | Solana 实现 | 差异说明 |
|------|---------|------------|---------|
| 权限管理 | AccessControl 合约 | 账户验证 + PDA | Solana 使用账户验证替代角色系统 |
| 暂停机制 | Pausable 合约 | 状态字段 + 验证 | 功能相同，实现方式不同 |
| 工厂模式 | Factory 合约 | 单指令创建 | Solana 使用单指令替代工厂合约 |
| 代币标准 | ERC20 | SPL Token | 标准不同，功能对等 |
| NFT 标准 | ERC721 | SPL Token (supply=1) | 标准不同，功能对等 |
| 事件日志 | Event | Event | 功能相同 |
| 升级机制 | UUPS Proxy | 程序升级 | Solana 使用程序升级替代代理模式 |

## 总结

本设计文档详细描述了如何在 Solana 程序中实现与 EVM 合约功能对等的系统。主要改进包括：

1. 完善的系统治理和权限管理
2. 工厂化账户初始化简化部署流程
3. 完整的募资流程和代币分发机制
4. 精确的还款资金分配逻辑
5. 灵活的早退和提取机制
6. 标准的 SPL 代币和 NFT 实现
7. 完善的事件日志记录

通过这些改进，Solana 程序将实现与 EVM 版本的功能对等，同时保持 Solana 的高性能和低成本优势。
