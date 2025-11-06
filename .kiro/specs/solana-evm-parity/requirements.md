# Requirements Document

## Introduction

本文档定义了 Solana 程序与 EVM 合约功能对齐的需求。当前 Solana 程序在系统治理、工厂化初始化、资金流转、还款分配、早退机制和 NFT 实现等方面与 EVM 版本存在功能差距。本项目旨在补全这些缺失功能,实现两个版本的功能对等。

## Glossary

- **SystemConfig**: 系统配置账户,存储管理员角色、费率参数、暂停状态和资产白名单
- **AssetPool**: 资产池账户,管理借贷项目的核心状态和参数
- **Funding**: 募资账户,管理 Senior 和 Junior 认购流程
- **SeniorPool**: 优先级资金池,管理 Senior 投资者的资金和 GROW 代币
- **FirstLossPool**: 首亏池,用于覆盖 Senior 资金缺口
- **JuniorInterestPool**: Junior 利息池,管理 Junior 投资者的利息分配
- **GROW Token**: SPL 代币,代表 Senior 投资者的份额
- **Junior NFT**: SPL NFT,代表 Junior 投资者的认购凭证
- **Treasury**: 金库账户,接收平台费用
- **PDA**: Program Derived Address,程序派生地址
- **ATA**: Associated Token Account,关联代币账户
- **SPL**: Solana Program Library,Solana 程序库标准

## Requirements

### Requirement 1: 系统治理与权限管理

**User Story:** 作为系统管理员,我希望能够管理多角色权限、暂停系统和配置参数,以便灵活控制系统运行和安全。

#### Acceptance Criteria

1. WHEN 超级管理员调用更新管理员指令时,THE SystemConfig SHALL 更新对应角色的管理员地址
2. WHEN 超级管理员调用暂停系统指令时,THE SystemConfig SHALL 将暂停状态设置为 true
3. WHEN 超级管理员调用恢复系统指令时,THE SystemConfig SHALL 将暂停状态设置为 false
4. WHEN 系统管理员调用更新费率指令时,THE SystemConfig SHALL 更新对应的费率参数
5. WHEN 系统管理员调用设置金库指令时,THE SystemConfig SHALL 更新金库地址

### Requirement 2: 资产白名单管理

**User Story:** 作为系统管理员,我希望能够管理支持的资产白名单,以便控制哪些代币可以用于创建资产池。

#### Acceptance Criteria

1. WHEN 系统管理员调用设置资产支持状态指令时,THE SystemConfig SHALL 更新指定资产的支持状态
2. WHEN 用户尝试创建资产池时,THE System SHALL 验证资产在白名单中
3. WHEN 用户尝试认购时,THE System SHALL 验证资产在白名单中
4. WHEN 用户尝试还款时,THE System SHALL 验证资产在白名单中

### Requirement 3: 系统暂停状态校验

**User Story:** 作为系统,我希望在暂停状态下阻止关键操作,以便在紧急情况下保护用户资金。

#### Acceptance Criteria

1. WHEN 系统处于暂停状态且用户尝试创建资产池时,THE System SHALL 拒绝该操作
2. WHEN 系统处于暂停状态且用户尝试认购时,THE System SHALL 拒绝该操作
3. WHEN 系统处于暂停状态且用户尝试还款时,THE System SHALL 拒绝该操作
4. WHEN 系统处于暂停状态且用户尝试提取时,THE System SHALL 拒绝该操作
5. WHEN 系统处于暂停状态且用户尝试早退时,THE System SHALL 拒绝该操作

### Requirement 4: 工厂化账户初始化

**User Story:** 作为资产池创建者,我希望系统能够一次性初始化所有相关账户,以便简化部署流程并确保账户关联正确。

#### Acceptance Criteria

1. WHEN 用户调用初始化相关账户指令时,THE System SHALL 创建 Funding PDA 账户
2. WHEN 用户调用初始化相关账户指令时,THE System SHALL 创建 SeniorPool PDA 账户
3. WHEN 用户调用初始化相关账户指令时,THE System SHALL 创建 FirstLossPool PDA 账户
4. WHEN 用户调用初始化相关账户指令时,THE System SHALL 创建 JuniorInterestPool PDA 账户
5. WHEN 用户调用初始化相关账户指令时,THE System SHALL 创建 GROW Token SPL Mint
6. WHEN 用户调用初始化相关账户指令时,THE System SHALL 创建 Junior NFT SPL Mint
7. WHEN 用户调用初始化相关账户指令时,THE System SHALL 创建资产池 Token Vault ATA
8. WHEN 用户调用初始化相关账户指令时,THE System SHALL 创建金库 ATA
9. WHEN 初始化完成后,THE System SHALL 将所有账户地址写入 AssetPool

### Requirement 5: 募资完成与代币分发

**User Story:** 作为投资者,我希望在募资成功后自动获得相应的代币或 NFT,以便证明我的投资份额。

#### Acceptance Criteria

1. WHEN 募资达到目标且调用完成募资指令时,THE System SHALL 为 Senior 投资者铸造 GROW 代币
2. WHEN 募资达到目标且调用完成募资指令时,THE System SHALL 为 Junior 投资者铸造 NFT
3. WHEN 募资达到目标且调用完成募资指令时,THE System SHALL 将 AssetPool 状态更新为 FUNDED
4. WHEN 募资达到目标且调用完成募资指令时,THE System SHALL 在 JuniorNFTMetadata 中记录 Junior 认购本金
5. WHEN 募资达到目标且调用完成募资指令时,THE System SHALL 将 GROW 代币转入 Senior 投资者的 ATA

### Requirement 6: 募资失败退款

**User Story:** 作为投资者,我希望在募资失败时能够收到全额退款,以便收回我的资金。

#### Acceptance Criteria

1. WHEN 募资失败且用户调用退款指令时,THE System SHALL 从资产池 Vault 转账至用户 ATA
2. WHEN 募资失败且用户调用退款指令时,THE System SHALL 转账金额等于用户的认购金额
3. WHEN 募资失败且用户调用退款指令时,THE System SHALL 将认购记录状态更新为 REFUNDED
4. WHEN 所有认购都已退款后,THE System SHALL 允许将 AssetPool 状态更新为 CANCELLED

### Requirement 7: 还款资金分配

**User Story:** 作为系统,我希望能够正确分配还款资金到平台费、Senior 本息和 Junior 利息,以便确保各方权益。

#### Acceptance Criteria

1. WHEN 借款人还款时,THE System SHALL 计算当前应还期数
2. WHEN 借款人还款时,THE System SHALL 计算并转账平台费至金库 ATA
3. WHEN 借款人还款时,THE System SHALL 计算并分配 Senior 应得本息至 SeniorPool
4. WHEN 借款人还款时,THE System SHALL 计算并分配剩余金额至 JuniorInterestPool
5. WHEN 借款人还款时,THE System SHALL 创建 RepaymentRecord 并记录期数
6. WHEN Senior 应得金额不足且 FirstLossPool 有余额时,THE System SHALL 从 FirstLossPool 补足差额

### Requirement 8: Senior 早退机制

**User Story:** 作为 Senior 投资者,我希望能够提前退出并支付相应费用,以便在需要时提前收回资金。

#### Acceptance Criteria

1. WHEN Senior 投资者调用早退指令时,THE System SHALL 验证募资已完成
2. WHEN Senior 投资者调用早退指令时,THE System SHALL 根据时间计算早退费率
3. WHEN Senior 投资者调用早退指令时,THE System SHALL 销毁用户的 GROW 代币
4. WHEN Senior 投资者调用早退指令时,THE System SHALL 计算净退款金额
5. WHEN Senior 投资者调用早退指令时,THE System SHALL 转账早退费用至金库 ATA
6. WHEN Senior 投资者调用早退指令时,THE System SHALL 转账净退款金额至用户 ATA
7. IF 资产池 Vault 余额不足时,THEN THE System SHALL 从 FirstLossPool 请求补足

### Requirement 9: Junior 利息领取

**User Story:** 作为 Junior 投资者,我希望能够领取基于实际还款的利息,以便获得投资收益。

#### Acceptance Criteria

1. WHEN Junior 投资者调用领取利息指令时,THE System SHALL 验证用户持有 Junior NFT
2. WHEN Junior 投资者调用领取利息指令时,THE System SHALL 从 JuniorInterestPool 计算可领取利息
3. WHEN Junior 投资者调用领取利息指令时,THE System SHALL 转账利息至用户 ATA
4. WHEN Junior 投资者调用领取利息指令时,THE System SHALL 更新 JuniorInterestPool 的已领取金额
5. WHEN Junior 投资者调用领取利息指令时,THE System SHALL 记录领取历史

### Requirement 10: Junior 本金提取

**User Story:** 作为 Junior 投资者,我希望在项目结束后能够提取本金,以便收回投资。

#### Acceptance Criteria

1. WHEN Junior 投资者调用提取本金指令时,THE System SHALL 验证 AssetPool 状态为 ENDED
2. WHEN Junior 投资者调用提取本金指令时,THE System SHALL 验证用户持有 Junior NFT
3. WHEN Junior 投资者调用提取本金指令时,THE System SHALL 验证用户未提取过本金
4. WHEN Junior 投资者调用提取本金指令时,THE System SHALL 从 JuniorNFTMetadata 读取本金金额
5. WHEN Junior 投资者调用提取本金指令时,THE System SHALL 转账本金至用户 ATA
6. WHEN Junior 投资者调用提取本金指令时,THE System SHALL 标记本金已提取

### Requirement 11: SPL 代币与 NFT 实现

**User Story:** 作为开发者,我希望使用标准 SPL 代币和 NFT,以便与 Solana 生态兼容。

#### Acceptance Criteria

1. WHEN 系统初始化 GROW Token 时,THE System SHALL 创建标准 SPL Mint
2. WHEN 系统初始化 Junior NFT 时,THE System SHALL 创建标准 SPL Mint
3. WHEN 系统铸造 GROW Token 时,THE System SHALL 使用 SPL Token Program 铸造指令
4. WHEN 系统铸造 Junior NFT 时,THE System SHALL 使用 SPL Token Program 铸造指令
5. WHEN 系统销毁 GROW Token 时,THE System SHALL 使用 SPL Token Program 销毁指令
6. WHEN 系统需要存储 Junior 本金信息时,THE System SHALL 使用 JuniorNFTMetadata PDA 账户

### Requirement 12: 事件日志记录

**User Story:** 作为开发者和审计员,我希望系统记录关键操作的事件日志,以便追踪和审计。

#### Acceptance Criteria

1. WHEN 系统执行管理员更新时,THE System SHALL 发出事件日志
2. WHEN 系统执行暂停或恢复时,THE System SHALL 发出事件日志
3. WHEN 系统执行费率更新时,THE System SHALL 发出事件日志
4. WHEN 系统执行资产池创建时,THE System SHALL 发出事件日志
5. WHEN 系统执行认购时,THE System SHALL 发出事件日志
6. WHEN 系统执行还款时,THE System SHALL 发出事件日志
7. WHEN 系统执行早退时,THE System SHALL 发出事件日志
8. WHEN 系统执行利息领取时,THE System SHALL 发出事件日志
