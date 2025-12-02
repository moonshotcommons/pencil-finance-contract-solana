import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PencilSolana as any;

  const [systemConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("system_config")],
    program.programId
  );

  console.log("Program ID:", program.programId.toBase58());
  console.log("SystemConfig PDA:", systemConfigPda.toBase58());

  // 如果已经有 system_config，就直接跳过初始化
  try {
    const existing = await program.account.systemConfig.fetch(systemConfigPda);
    console.log("✅ SystemConfig already exists:", existing);
    return;
  } catch (e) {
    console.log("ℹ️ SystemConfig not found on devnet, initializing...");
  }

  // 使用与测试中相同的一组默认参数
  const platformFeeRate = 500; // 5%
  const seniorEarlyBeforeExitFeeRate = 100; // 1%
  const seniorEarlyAfterExitFeeRate = 200; // 2%
  const juniorEarlyBeforeExitFeeRate = 300; // 3%
  const defaultMinJuniorRatio = 1000; // 10%

  const txSig = await program.methods
    .initializeSystemConfig(
      platformFeeRate,
      seniorEarlyBeforeExitFeeRate,
      seniorEarlyAfterExitFeeRate,
      juniorEarlyBeforeExitFeeRate,
      defaultMinJuniorRatio
    )
    .accounts({
      // payer 默认就是 provider.wallet
      treasury: provider.wallet.publicKey,
    })
    .rpc();

  console.log("✅ SystemConfig initialized, tx:", txSig);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
