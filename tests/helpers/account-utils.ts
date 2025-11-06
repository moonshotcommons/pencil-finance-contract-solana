import * as anchor from "@coral-xyz/anchor";
import { PencilSolana } from "../../target/types/pencil_solana";
import { PublicKey } from "@solana/web3.js";

/**
 * Interface containing all pool-related account addresses
 */
export interface PoolAccounts {
  assetPool: PublicKey;
  funding: PublicKey;
  seniorPool: PublicKey;
  firstLossPool: PublicKey;
  juniorInterestPool: PublicKey;
  growTokenMint: PublicKey;
  juniorNftMint: PublicKey;
  assetPoolVault: PublicKey;
  treasuryAta: PublicKey;
}

/**
 * Derives all pool-related account PDAs
 * @param program Pencil Solana program
 * @param creator Pool creator public key
 * @param poolName Name of the pool
 * @param assetMint Asset token mint address
 * @param treasuryAddress Treasury address for ATA derivation
 * @returns PoolAccounts with all derived addresses
 */
export async function derivePoolAccounts(
  program: anchor.Program<PencilSolana>,
  creator: PublicKey,
  poolName: string,
  assetMint: PublicKey,
  treasuryAddress: PublicKey
): Promise<PoolAccounts> {
  // Derive asset pool PDA
  const [assetPoolPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("asset_pool"), creator.toBuffer(), Buffer.from(poolName)],
    program.programId
  );

  // Derive funding PDA
  const [fundingPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("funding"), assetPoolPda.toBuffer()],
    program.programId
  );

  // Derive senior pool PDA
  const [seniorPoolPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("senior_pool"), assetPoolPda.toBuffer()],
    program.programId
  );

  // Derive first loss pool PDA
  const [firstLossPoolPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("first_loss_pool"), assetPoolPda.toBuffer()],
    program.programId
  );

  // Derive junior interest pool PDA
  const [juniorInterestPoolPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("junior_interest_pool"), assetPoolPda.toBuffer()],
    program.programId
  );

  // Derive GROW token mint PDA
  const [growTokenMintPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("grow_token_mint"), assetPoolPda.toBuffer()],
    program.programId
  );

  // Derive junior NFT mint PDA
  const [juniorNftMintPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("junior_nft_mint"), assetPoolPda.toBuffer()],
    program.programId
  );

  // Derive asset pool vault (associated token account)
  const assetPoolVault = anchor.utils.token.associatedAddress({
    mint: assetMint,
    owner: assetPoolPda,
  });

  // Derive treasury ATA
  const treasuryAta = anchor.utils.token.associatedAddress({
    mint: assetMint,
    owner: treasuryAddress,
  });

  return {
    assetPool: assetPoolPda,
    funding: fundingPda,
    seniorPool: seniorPoolPda,
    firstLossPool: firstLossPoolPda,
    juniorInterestPool: juniorInterestPoolPda,
    growTokenMint: growTokenMintPda,
    juniorNftMint: juniorNftMintPda,
    assetPoolVault,
    treasuryAta,
  };
}

/**
 * Derives subscription PDA for a user
 * @param program Pencil Solana program
 * @param assetPool Asset pool address
 * @param user User public key
 * @param tranche Tranche type ("senior" or "junior")
 * @returns Subscription PDA
 */
export function deriveSubscriptionPda(
  program: anchor.Program<PencilSolana>,
  assetPool: PublicKey,
  user: PublicKey,
  tranche: "senior" | "junior"
): PublicKey {
  const [subscriptionPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("subscription"),
      assetPool.toBuffer(),
      user.toBuffer(),
      Buffer.from(tranche),
    ],
    program.programId
  );

  return subscriptionPda;
}

/**
 * Derives NFT metadata PDA for a junior NFT
 * @param program Pencil Solana program
 * @param assetPool Asset pool address
 * @param nftId NFT ID
 * @returns NFT metadata PDA
 */
export function deriveNftMetadataPda(
  program: anchor.Program<PencilSolana>,
  assetPool: PublicKey,
  nftId: anchor.BN
): PublicKey {
  const [nftMetadataPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("junior_nft_metadata"),
      assetPool.toBuffer(),
      nftId.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  return nftMetadataPda;
}

/**
 * Derives the junior NFT mint PDA for a specific NFT ID
 * @param program Pencil Solana program
 * @param assetPool Asset pool address
 * @param nftId NFT ID
 * @returns Junior NFT mint PDA
 */
export function deriveJuniorNftMintPda(
  program: anchor.Program<PencilSolana>,
  assetPool: PublicKey,
  nftId: anchor.BN
): PublicKey {
  const [juniorNftMintPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("junior_nft_mint"),
      assetPool.toBuffer(),
      nftId.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  return juniorNftMintPda;
}

/**
 * Derives asset whitelist PDA
 * @param program Pencil Solana program
 * @returns Asset whitelist PDA
 */
export function deriveAssetWhitelistPda(
  program: anchor.Program<PencilSolana>
): PublicKey {
  const [assetWhitelistPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("asset_whitelist")],
    program.programId
  );

  return assetWhitelistPda;
}

/**
 * Derives system config PDA
 * @param program Pencil Solana program
 * @returns System config PDA
 */
export function deriveSystemConfigPda(
  program: anchor.Program<PencilSolana>
): PublicKey {
  const [systemConfigPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("system_config")],
    program.programId
  );

  return systemConfigPda;
}
