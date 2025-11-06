import * as anchor from "@coral-xyz/anchor";
import { PencilSolana } from "../../target/types/pencil_solana";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

/**
 * Test environment interface containing all necessary accounts and configuration
 */
export interface TestEnvironment {
  provider: anchor.AnchorProvider;
  program: anchor.Program<PencilSolana>;
  systemConfig: PublicKey;
  treasury: Keypair;

  // Admin accounts
  superAdmin: Keypair;
  systemAdmin: Keypair;
  operationAdmin: Keypair;
  treasuryAdmin: Keypair;

  // Test tokens
  usdtMint: PublicKey;  // 6 decimals
  usdcMint: PublicKey;  // 9 decimals

  // User accounts
  borrower1: Keypair;
  borrower2: Keypair;
  seniorInvestor1: Keypair;
  seniorInvestor2: Keypair;
  juniorInvestor1: Keypair;
  juniorInvestor2: Keypair;
}

/**
 * Sets up the complete test environment with all necessary accounts
 * @returns TestEnvironment with all initialized accounts
 */
export async function setupTestEnvironment(): Promise<TestEnvironment> {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.PencilSolana as anchor.Program<PencilSolana>;

  // Generate admin accounts
  const treasury = Keypair.generate();
  const superAdmin = (provider.wallet as any).payer as Keypair;
  const systemAdmin = Keypair.generate();
  const operationAdmin = Keypair.generate();
  const treasuryAdmin = Keypair.generate();

  // Generate user accounts
  const borrower1 = Keypair.generate();
  const borrower2 = Keypair.generate();
  const seniorInvestor1 = Keypair.generate();
  const seniorInvestor2 = Keypair.generate();
  const juniorInvestor1 = Keypair.generate();
  const juniorInvestor2 = Keypair.generate();

  // Derive system config PDA
  const [systemConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("system_config")],
    program.programId
  );

  // Airdrop SOL to all accounts
  await airdropSol(provider, treasury.publicKey, 2);
  await airdropSol(provider, systemAdmin.publicKey, 2);
  await airdropSol(provider, operationAdmin.publicKey, 2);
  await airdropSol(provider, treasuryAdmin.publicKey, 2);
  await airdropSol(provider, borrower1.publicKey, 2);
  await airdropSol(provider, borrower2.publicKey, 2);
  await airdropSol(provider, seniorInvestor1.publicKey, 2);
  await airdropSol(provider, seniorInvestor2.publicKey, 2);
  await airdropSol(provider, juniorInvestor1.publicKey, 2);
  await airdropSol(provider, juniorInvestor2.publicKey, 2);

  return {
    provider,
    program,
    systemConfig: systemConfigPda,
    treasury,
    superAdmin,
    systemAdmin,
    operationAdmin,
    treasuryAdmin,
    usdtMint: PublicKey.default, // Will be set during token creation
    usdcMint: PublicKey.default, // Will be set during token creation
    borrower1,
    borrower2,
    seniorInvestor1,
    seniorInvestor2,
    juniorInvestor1,
    juniorInvestor2,
  };
}

/**
 * Airdrops SOL to a specified public key
 * @param provider Anchor provider
 * @param pubkey Public key to receive SOL
 * @param amount Amount of SOL to airdrop
 */
export async function airdropSol(
  provider: anchor.AnchorProvider,
  pubkey: PublicKey,
  amount: number
): Promise<void> {
  const signature = await provider.connection.requestAirdrop(
    pubkey,
    amount * LAMPORTS_PER_SOL
  );
  await provider.connection.confirmTransaction(signature);
}

/**
 * Initializes the system configuration with fee rates and admin roles
 * @param env Test environment
 */
export async function initializeSystemConfig(env: TestEnvironment): Promise<void> {
  const platformFeeRate = 500; // 5%
  const seniorEarlyBeforeExitFeeRate = 100; // 1%
  const seniorEarlyAfterExitFeeRate = 200; // 2%
  const juniorEarlyBeforeExitFeeRate = 300; // 3%
  const defaultMinJuniorRatio = 2000; // 20%

  await env.program.methods
    .initializeSystemConfig(
      platformFeeRate,
      seniorEarlyBeforeExitFeeRate,
      seniorEarlyAfterExitFeeRate,
      juniorEarlyBeforeExitFeeRate,
      defaultMinJuniorRatio
    )
    .accounts({
      treasury: env.treasury.publicKey,
    })
    .signers([env.treasury])
    .rpc();

  // Set additional admin roles
  await env.program.methods
    .updateAdmin({ systemAdmin: {} }, env.systemAdmin.publicKey)
    .rpc();

  await env.program.methods
    .updateAdmin({ operationAdmin: {} }, env.operationAdmin.publicKey)
    .rpc();

  await env.program.methods
    .updateAdmin({ treasuryAdmin: {} }, env.treasuryAdmin.publicKey)
    .rpc();
}
