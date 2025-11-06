import * as anchor from "@coral-xyz/anchor";
import { Program, BN, Wallet } from "@coral-xyz/anchor";
import { PencilSolana } from "../target/types/pencil_solana";
import { PublicKey, Keypair, LAMPORTS_PER_SOL, Connection } from "@solana/web3.js";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";

/**
 * Solana 版本的部署和初始化脚本
 * 等效于 EVM 版本的 deployToEDUChain.js
 */

// 颜色输出辅助函数
const colors = {
  reset: "\x1b[0m",
  bright: "\x1b[1m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  blue: "\x1b[34m",
  cyan: "\x1b[36m",
  red: "\x1b[31m",
};

function log(emoji: string, message: string, color: string = colors.reset) {
  console.log(`${color}${emoji} ${message}${colors.reset}`);
}

function separator(char: string = "=", length: number = 60) {
  console.log(char.repeat(length));
}

// 交易追踪器
class TransactionTracker {
  private transactions: Array<{
    description: string;
    signature: string;
    slot: number;
    fee: number;
  }> = [];
  private totalFee: number = 0;

  async trackTransaction(
    txPromise: Promise<string>,
    description: string,
    connection: Connection
  ): Promise<string> {
    log("⏳", `${description}...`, colors.cyan);
    const signature = await txPromise;

    // 等待确认
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...latestBlockhash,
    });

    // 获取交易详情（使用 confirmed commitment）
    const txInfo = await connection.getTransaction(signature, {
      maxSupportedTransactionVersion: 0,
      commitment: "confirmed",
    });

    const fee = txInfo?.meta?.fee || 0;
    const slot = txInfo?.slot || 0;

    this.transactions.push({
      description,
      signature,
      slot,
      fee,
    });

    this.totalFee += fee;

    log(
      "✅",
      `${description} 完成`,
      colors.green
    );
    log(
      "  💰",
      `费用: ${(fee / LAMPORTS_PER_SOL).toFixed(6)} SOL`,
      colors.yellow
    );
    log(
      "  🔗",
      `交易: https://explorer.solana.com/tx/${signature}?cluster=devnet`,
      colors.blue
    );

    return signature;
  }

  printSummary() {
    separator();
    log("📊", "交易统计报告", colors.bright);
    separator();

    this.transactions.forEach((tx, index) => {
      console.log(`${(index + 1).toString().padStart(2)}. ${tx.description}`);
      console.log(`    费用: ${(tx.fee / LAMPORTS_PER_SOL).toFixed(6)} SOL`);
      console.log(`    Slot: ${tx.slot}`);
      console.log(`    签名: ${tx.signature}`);
      console.log("");
    });

    console.log("📊 总计统计:");
    console.log(`总交易费用: ${(this.totalFee / LAMPORTS_PER_SOL).toFixed(6)} SOL`);
    console.log(`交易笔数: ${this.transactions.length}`);
    console.log(
      `平均每笔费用: ${(this.totalFee / this.transactions.length / LAMPORTS_PER_SOL).toFixed(6)} SOL`
    );
    separator();
  }
}

// 读取 Anchor.toml 配置
function loadAnchorConfig(): { cluster: string; wallet: string } {
  try {
    const tomlContent = fs.readFileSync("Anchor.toml", "utf-8");
    const clusterMatch = tomlContent.match(/cluster\s*=\s*"([^"]+)"/);
    const walletMatch = tomlContent.match(/wallet\s*=\s*"([^"]+)"/);

    const cluster = clusterMatch ? clusterMatch[1] : "localnet";
    const wallet = walletMatch ? walletMatch[1].replace("~", os.homedir()) : path.join(os.homedir(), ".config/solana/id.json");

    return { cluster, wallet };
  } catch (error) {
    log("⚠️", "无法读取 Anchor.toml，使用默认配置", colors.yellow);
    return {
      cluster: "localnet",
      wallet: path.join(os.homedir(), ".config/solana/id.json"),
    };
  }
}

// 获取 RPC URL
function getClusterUrl(cluster: string): string {
  const clusterLower = cluster.toLowerCase();
  switch (clusterLower) {
    case "localnet":
      return "http://127.0.0.1:8899";
    case "devnet":
      return "https://distinguished-morning-snowflake.solana-devnet.quiknode.pro/5834e30419564fcc6461f3746bf23f7d96b0d1d7";
    case "testnet":
      return "https://api.testnet.solana.com";
    case "mainnet":
    case "mainnet-beta":
      return "https://api.mainnet-beta.solana.com";
    default:
      return cluster; // 假设是自定义 URL
  }
}

async function main() {
  log("🚀", "开始部署 Pencil Solana 程序...", colors.bright);
  separator();

  // 读取配置
  const config = loadAnchorConfig();
  const rpcUrl = getClusterUrl(config.cluster);

  // 加载钱包
  let walletKeypair: Keypair;
  try {
    const walletData = JSON.parse(fs.readFileSync(config.wallet, "utf-8"));
    walletKeypair = Keypair.fromSecretKey(new Uint8Array(walletData));
  } catch (error) {
    log("❌", `无法加载钱包文件: ${config.wallet}`, colors.red);
    throw error;
  }

  // 创建 provider
  const connection = new Connection(rpcUrl, "confirmed");
  const wallet = new Wallet(walletKeypair);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const program = anchor.workspace.PencilSolana as Program<PencilSolana>;


  // 检测网络类型
  const rpcEndpoint = connection.rpcEndpoint;
  let networkName = "Unknown";
  let isMainnet = false;

  if (rpcEndpoint.includes("localhost") || rpcEndpoint.includes("127.0.0.1")) {
    networkName = "Localnet";
  } else if (rpcEndpoint.includes("devnet")) {
    networkName = "Devnet";
  } else if (rpcEndpoint.includes("testnet")) {
    networkName = "Testnet";
  } else if (rpcEndpoint.includes("mainnet")) {
    networkName = "Mainnet-Beta";
    isMainnet = true;
  }

  log("📍", `网络: ${networkName} (${rpcEndpoint})`, colors.cyan);
  log("👤", `部署账户: ${wallet.publicKey.toString()}`, colors.cyan);
  log("🆔", `程序 ID: ${program.programId.toString()}`, colors.cyan);

  // Mainnet 警告
  if (isMainnet) {
    separator("!", 60);
    log("⚠️", "警告：您正在部署到 Mainnet！", colors.red);
    log("⚠️", "这将使用真实的 SOL，请确保您知道自己在做什么！", colors.red);
    separator("!", 60);
    console.log("");
  }

  // 检查账户余额
  const balance = await connection.getBalance(wallet.publicKey);
  log("💰", `账户余额: ${(balance / LAMPORTS_PER_SOL).toFixed(4)} SOL`, colors.yellow);

  if (balance < 0.1 * LAMPORTS_PER_SOL) {
    log("⚠️", "警告：账户余额较低，可能不足以完成部署", colors.red);
  }

  separator();

  const tracker = new TransactionTracker();
  const deployedAccounts: Record<string, string> = {};

  try {
    // 1. 初始化 SystemConfig
    log("⚙️", "1. 初始化 SystemConfig...", colors.bright);

    const treasury = Keypair.generate();
    deployedAccounts.treasury = treasury.publicKey.toString();

    const [systemConfigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("system_config")],
      program.programId
    );
    deployedAccounts.systemConfig = systemConfigPda.toString();

    // 配置参数（与 EVM 版本保持一致）
    const platformFeeRate = 500; // 5%
    const seniorEarlyBeforeExitFeeRate = 100; // 1%
    const seniorEarlyAfterExitFeeRate = 200; // 2%
    const juniorEarlyBeforeExitFeeRate = 300; // 3%
    const defaultMinJuniorRatio = 1000; // 10%

    const initTx = await tracker.trackTransaction(
      program.methods
        .initializeSystemConfig(
          platformFeeRate,
          seniorEarlyBeforeExitFeeRate,
          seniorEarlyAfterExitFeeRate,
          juniorEarlyBeforeExitFeeRate,
          defaultMinJuniorRatio
        )
        .accounts({
          treasury: treasury.publicKey,
        })
        .signers([treasury])
        .rpc(),
      "初始化 SystemConfig",
      connection
    );

    log("✅", `SystemConfig PDA: ${systemConfigPda.toString()}`, colors.green);

    // 2. 创建示例资产池
    log("\n📦", "2. 创建示例资产池...", colors.bright);

    const assetAddress = Keypair.generate();
    deployedAccounts.assetAddress = assetAddress.publicKey.toString();

    const assetPoolName = "Demo Asset Pool";
    const [assetPoolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("asset_pool"), wallet.publicKey.toBuffer(), Buffer.from(assetPoolName)],
      program.programId
    );
    deployedAccounts.assetPool = assetPoolPda.toString();

    const now = Math.floor(Date.now() / 1000);
    const fundingStartTime = new BN(now + 60); // 1分钟后开始
    const fundingEndTime = new BN(now + 60 + 86400); // 1天募资期

    const createPoolTx = await tracker.trackTransaction(
      program.methods
        .createAssetPool(
          assetPoolName,
          500, // platform_fee: 5%
          100, // senior_early_before_exit_fee: 1%
          200, // senior_early_after_exit_fee: 2%
          300, // junior_early_before_exit_fee: 3%
          1000, // min_junior_ratio: 10%
          10000, // repayment_rate: 100%
          800, // senior_fixed_rate: 8%
          new BN(30), // repayment_period: 30 days
          new BN(12), // repayment_count: 12 periods
          new BN(1000000 * 1_000_000), // total_amount: 1M USDC (假设6位精度)
          new BN(100000 * 1_000_000), // min_amount: 100K USDC
          fundingStartTime,
          fundingEndTime
        )
        .accounts({
          assetAddress: assetAddress.publicKey,
        })
        .signers([assetAddress])
        .rpc(),
      "创建示例资产池",
      connection
    );

    log("✅", `Asset Pool PDA: ${assetPoolPda.toString()}`, colors.green);

    // 3. 批准资产池
    log("\n✅", "3. 批准资产池...", colors.bright);

    const approveTx = await tracker.trackTransaction(
      program.methods
        .approveAssetPool(wallet.publicKey, assetPoolName)
        .rpc(),
      "批准资产池",
      connection
    );

    // 4. 验证部署结果
    log("\n🔍", "4. 验证部署结果...", colors.bright);

    const systemConfig = await program.account.systemConfig.fetch(systemConfigPda);
    log("  📊", `平台费率: ${systemConfig.platformFeeRate} (${systemConfig.platformFeeRate / 100}%)`, colors.cyan);
    log("  🚪", `优先份额提前退出费（募资前）: ${systemConfig.seniorEarlyBeforeExitFeeRate} (${systemConfig.seniorEarlyBeforeExitFeeRate / 100}%)`, colors.cyan);
    log("  🚪", `优先份额提前退出费（募资后）: ${systemConfig.seniorEarlyAfterExitFeeRate} (${systemConfig.seniorEarlyAfterExitFeeRate / 100}%)`, colors.cyan);
    log("  🚪", `劣后份额提前退出费: ${systemConfig.juniorEarlyBeforeExitFeeRate} (${systemConfig.juniorEarlyBeforeExitFeeRate / 100}%)`, colors.cyan);
    log("  📈", `默认最低 Junior 占比: ${systemConfig.defaultMinJuniorRatio} (${systemConfig.defaultMinJuniorRatio / 100}%)`, colors.cyan);
    log("  👑", `Super Admin: ${systemConfig.superAdmin.toString()}`, colors.cyan);
    log("  🏦", `Treasury: ${systemConfig.treasury.toString()}`, colors.cyan);

    const assetPool = await program.account.assetPool.fetch(assetPoolPda);
    log("  📦", `资产池名称: ${assetPool.name}`, colors.cyan);
    log("  📊", `资产池状态: ${assetPool.status === 0 ? "已创建" : assetPool.status === 1 ? "已批准" : "未知"}`, colors.cyan);
    log("  👤", `创建者: ${assetPool.creator.toString()}`, colors.cyan);
    log("  💰", `总金额: ${assetPool.totalAmount.toString()}`, colors.cyan);
    log("  📉", `最低金额: ${assetPool.minAmount.toString()}`, colors.cyan);

    // 显示交易统计
    tracker.printSummary();

    // 5. 输出部署摘要
    separator();
    log("🎉", "部署完成！账户地址摘要:", colors.bright);
    separator();

    Object.entries(deployedAccounts).forEach(([name, address]) => {
      console.log(`${name.padEnd(20)}: ${address}`);
      console.log(`${"".padEnd(20)}  🔗 https://explorer.solana.com/address/${address}?cluster=devnet`);
    });

    // 保存部署信息
    const deploymentInfo = {
      network: networkName,
      rpcEndpoint: rpcEndpoint,
      programId: program.programId.toString(),
      deployedAt: new Date().toISOString(),
      deployer: wallet.publicKey.toString(),
      accounts: deployedAccounts,
      config: {
        platformFeeRate,
        seniorEarlyBeforeExitFeeRate,
        seniorEarlyAfterExitFeeRate,
        juniorEarlyBeforeExitFeeRate,
        defaultMinJuniorRatio,
      },
      transactions: tracker["transactions"],
    };

    fs.writeFileSync(
      "deployment-solana.json",
      JSON.stringify(deploymentInfo, null, 2)
    );

    log("\n💾", "部署信息已保存到 deployment-solana.json", colors.green);

    separator();
    log("📋", "下一步操作:", colors.bright);
    console.log("1. 保存上述账户地址用于前端集成");
    console.log("2. 在 Solana Explorer 中查看账户和交易");
    console.log("3. 测试合约功能（订阅、还款等）");
    console.log("4. 创建更多资产池进行测试");
    console.log("");
    log("⚠️", "安全提示:", colors.yellow);
    console.log("1. 在生产环境中，SuperAdmin 应该是多签钱包地址");
    console.log("2. Treasury 应该使用安全的密钥管理方案");
    console.log("3. 建议在 Devnet 充分测试后再部署到 Mainnet");
    console.log("");
    log("🔗", "重要链接:", colors.blue);
    console.log(`   Solana Explorer: https://explorer.solana.com/?cluster=devnet`);
    console.log(`   程序地址: https://explorer.solana.com/address/${program.programId.toString()}?cluster=devnet`);

  } catch (error) {
    log("\n❌", "部署失败:", colors.red);
    console.error(error);

    if (tracker["transactions"].length > 0) {
      tracker.printSummary();
    }

    throw error;
  }
}

main()
  .then(() => {
    log("\n🎉", "Solana 部署脚本执行完成！", colors.bright);
    process.exit(0);
  })
  .catch((error) => {
    log("\n💥", "部署脚本执行失败:", colors.red);
    console.error(error);
    process.exit(1);
  });
