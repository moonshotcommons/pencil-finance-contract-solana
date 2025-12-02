import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

async function main() {
  // 使用 Anchor.toml 中的 provider 配置（devnet + wallet.json）
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 通过 workspace 拿到已部署的程序（要求 `anchor build` 过，且 target/types 存在）
  const program = anchor.workspace.PencilSolana as anchor.Program<any>;

  // 根据和合约里一致的 seed 推导 PDA
  const [systemConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("system_config")],
    program.programId
  );
  const [assetWhitelistPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("asset_whitelist")],
    program.programId
  );

  // 要加入白名单的资产 mint
  const assetMint = new PublicKey(
    "AGF47QrB3xLeDU5bHMqhUG8Ka6uLmoCRN62XBinyWfc9"
  );

  console.log("Program ID:", program.programId.toBase58());
  console.log("SystemConfig PDA:", systemConfigPda.toBase58());
  console.log("AssetWhitelist PDA:", assetWhitelistPda.toBase58());
  console.log("Asset Mint:", assetMint.toBase58());

  const anyProgram = program as any;

  const txSig = await anyProgram.methods
    .setAssetSupported(assetMint, true)
    .accounts({
      operationAdmin: provider.wallet.publicKey,
      systemConfig: systemConfigPda,
      assetWhitelist: assetWhitelistPda,
    })
    .rpc();

  console.log("✅ Asset added to whitelist, tx:", txSig);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
