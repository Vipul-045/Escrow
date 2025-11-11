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

  


})