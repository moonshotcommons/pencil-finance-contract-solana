import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { PencilSolana } from "../target/types/pencil_solana";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert } from "chai";

describe("pencil-solana", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.PencilSolana as Program<PencilSolana>;

  const payer = provider.wallet;
  const treasury = Keypair.generate();
  const assetAddress = Keypair.generate();

  let systemConfigPda: PublicKey;
  let assetPoolPda: PublicKey;

  const platformFeeRate = 500;
  const seniorEarlyBeforeExitFeeRate = 100;
  const seniorEarlyAfterExitFeeRate = 200;
  const juniorEarlyBeforeExitFeeRate = 300;
  const defaultMinJuniorRatio = 1000;

  describe("System Configuration", () => {
    it("Initializes system config", async () => {
      [systemConfigPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("system_config")],
        program.programId
      );

      const tx = await program.methods
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
        .rpc();

      console.log("✅ System config initialized:", tx);

      const systemConfig = await program.account.systemConfig.fetch(systemConfigPda);
      assert.equal(systemConfig.superAdmin.toString(), payer.publicKey.toString());
      assert.equal(systemConfig.treasury.toString(), treasury.publicKey.toString());
      assert.equal(systemConfig.platformFeeRate, platformFeeRate);
    });
  });

  describe("Asset Pool", () => {
    it("Creates an asset pool", async () => {
      [assetPoolPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("asset_pool"), payer.publicKey.toBuffer()],
        program.programId
      );

      const now = Math.floor(Date.now() / 1000);
      const fundingStartTime = new BN(now + 60);
      const fundingEndTime = new BN(now + 60 + 86400); // 确保募资期限至少为 86400 秒（1天）

      const tx = await program.methods
        .createAssetPool(
          "Test Asset Pool",
          500,
          100,
          200,
          300,
          1000,
          10000,
          800,
          new BN(30),
          new BN(12),
          new BN(1000000 * 1_000_000),
          new BN(100000 * 1_000_000),
          fundingStartTime,
          fundingEndTime
        )
        .accounts({
          assetAddress: assetAddress.publicKey,
        })
        .signers([assetAddress])
        .rpc();

      console.log("✅ Asset pool created:", tx);

      const assetPool = await program.account.assetPool.fetch(assetPoolPda);
      assert.equal(assetPool.name, "Test Asset Pool");
      assert.equal(assetPool.creator.toString(), payer.publicKey.toString());
      assert.equal(assetPool.status, 0);
    });

    it("Approves an asset pool", async () => {
      const tx = await program.methods
        .approveAssetPool(payer.publicKey)
        .rpc();

      console.log("✅ Asset pool approved:", tx);

      const assetPool = await program.account.assetPool.fetch(assetPoolPda);
      assert.equal(assetPool.status, 1);
    });
  });
});
