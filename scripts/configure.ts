import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { PencilSolana } from "../target/types/pencil_solana";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";

/**
 * 系统配置管理脚本
 * 用于管理和更新 SystemConfig 的各种参数
 */

interface DeploymentInfo {
  network: string;
  programId: string;
  accounts: {
    systemConfig: string;
    treasury: string;
    [key: string]: string;
  };
  config: {
    platformFeeRate: number;
    seniorEarlyBeforeExitFeeRate: number;
    seniorEarlyAfterExitFeeRate: number;
    juniorEarlyBeforeExitFeeRate: number;
    defaultMinJuniorRatio: number;
  };
}

async function loadDeploymentInfo(): Promise<DeploymentInfo> {
  try {
    const data = fs.readFileSync("deployment-solana.json", "utf-8");
    return JSON.parse(data);
  } catch (error) {
    console.error("❌ 无法加载部署信息文件 deployment-solana.json");
    console.error("   请先运行部署脚本: ts-node scripts/deploy.ts");
    throw error;
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
    console.log("⚠️ 无法读取 Anchor.toml，使用默认配置");
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
      return "https://api.devnet.solana.com";
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
  console.log("🔧 Pencil Solana 配置管理工具");
  console.log("=".repeat(60));

  // 加载部署信息
  const deploymentInfo = await loadDeploymentInfo();
  console.log(`📍 网络: ${deploymentInfo.network}`);
  console.log(`🆔 程序 ID: ${deploymentInfo.programId}`);
  console.log("=".repeat(60));

  // 读取配置
  const config = loadAnchorConfig();
  const rpcUrl = getClusterUrl(config.cluster);

  // 加载钱包
  let walletKeypair: Keypair;
  try {
    const walletData = JSON.parse(fs.readFileSync(config.wallet, "utf-8"));
    walletKeypair = Keypair.fromSecretKey(new Uint8Array(walletData));
  } catch (error) {
    console.log(`❌ 无法加载钱包文件: ${config.wallet}`);
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

  if (rpcEndpoint.includes("localhost") || rpcEndpoint.includes("127.0.0.1")) {
    networkName = "Localnet";
  } else if (rpcEndpoint.includes("devnet")) {
    networkName = "Devnet";
  } else if (rpcEndpoint.includes("testnet")) {
    networkName = "Testnet";
  } else if (rpcEndpoint.includes("mainnet")) {
    networkName = "Mainnet-Beta";
  }

  console.log(`🌐 当前网络: ${networkName}`);
  console.log(`🔗 RPC 端点: ${rpcEndpoint}`);
  console.log(`👤 当前账户: ${wallet.publicKey.toString()}`);
  console.log("");

  // 验证网络匹配
  if (deploymentInfo.network !== networkName) {
    console.log("⚠️ 警告：当前网络与部署信息不匹配！");
    console.log(`   部署网络: ${deploymentInfo.network}`);
    console.log(`   当前网络: ${networkName}`);
    console.log("");
  }

  // 获取 SystemConfig PDA
  const systemConfigPda = new PublicKey(deploymentInfo.accounts.systemConfig);

  // 获取当前配置
  console.log("📊 当前系统配置:");
  console.log("-".repeat(60));

  const systemConfig = await program.account.systemConfig.fetch(systemConfigPda);

  console.log(`平台费率: ${systemConfig.platformFeeRate} (${systemConfig.platformFeeRate / 100}%)`);
  console.log(`优先份额提前退出费（募资前）: ${systemConfig.seniorEarlyBeforeExitFeeRate} (${systemConfig.seniorEarlyBeforeExitFeeRate / 100}%)`);
  console.log(`优先份额提前退出费（募资后）: ${systemConfig.seniorEarlyAfterExitFeeRate} (${systemConfig.seniorEarlyAfterExitFeeRate / 100}%)`);
  console.log(`劣后份额提前退出费: ${systemConfig.juniorEarlyBeforeExitFeeRate} (${systemConfig.juniorEarlyBeforeExitFeeRate / 100}%)`);
  console.log(`默认最低 Junior 占比: ${systemConfig.defaultMinJuniorRatio} (${systemConfig.defaultMinJuniorRatio / 100}%)`);
  console.log("");
  console.log("👥 管理员角色:");
  console.log(`Super Admin: ${systemConfig.superAdmin.toString()}`);
  console.log(`System Admin: ${systemConfig.systemAdmin.toString()}`);
  console.log(`Treasury Admin: ${systemConfig.treasuryAdmin.toString()}`);
  console.log(`Operation Admin: ${systemConfig.operationAdmin.toString()}`);
  console.log(`Treasury: ${systemConfig.treasury.toString()}`);
  console.log("");
  console.log(`初始化状态: ${systemConfig.initialized ? "✅ 已初始化" : "❌ 未初始化"}`);

  console.log("=".repeat(60));
  console.log("");
  console.log("💡 可用操作:");
  console.log("1. 查看当前配置（已完成）");
  console.log("2. 更新费率参数（需要实现对应的更新指令）");
  console.log("3. 更新管理员角色（需要实现对应的更新指令）");
  console.log("");
  console.log("⚠️ 注意：当前程序尚未实现配置更新指令");
  console.log("   如需更新配置，请在程序中添加相应的指令");
  console.log("");

  // 显示所有资产池
  console.log("📦 查询资产池...");
  console.log("-".repeat(60));

  try {
    // 尝试获取第一个资产池
    if (deploymentInfo.accounts.assetPool) {
      const assetPoolPda = new PublicKey(deploymentInfo.accounts.assetPool);
      const assetPool = await program.account.assetPool.fetch(assetPoolPda);

      console.log(`资产池地址: ${assetPoolPda.toString()}`);
      console.log(`名称: ${assetPool.name}`);
      console.log(`状态: ${assetPool.status === 0 ? "已创建" : assetPool.status === 1 ? "已批准" : assetPool.status === 2 ? "募资中" : assetPool.status === 3 ? "已完成" : "未知"}`);
      console.log(`创建者: ${assetPool.creator.toString()}`);
      console.log(`资产地址: ${assetPool.assetAddress.toString()}`);
      console.log(`总金额: ${assetPool.totalAmount.toString()}`);
      console.log(`最低金额: ${assetPool.minAmount.toString()}`);
      console.log(`优先份额固定利率: ${assetPool.seniorFixedRate} (${assetPool.seniorFixedRate / 100}%)`);
      console.log(`还款周期: ${assetPool.repaymentPeriod.toString()} 天`);
      console.log(`还款期数: ${assetPool.repaymentCount.toString()}`);
      console.log(`创建时间: ${new Date(assetPool.createdAt.toNumber() * 1000).toLocaleString()}`);
      console.log("");
    }
  } catch (error) {
    console.log("ℹ️ 暂无资产池数据");
  }

  console.log("=".repeat(60));
  console.log("✅ 配置查询完成");
}

main()
  .then(() => {
    process.exit(0);
  })
  .catch((error) => {
    console.error("❌ 配置脚本执行失败:", error);
    process.exit(1);
  });
