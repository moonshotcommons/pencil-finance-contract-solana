# Pencil Solana 部署和配置脚本

本目录包含 Solana 版本的部署和配置脚本，等效于 EVM 版本的 `evm/scripts/deployToEDUChain.js`。

## 📁 脚本文件

### 1. `deploy.ts` - 部署和初始化脚本

完整的部署流程脚本，包括：
- 初始化 SystemConfig
- 创建示例资产池
- 批准资产池
- 验证部署结果
- 生成部署报告

### 2. `configure.ts` - 配置管理脚本

系统配置查询和管理工具，用于：
- 查看当前系统配置
- 查看管理员角色
- 查看资产池信息
- （未来）更新配置参数

## 🚀 使用方法

### 前置条件

1. **安装依赖**
   ```bash
   yarn install
   # 或
   npm install
   ```

2. **配置 Solana CLI**
   ```bash
   # 设置网络为 Devnet
   solana config set --url devnet

   # 查看当前配置
   solana config get

   # 查看账户余额
   solana balance

   # 如果余额不足，申请空投
   solana airdrop 2
   ```

3. **配置 Anchor**

   确保 `Anchor.toml` 中的配置正确：
   ```toml
   [provider]
   cluster = "Devnet"
   wallet = "~/.config/solana/id.json"
   ```

### 部署流程

#### 步骤 0: 配置网络（重要！）

**配置 Solana CLI：**
```bash
# 设置网络为 Devnet
solana config set --url devnet

# 查看当前配置
solana config get

# 查看账户余额
solana balance

# 如果余额不足，申请空投（仅 Devnet/Testnet）
solana airdrop 2
```

**配置 Anchor.toml：**

确保 `Anchor.toml` 中的网络配置正确：

```toml
[provider]
cluster = "Devnet"  # 可选: "Localnet", "Devnet", "Testnet", "Mainnet"
wallet = "~/.config/solana/id.json"
```

⚠️ **重要提示：**
- 部署脚本会自动读取 `Anchor.toml` 中的 `cluster` 和 `wallet` 配置
- 无需设置 `ANCHOR_PROVIDER_URL` 或 `ANCHOR_WALLET` 环境变量
- 确保 Solana CLI 和 Anchor.toml 的网络设置一致
- 不同网络需要不同的 SOL 余额
- 脚本会自动将 `~` 扩展为用户主目录路径

#### 步骤 1: 构建程序

```bash
anchor build
```

#### 步骤 2: 部署程序

```bash
# 部署到 Anchor.toml 中配置的网络
anchor deploy

# 或明确指定网络
anchor deploy --provider.cluster devnet
```

这将部署程序到 Solana 网络并输出程序 ID。

#### 步骤 3: 运行初始化脚本

```bash
# 方式 1: 使用 Anchor 脚本（推荐）
anchor run deploy

# 方式 2: 使用 npm/yarn（自动读取 Anchor.toml）
npm run deploy:solana
# 或
yarn deploy:solana

# 方式 3: 直接运行 TypeScript（自动读取 Anchor.toml）
ts-node scripts/deploy.ts
# 或使用 bun
bun run scripts/deploy.ts
```

**注意：** 所有方式都会自动从 `Anchor.toml` 读取网络配置，无需设置环境变量。

**脚本执行内容：**
1. ✅ 初始化 SystemConfig（设置费率、管理员等）
2. ✅ 创建示例资产池
3. ✅ 批准资产池
4. ✅ 验证部署结果
5. ✅ 生成 `deployment-solana.json` 文件

**输出示例：**
```
🚀 开始部署 Pencil Solana 程序...
============================================================
📍 网络: https://api.devnet.solana.com
👤 部署账户: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
🆔 程序 ID: RXo7Ai9ugeBp9giAKqera2pg1xMj49exA5SgWdMBMuM
💰 账户余额: 2.5000 SOL
============================================================

⚙️ 1. 初始化 SystemConfig...
⏳ 初始化 SystemConfig...
✅ 初始化 SystemConfig 完成
  💰 费用: 0.000005 SOL
  🔗 交易: https://explorer.solana.com/tx/...?cluster=devnet
✅ SystemConfig PDA: 8xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU

📦 2. 创建示例资产池...
...
```

#### 步骤 4: 查看配置

```bash
# 方式 1: 使用 Anchor 脚本（推荐）
anchor run configure

# 方式 2: 使用 npm/yarn（自动读取 Anchor.toml）
npm run configure:solana
# 或
yarn configure:solana

# 方式 3: 直接运行 TypeScript（自动读取 Anchor.toml）
ts-node scripts/configure.ts
# 或使用 bun
bun run scripts/configure.ts
```

**注意：** 所有方式都会自动从 `Anchor.toml` 读取网络配置，无需设置环境变量。

**输出示例：**
```
🔧 Pencil Solana 配置管理工具
============================================================
📍 网络: Devnet
🆔 程序 ID: RXo7Ai9ugeBp9giAKqera2pg1xMj49exA5SgWdMBMuM
============================================================
👤 当前账户: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU

📊 当前系统配置:
------------------------------------------------------------
平台费率: 500 (5%)
优先份额提前退出费（募资前）: 100 (1%)
优先份额提前退出费（募资后）: 200 (2%)
劣后份额提前退出费: 300 (3%)
默认最低 Junior 占比: 1000 (10%)

👥 管理员角色:
Super Admin: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
System Admin: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
Treasury Admin: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
Operation Admin: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
Treasury: 9yKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU

初始化状态: ✅ 已初始化
```

## 📊 部署信息文件

部署完成后，会生成 `deployment-solana.json` 文件，包含：

```json
{
  "network": "Devnet",
  "programId": "RXo7Ai9ugeBp9giAKqera2pg1xMj49exA5SgWdMBMuM",
  "deployedAt": "2025-10-22T10:30:00.000Z",
  "deployer": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
  "accounts": {
    "systemConfig": "8xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "treasury": "9yKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "assetPool": "AxKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "assetAddress": "BxKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"
  },
  "config": {
    "platformFeeRate": 500,
    "seniorEarlyBeforeExitFeeRate": 100,
    "seniorEarlyAfterExitFeeRate": 200,
    "juniorEarlyBeforeExitFeeRate": 300,
    "defaultMinJuniorRatio": 1000
  },
  "transactions": [...]
}
```

## 🔧 配置参数说明

### 费率参数（基点，10000 = 100%）

| 参数 | 默认值 | 说明 |
|------|--------|------|
| `platformFeeRate` | 500 | 平台费率（5%） |
| `seniorEarlyBeforeExitFeeRate` | 100 | 优先份额募资前提前退出费（1%） |
| `seniorEarlyAfterExitFeeRate` | 200 | 优先份额募资后提前退出费（2%） |
| `juniorEarlyBeforeExitFeeRate` | 300 | 劣后份额提前退出费（3%） |
| `defaultMinJuniorRatio` | 1000 | 默认最低 Junior 占比（10%） |

### 资产池参数

| 参数 | 示例值 | 说明 |
|------|--------|------|
| `name` | "Demo Asset Pool" | 资产池名称 |
| `total_amount` | 1,000,000 USDC | 总募资金额 |
| `min_amount` | 100,000 USDC | 最低募资目标 |
| `senior_fixed_rate` | 800 (8%) | 优先份额固定利率 |
| `repayment_period` | 30 天 | 还款周期 |
| `repayment_count` | 12 期 | 还款期数 |
| `funding_start_time` | 当前时间 + 1分钟 | 募资开始时间 |
| `funding_end_time` | 当前时间 + 1天 | 募资结束时间 |

## 🌐 网络配置对照表

| 网络 | Solana CLI URL | Anchor.toml cluster | 用途 | 获取 SOL |
|------|----------------|---------------------|------|----------|
| **Localnet** | `http://localhost:8899` | `"Localnet"` | 本地开发测试 | `solana airdrop 100` |
| **Devnet** | `https://api.devnet.solana.com` | `"Devnet"` | 开发和集成测试 | `solana airdrop 2` |
| **Testnet** | `https://api.testnet.solana.com` | `"Testnet"` | 预生产测试 | `solana airdrop 1` |
| **Mainnet** | `https://api.mainnet-beta.solana.com` | `"Mainnet"` | 生产环境 | 购买真实 SOL |

**配置示例：**

```bash
# Devnet（推荐用于开发）
solana config set --url devnet
# 在 Anchor.toml 中设置: cluster = "Devnet"

# Testnet（用于预生产测试）
solana config set --url testnet
# 在 Anchor.toml 中设置: cluster = "Testnet"

# Mainnet（生产环境，谨慎使用）
solana config set --url mainnet-beta
# 在 Anchor.toml 中设置: cluster = "Mainnet"
```

## 🔗 与 EVM 版本的对比

| 功能 | EVM 版本 | Solana 版本 |
|------|----------|-------------|
| 部署方式 | Hardhat deploy | Anchor deploy |
| 初始化 | 构造函数 | initialize 指令 |
| 代理模式 | ERC1967 Proxy | 无需代理（程序可升级） |
| 工厂模式 | Factory 合约 | 直接调用程序指令 |
| 代币 | ERC20/ERC721 | SPL Token/NFT |
| 验证 | Blockscout | Solana Explorer |
| Gas 费用 | ETH/EDU | SOL |
| 网络配置 | hardhat.config.js | Anchor.toml + Solana CLI |

## 📝 添加到 package.json

建议在 `package.json` 中添加以下脚本：

```json
{
  "scripts": {
    "deploy:solana": "ts-node scripts/deploy.ts",
    "configure:solana": "ts-node scripts/configure.ts",
    "build:solana": "anchor build",
    "test:solana": "anchor test"
  }
}
```

## ⚠️ 注意事项

1. **网络选择**
   - Devnet: 用于开发和测试
   - Testnet: 用于预生产测试
   - Mainnet: 生产环境

2. **密钥安全**
   - 不要将私钥提交到版本控制
   - 生产环境使用硬件钱包或多签
   - Treasury 密钥应妥善保管

3. **费用管理**
   - Devnet 可以免费申请空投
   - Mainnet 需要真实的 SOL
   - 建议先在 Devnet 充分测试

4. **程序升级**
   - Solana 程序默认可升级
   - 升级权限由部署者控制
   - 生产环境建议转移给多签账户

## 🔍 故障排查

### 问题 1: 余额不足

```bash
# 申请 Devnet 空投
solana airdrop 2

# 查看余额
solana balance
```

### 问题 2: 程序 ID 不匹配

```bash
# 重新构建并更新程序 ID
anchor build
anchor keys list
# 更新 lib.rs 和 Anchor.toml 中的程序 ID
```

### 问题 3: 账户已存在

```bash
# 使用不同的钱包或清理测试账户
solana-keygen new -o ~/.config/solana/test-wallet.json
solana config set --keypair ~/.config/solana/test-wallet.json
```

## 📚 相关文档

- [Anchor 文档](https://www.anchor-lang.com/)
- [Solana 文档](https://docs.solana.com/)
- [项目文档](../docs/README.md)
- [API 参考](../docs/API_REFERENCE.md)
- [部署指南](../docs/DEPLOYMENT_AND_TESTING.md)
