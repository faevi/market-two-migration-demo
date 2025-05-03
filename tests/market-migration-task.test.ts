
import { startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { BN, Program, web3 } from "@coral-xyz/anchor";
import { MarketMigrationTask } from "../target/types/market_migration_task";
import { expect } from "chai";
import { config } from "dotenv";
config();

const MarketMigrationTaskIdl = require("../target/idl/market_migration_task.json");
const MARKET_TWO_PROGRAM_ID = new PublicKey("ExponentnaRg3CQbW6dqQNZKXp7gtZ9DGMp1cwC4HAS7");
const MARKET_TWO_KEY = new PublicKey("EJ4GPTCnNtemBVrT7QKhRfSKfM53aV2UJYGAC8gdVz5b");
const DEFAULT_FLASH_SWAP_FEE_BPS = 1000;
const connection = new Connection(process.env.RPC_URL!, "confirmed");
const secretArray: number[] = JSON.parse(process.env.ADMIN_SECRET_KEY!);
const adminKeypair = Keypair.fromSecretKey(Uint8Array.from(secretArray));
const U32TYPE_SIZE = 4;

function readU32LE(buf: Buffer | Uint8Array, offset: number): number {
  const b = Buffer.isBuffer(buf) ? buf : Buffer.from(buf);
  return b.readUInt32LE(offset);
}

async function getBankrunContext() {
  const marketTwoInfo = await connection.getAccountInfo(MARKET_TWO_KEY);
  return startAnchor(".", [],
    [
      {
        address: MARKET_TWO_KEY,
        info: {
          executable: false,
          owner: MARKET_TWO_PROGRAM_ID,
          lamports: marketTwoInfo!.lamports,
          data: marketTwoInfo!.data,
          rentEpoch: marketTwoInfo!.rentEpoch,
        }
      },
      {
        address: adminKeypair.publicKey,
        info: {
          executable: false,
          owner: SystemProgram.programId,
          data: Buffer.from([]),
          rentEpoch: 0,
          lamports: 1e10
        }
      },
    ]);
}

test("migrate market two add flash swap fee bps", async () => {
  const marketTwoInfo = await connection.getAccountInfo(MARKET_TWO_KEY);

  const context = await getBankrunContext();

  const provider = new BankrunProvider(context);
  const program = new Program<MarketMigrationTask>(MarketMigrationTaskIdl, provider);

  await program.methods
    .migrateMarketTwoFlashFeeBps()
    .accountsStrict({
      admin: adminKeypair.publicKey,
      marketTwo: MARKET_TWO_KEY,
      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
    })
    .signers([adminKeypair])
    .rpc().catch((e) => {
      console.log("Error: ", e);
    });

  const market_two_migrated = await program.account.marketTwo.fetch(MARKET_TWO_KEY);
  const flashSwapFeeBps = market_two_migrated.flashSwapFeeBps

  expect(flashSwapFeeBps).equal(DEFAULT_FLASH_SWAP_FEE_BPS, "flashSwapFeeBps should be DEFAULT_FLASH_SWAP_FEE_BPS, flashSwapFeeBps: " + flashSwapFeeBps);
});

test("migrate market two realloc new field flash swap fee bps", async () => {
  const marketTwoInfo = await connection.getAccountInfo(MARKET_TWO_KEY);

  const context = await getBankrunContext();

  const provider = new BankrunProvider(context);
  const program = new Program<MarketMigrationTask>(MarketMigrationTaskIdl, provider);

  await program.methods
    .migrateMarketTwoFlashFeeBps()
    .accountsStrict({
      admin: adminKeypair.publicKey,
      marketTwo: MARKET_TWO_KEY,
      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
    })
    .signers([adminKeypair])
    .rpc().catch((e) => {
      console.log("Error: ", e);
    });

  const market_two_info_migrated = await provider.connection.getAccountInfo(MARKET_TWO_KEY);

  expect(market_two_info_migrated?.data.length).equal(marketTwoInfo.data.length + U32TYPE_SIZE, "wrong size marketTwo");
});

test("field migration test", async () => {
  const marketTwoInfo = await connection.getAccountInfo(MARKET_TWO_KEY);

  const context = await getBankrunContext();

  const provider = new BankrunProvider(context);
  const program = new Program<MarketMigrationTask>(MarketMigrationTaskIdl, provider);
  const parsedMarketTwoDecoded = program.coder.accounts.decode("marketTwo", marketTwoInfo.data);

  await program.methods
    .migrateMarketTwoFlashFeeBps()
    .accountsStrict({
      admin: adminKeypair.publicKey,
      marketTwo: MARKET_TWO_KEY,
      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
    })
    .signers([adminKeypair])
    .rpc().catch((e) => {
      console.log("Error: ", e);
    });



  // Compare the old and new data
  const parsedMarketTwoMigratedDecoded = await program.account.marketTwo.fetch(MARKET_TWO_KEY);

  // instead of .equal(…)
  expect(parsedMarketTwoMigratedDecoded)
    .to.deep.equal({
      ...parsedMarketTwoDecoded,
      flashSwapFeeBps: DEFAULT_FLASH_SWAP_FEE_BPS,
    }, "old data should be equal to prev data");
  // Check the new field
});

