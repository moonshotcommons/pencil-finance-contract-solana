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
  console.log("Current super admin (signer):", provider.wallet.publicKey.toBase58());

  const newAdminStr = process.env.NEW_ADMIN;
  if (!newAdminStr) {
    throw new Error("NEW_ADMIN env var is required (new admin public key)");
  }
  const newAdmin = new PublicKey(newAdminStr);
  console.log("New admin:", newAdmin.toBase58());

  // 注意：UpdateAdmin 始终要求 super_admin 作为签名人，
  // 所以必须在最后才更新 SuperAdmin，自下而上依次更新。
  const roles: { label: string; arg: any }[] = [
    { label: "SystemAdmin", arg: { systemAdmin: {} } },
    { label: "TreasuryAdmin", arg: { treasuryAdmin: {} } },
    { label: "OperationAdmin", arg: { operationAdmin: {} } },
    { label: "SuperAdmin", arg: { superAdmin: {} } },
  ];

  for (const role of roles) {
    console.log(`\nUpdating role: ${role.label} ->`, newAdmin.toBase58());

    const txSig = await program.methods
      .updateAdmin(role.arg, newAdmin)
      .accounts({
        superAdmin: provider.wallet.publicKey,
        systemConfig: systemConfigPda,
      })
      .rpc();

    console.log(`✅ ${role.label} updated, tx:`, txSig);
  }

  console.log("\n✅ All admin roles updated to:", newAdmin.toBase58());
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
