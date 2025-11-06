import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createInitializeMintInstruction,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  getAssociatedTokenAddress,
  getAccount,
  getMint,
  MINT_SIZE,
  getMinimumBalanceForRentExemptMint,
} from "@solana/spl-token";

/**
 * Creates a test SPL token mint with specified decimals
 * @param provider Anchor provider
 * @param decimals Number of decimals for the token
 * @param name Token name (for logging)
 * @param symbol Token symbol (for logging)
 * @returns Public key of the created mint
 */
export async function createTestToken(
  provider: anchor.AnchorProvider,
  decimals: number,
  name: string,
  symbol: string
): Promise<PublicKey> {
  const mintKeypair = Keypair.generate();
  const payer = (provider.wallet as any).payer as Keypair;

  const lamports = await getMinimumBalanceForRentExemptMint(provider.connection);

  const transaction = new anchor.web3.Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mintKeypair.publicKey,
      space: MINT_SIZE,
      lamports,
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(
      mintKeypair.publicKey,
      decimals,
      payer.publicKey,
      payer.publicKey,
      TOKEN_PROGRAM_ID
    )
  );

  await provider.sendAndConfirm(transaction, [mintKeypair]);

  console.log(`✅ Created ${name} (${symbol}) token mint with ${decimals} decimals`);
  console.log(`   Mint address: ${mintKeypair.publicKey.toString()}`);

  return mintKeypair.publicKey;
}

/**
 * Mints tokens to a destination account
 * @param provider Anchor provider
 * @param mint Token mint address
 * @param destination Destination token account
 * @param amount Amount to mint (in token base units)
 */
export async function mintTokensTo(
  provider: anchor.AnchorProvider,
  mint: PublicKey,
  destination: PublicKey,
  amount: anchor.BN
): Promise<void> {
  const payer = (provider.wallet as any).payer as Keypair;

  const transaction = new anchor.web3.Transaction().add(
    createMintToInstruction(
      mint,
      destination,
      payer.publicKey,
      BigInt(amount.toString()),
      [],
      TOKEN_PROGRAM_ID
    )
  );

  await provider.sendAndConfirm(transaction);
}

/**
 * Gets the token balance of an account
 * @param provider Anchor provider
 * @param tokenAccount Token account address
 * @returns Balance as BN
 */
export async function getTokenBalance(
  provider: anchor.AnchorProvider,
  tokenAccount: PublicKey
): Promise<anchor.BN> {
  try {
    const account = await getAccount(provider.connection, tokenAccount);
    return new anchor.BN(account.amount.toString());
  } catch (error) {
    // Account doesn't exist or has no balance
    return new anchor.BN(0);
  }
}

/**
 * Creates an associated token account for a user
 * @param provider Anchor provider
 * @param mint Token mint address
 * @param owner Owner of the token account
 * @returns Public key of the created associated token account
 */
export async function createAssociatedTokenAccount(
  provider: anchor.AnchorProvider,
  mint: PublicKey,
  owner: PublicKey
): Promise<PublicKey> {
  const payer = (provider.wallet as any).payer as Keypair;

  const associatedTokenAddress = await getAssociatedTokenAddress(
    mint,
    owner,
    false,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  // Check if account already exists
  try {
    await getAccount(provider.connection, associatedTokenAddress);
    return associatedTokenAddress;
  } catch (error) {
    // Account doesn't exist, create it
  }

  const transaction = new anchor.web3.Transaction().add(
    createAssociatedTokenAccountInstruction(
      payer.publicKey,
      associatedTokenAddress,
      owner,
      mint,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )
  );

  await provider.sendAndConfirm(transaction);

  return associatedTokenAddress;
}

/**
 * Converts a human-readable token amount to base units
 * @param amount Human-readable amount
 * @param decimals Token decimals
 * @returns Amount in base units as BN
 */
export function toTokenAmount(amount: number, decimals: number): anchor.BN {
  return new anchor.BN(amount * Math.pow(10, decimals));
}

/**
 * Converts token base units to human-readable amount
 * @param amount Amount in base units
 * @param decimals Token decimals
 * @returns Human-readable amount
 */
export function fromTokenAmount(amount: anchor.BN, decimals: number): number {
  return amount.toNumber() / Math.pow(10, decimals);
}
