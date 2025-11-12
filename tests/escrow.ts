import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import {
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Keypair,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  createMint,
  createAccount,
  mintTo,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  createAssociatedTokenAccount,
} from "@solana/spl-token";
const { assert } = require("chai");

describe("escrow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;

  let mint: PublicKey;
  let initializerTokenAccount: PublicKey;
  let receiverTokenAccount: PublicKey;
  let escrowKeypair: Keypair;
  let vaultAuthority: PublicKey;
  let vault: PublicKey;
  let receiver: Keypair

  const escrowAmount = new anchor.BN(100_000_000);
  const expiry = new anchor.BN(Math.floor(Date.now() / 1000) + 60 * 5);

  before(async () => {
    //Create Mint
    mint = await createMint(
      provider.connection,
      (provider.wallet as any).payer,
      provider.publicKey,
      null,
      6
    )

    //Create Initializer token account
    initializerTokenAccount = await createAccount(
      provider.connection,
      (provider.wallet as any).payer,
      mint,
      provider.publicKey
    )

    //Mint some tokens to initializer
    await mintTo(
      provider.connection,
      (provider.wallet as any).payer,
      mint,
      initializerTokenAccount,
      provider.publicKey,
      1_000_000_000
    )

    receiver = Keypair.generate();

    const signature = await provider.connection.requestAirdrop(
      receiver.publicKey,
      1e9
    );

    const latestBlockhash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      signature,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });

    receiverTokenAccount = await createAccount(
      provider.connection,
      (provider.wallet as any).payer,
      mint,
      receiver.publicKey
    )

    escrowKeypair = Keypair.generate();

    [vaultAuthority] = await PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), escrowKeypair.publicKey.toBuffer()],
      program.programId,
    )

    const vaultSeedConst = Buffer.from([
      231, 242, 71, 130, 11, 40, 23, 98,
      134, 142, 64, 33, 218, 211, 26, 221,
      220, 102, 241, 246, 3, 229, 91, 35,
      184, 32, 193, 148, 50, 253, 3, 93
    ]);

    [vault] = await PublicKey.findProgramAddressSync(
      [vaultAuthority.toBuffer(), vaultSeedConst, mint.toBuffer()],
      program.programId,
    );
  });



})