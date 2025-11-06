import * as anchor from "@coral-xyz/anchor";
import { PencilSolana } from "../../target/types/pencil_solana";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import { getTokenBalance, fromTokenAmount } from "./token-utils";

/**
 * Asserts that a token account has the expected balance
 * @param provider Anchor provider
 * @param tokenAccount Token account address
 * @param expectedAmount Expected balance
 * @param message Optional custom error message
 */
export async function assertTokenBalance(
  provider: anchor.AnchorProvider,
  tokenAccount: PublicKey,
  expectedAmount: anchor.BN,
  message?: string
): Promise<void> {
  const actualBalance = await getTokenBalance(provider, tokenAccount);

  const errorMessage = message ||
    `Expected balance ${expectedAmount.toString()}, but got ${actualBalance.toString()}`;

  assert.equal(
    actualBalance.toString(),
    expectedAmount.toString(),
    errorMessage
  );
}

/**
 * Asserts that an asset pool has the expected status
 * @param program Pencil Solana program
 * @param assetPool Asset pool address
 * @param expectedStatus Expected pool status (0=Pending, 1=Approved, 2=Funded, 3=Ended, etc.)
 */
export async function assertPoolStatus(
  program: anchor.Program<PencilSolana>,
  assetPool: PublicKey,
  expectedStatus: number
): Promise<void> {
  const poolAccount = await program.account.assetPool.fetch(assetPool);

  const statusNames = [
    "Pending",
    "Approved",
    "Funded",
    "Ended",
    "Defaulted",
    "Liquidated",
    "Cancelled"
  ];

  const expectedStatusName = statusNames[expectedStatus] || `Unknown(${expectedStatus})`;
  const actualStatusName = statusNames[poolAccount.status] || `Unknown(${poolAccount.status})`;

  assert.equal(
    poolAccount.status,
    expectedStatus,
    `Expected pool status ${expectedStatusName}, but got ${actualStatusName}`
  );
}

/**
 * Asserts that a subscription has the expected amount
 * @param program Pencil Solana program
 * @param subscription Subscription PDA
 * @param expectedAmount Expected subscription amount
 */
export async function assertSubscriptionAmount(
  program: anchor.Program<PencilSolana>,
  subscription: PublicKey,
  expectedAmount: anchor.BN
): Promise<void> {
  const subscriptionAccount = await program.account.subscription.fetch(subscription);

  assert.equal(
    subscriptionAccount.amount.toString(),
    expectedAmount.toString(),
    `Expected subscription amount ${expectedAmount.toString()}, but got ${subscriptionAccount.amount.toString()}`
  );
}

/**
 * Logs a balance change with formatting
 * @param label Description of the balance
 * @param before Balance before operation
 * @param after Balance after operation
 * @param decimals Token decimals for formatting
 */
export function logBalanceChange(
  label: string,
  before: anchor.BN,
  after: anchor.BN,
  decimals: number
): void {
  const beforeFormatted = fromTokenAmount(before, decimals).toLocaleString();
  const afterFormatted = fromTokenAmount(after, decimals).toLocaleString();
  const change = after.sub(before);
  const changeFormatted = fromTokenAmount(change, decimals).toLocaleString();
  const changeSign = change.isNeg() ? "" : "+";

  console.log(`   ${label}:`);
  console.log(`     Before: ${beforeFormatted}`);
  console.log(`     After:  ${afterFormatted}`);
  console.log(`     Change: ${changeSign}${changeFormatted}`);
}

/**
 * Logs a test phase with emoji and formatting
 * @param phase Phase name
 * @param emoji Emoji to display
 */
export function logTestPhase(phase: string, emoji: string = "📋"): void {
  const separator = "=".repeat(60);
  console.log(`\n${separator}`);
  console.log(`${emoji} ${phase}`);
  console.log(`${separator}\n`);
}

/**
 * Logs a successful operation
 * @param operation Operation description
 */
export function logSuccess(operation: string): void {
  console.log(`✅ ${operation}`);
}

/**
 * Logs a warning or skipped operation
 * @param operation Operation description
 */
export function logWarning(operation: string): void {
  console.log(`⚠️  ${operation}`);
}

/**
 * Logs an error
 * @param operation Operation description
 */
export function logError(operation: string): void {
  console.log(`❌ ${operation}`);
}

/**
 * Logs detailed information with indentation
 * @param info Information to log
 */
export function logInfo(info: string): void {
  console.log(`   ${info}`);
}

/**
 * Logs a transaction signature
 * @param label Transaction description
 * @param signature Transaction signature
 */
export function logTransaction(label: string, signature: string): void {
  console.log(`✅ ${label}`);
  console.log(`   Tx: ${signature}`);
}

/**
 * Logs account balances in a formatted table
 * @param balances Map of account labels to balances
 * @param decimals Token decimals
 */
export function logBalances(
  balances: Map<string, anchor.BN>,
  decimals: number
): void {
  console.log("\n   Account Balances:");
  balances.forEach((balance, label) => {
    const formatted = fromTokenAmount(balance, decimals).toLocaleString();
    console.log(`     ${label.padEnd(30)} ${formatted}`);
  });
  console.log();
}

/**
 * Asserts that two BN values are approximately equal within a tolerance
 * @param actual Actual value
 * @param expected Expected value
 * @param tolerance Tolerance (in base units)
 * @param message Optional error message
 */
export function assertApproximately(
  actual: anchor.BN,
  expected: anchor.BN,
  tolerance: anchor.BN,
  message?: string
): void {
  const diff = actual.sub(expected).abs();

  const errorMessage = message ||
    `Expected ${expected.toString()} ± ${tolerance.toString()}, but got ${actual.toString()} (diff: ${diff.toString()})`;

  assert.isTrue(
    diff.lte(tolerance),
    errorMessage
  );
}

/**
 * Logs a calculation breakdown for debugging
 * @param label Calculation description
 * @param values Map of variable names to values
 * @param decimals Token decimals (optional)
 */
export function logCalculation(
  label: string,
  values: Map<string, anchor.BN | number>,
  decimals?: number
): void {
  console.log(`\n   ${label}:`);
  values.forEach((value, name) => {
    if (typeof value === "number") {
      console.log(`     ${name} = ${value}`);
    } else if (decimals !== undefined) {
      const formatted = fromTokenAmount(value, decimals).toLocaleString();
      console.log(`     ${name} = ${formatted} (${value.toString()} base units)`);
    } else {
      console.log(`     ${name} = ${value.toString()}`);
    }
  });
  console.log();
}
