import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { PencilSolana } from "../target/types/pencil_solana";
import { PublicKey, Connection, SystemProgram } from "@solana/web3.js";
import {
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";

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
      return cluster;
  }
}

async function main() {
  console.log("🔧 初始化资产池关联账户 (initialize_related_accounts)");
  console.log("=".repeat(60));

  const anchorCfg = loadAnchorConfig();
  const rpcUrl = process.env.RPC_URL || getClusterUrl(anchorCfg.cluster);

  const assetPoolEnv = process.env.ASSET_POOL;
  const assetMintEnv = process.env.ASSET_MINT;

  if (!assetPoolEnv || !assetMintEnv) {
    console.error("❌ 请通过环境变量 ASSET_POOL 和 ASSET_MINT 提供资产池地址和资产 Mint 地址");
    process.exit(1);
  }

  // 加载钱包
  const walletData = JSON.parse(fs.readFileSync(anchorCfg.wallet, "utf-8"));
  const keypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array(walletData));

  const connection = new Connection(rpcUrl, "confirmed");
  const wallet = new Wallet(keypair);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const program = anchor.workspace.PencilSolana as Program<PencilSolana>;

  console.log("📍 Cluster:", anchorCfg.cluster);
  console.log("🔗 RPC:", rpcUrl);
  console.log("🆔 Program:", program.programId.toBase58());
  console.log("👤 Payer:", wallet.publicKey.toBase58());
  console.log("" );

  const assetPool = new PublicKey(assetPoolEnv);
  const assetMint = new PublicKey(assetMintEnv);

  // 1. 推导 SystemConfig PDA
  const [systemConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("system_config")],
    program.programId
  );

  // 2. 读取 SystemConfig 以获取 treasury
  const systemConfig = await program.account.systemConfig.fetch(systemConfigPda);
  const treasury = systemConfig.treasury as PublicKey;

  console.log("🏦 Treasury:", treasury.toBase58());

  // 3. 推导所有需要的 PDA
  const [funding] = PublicKey.findProgramAddressSync(
    [Buffer.from("funding"), assetPool.toBuffer()],
    program.programId
  );

  const [seniorPool] = PublicKey.findProgramAddressSync(
    [Buffer.from("senior_pool"), assetPool.toBuffer()],
    program.programId
  );

  const [firstLossPool] = PublicKey.findProgramAddressSync(
    [Buffer.from("first_loss_pool"), assetPool.toBuffer()],
    program.programId
  );

  const [juniorInterestPool] = PublicKey.findProgramAddressSync(
    [Buffer.from("junior_interest_pool"), assetPool.toBuffer()],
    program.programId
  );

  const [growTokenMint] = PublicKey.findProgramAddressSync(
    [Buffer.from("grow_token_mint"), assetPool.toBuffer()],
    program.programId
  );

  const [juniorNftMint] = PublicKey.findProgramAddressSync(
    [Buffer.from("junior_nft_mint"), assetPool.toBuffer()],
    program.programId
  );

  // 4. 关联账户
  // 4.1 金库 ATA（用户钱包 -> Treasury 的 ATA，使用 ATA Program 创建）
  const treasuryAta = await getAssociatedTokenAddress(
    assetMint,
    treasury,
    false
  );

  // 4.2 资产池 Vault：使用普通 TokenAccount，由 AssetPool PDA 作为 authority
  const assetPoolVaultKeypair = anchor.web3.Keypair.generate();

  console.log("💾 AssetPool:", assetPool.toBase58());
  console.log("💰 Asset Mint:", assetMint.toBase58());
  console.log("📦 Funding:", funding.toBase58());
  console.log("🏦 Treasury ATA:", treasuryAta.toBase58());
  console.log("🏦 Asset Pool Vault (new):", assetPoolVaultKeypair.publicKey.toBase58());
  console.log("" );

  // 5. 调用 initializeRelatedAccounts
  const txSig = await program.methods
    .initializeRelatedAccounts()
    .accounts({
      payer: wallet.publicKey,
      systemConfig: systemConfigPda,
      assetPool,
      assetMint,
      funding,
      seniorPool,
      firstLossPool,
      juniorInterestPool,
      growTokenMint,
      juniorNftMint,
      assetPoolVault: assetPoolVaultKeypair.publicKey,
      treasury,
      treasuryAta,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    } as any)
    .signers([assetPoolVaultKeypair])
    .rpc();

  console.log("✅ initialize_related_accounts 交易签名:", txSig);
}

main().catch((err) => {
  console.error("❌ 初始化关联账户失败:", err);
  process.exit(1);
});
