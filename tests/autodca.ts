import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Autodca } from "../target/types/autodca";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  mintToChecked,
  createAccount,
} from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import { SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import BN from "bn.js";
import { delay } from "./utils";

describe("autodca", async () => {
  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.Autodca as Program<Autodca>;

  // Misc variables
  const mintDecimals = 6;
  const intervalLength = 10; // in seconds
  const maxIntervals = 3;
  let toPaymentMint: anchor.web3.PublicKey;
  let fromPaymentMint: anchor.web3.PublicKey;

  // Initialize Crank Authority accounts
  const authorityPayer = anchor.web3.Keypair.generate();
  const crankAuthority = anchor.web3.Keypair.generate();
  const currentAuthority = anchor.web3.Keypair.generate();
  const crankTreasury = anchor.web3.Keypair.generate();

  // Initialize DCA Metadata accounts - rest instantiated inside instruction test
  const userPayer = anchor.web3.Keypair.generate();
  let userFromMintTokenAccount: anchor.web3.PublicKey;
  const dcaMetadata = anchor.web3.Keypair.generate();

  // Trigger DCA Payment accounts
  let currentAuthorityFromMintTokenAccount: anchor.web3.PublicKey;

  // Param objects
  let initializeDcaMetadataParams = {
    amountPerInterval: 100 * Math.pow(10, mintDecimals),
    intervalLength: intervalLength,
    maxIntervals: maxIntervals,
  };

  let [programAsSigner] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("program"), Buffer.from("signer")],
    program.programId
  );

  before(async () => {
    // Fund authorityPayer with 10 SOL
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        authorityPayer.publicKey,
        10_000_000_000
      ), // 10 SOL
      "confirmed"
    );

    // Fund userPayer with 10 SOL
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        userPayer.publicKey,
        10_000_000_000
      ), // 10 SOL
      "confirmed"
    );

    // Fund currentAuthority with 10 SOL
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        currentAuthority.publicKey,
        10_000_000_000
      ), // 10 SOL
      "confirmed"
    );

    // Create _to_ token mint
    let toMint = await createMint(
      provider.connection,
      authorityPayer,
      authorityPayer.publicKey,
      authorityPayer.publicKey,
      mintDecimals
    );
    toPaymentMint = toMint;

    // Create _from_ token mint
    let fromMint = await createMint(
      provider.connection,
      authorityPayer,
      authorityPayer.publicKey,
      authorityPayer.publicKey,
      mintDecimals
    );
    fromPaymentMint = fromMint;

    // Create user token account - from
    let userFromMintTokenAccountPubkey = await createAccount(
      provider.connection,
      userPayer,
      fromPaymentMint,
      userPayer.publicKey
    );
    userFromMintTokenAccount = userFromMintTokenAccountPubkey;

    // Mint fromMint tokens to user from token account
    await mintToChecked(
      provider.connection,
      userPayer,
      fromMint,
      userFromMintTokenAccount,
      authorityPayer,
      10000000000,
      mintDecimals
    );

    // Create currentAuthority token account - from
    let currentAuthorityFromMintTokenAccountPubkey = await createAccount(
      provider.connection,
      currentAuthority,
      fromPaymentMint,
      currentAuthority.publicKey
    );
    currentAuthorityFromMintTokenAccount =
      currentAuthorityFromMintTokenAccountPubkey;
  });

  it("creates a CrankAuthority account!", async () => {
    await program.methods
      .initializeCrankAuthority(100)
      .accounts({
        payer: authorityPayer.publicKey,
        crankAuthority: crankAuthority.publicKey,
        currentAuthority: currentAuthority.publicKey,
        crankTreasury: crankTreasury.publicKey,
      })
      .signers([authorityPayer, crankAuthority])
      .rpc();

    // let crankAuthorityAccount = await program.account.crankAuthority.fetch(
    //   crankAuthority.publicKey
    // );
  });

  it("creates a DCA metadata account!", async () => {
    // User To mint ATA
    let [userToMintTokenPublicKey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          userPayer.publicKey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          toPaymentMint.toBuffer(),
        ],
        ASSOCIATED_PROGRAM_ID
      );

    let [vaultFromMintTokenPublicKey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("vault"), fromPaymentMint.toBuffer()],
        program.programId
      );

    let [vaultToMintTokenPublicKey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("vault"), toPaymentMint.toBuffer()],
        program.programId
      );

    await program.methods
      .initializeDcaMetadata(
        new BN(initializeDcaMetadataParams.amountPerInterval),
        new BN(initializeDcaMetadataParams.intervalLength),
        initializeDcaMetadataParams.maxIntervals
      )
      .accounts({
        payer: userPayer.publicKey,
        crankAuthority: crankAuthority.publicKey,
        dcaMetadata: dcaMetadata.publicKey,
        fromMint: fromPaymentMint,
        toMint: toPaymentMint,
        fromMintUserTokenAccount: userFromMintTokenAccount,
        toMintUserTokenAccount: userToMintTokenPublicKey,
        fromMintVaultTokenAccount: vaultFromMintTokenPublicKey,
        toMintVaultTokenAccount: vaultToMintTokenPublicKey,
        programAsSigner: programAsSigner,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([userPayer, dcaMetadata])
      .rpc();
  });

  it("triggers a DCA payment", async () => {
    let [vaultFromMintTokenPublicKey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("vault"), fromPaymentMint.toBuffer()],
        program.programId
      );

    let dcaAccount = await program.account.dcaMetadata.fetch(
      dcaMetadata.publicKey
    );

    // console.log(parseInt(dcaAccount.createdAt));
    // console.log(dcaAccount.createdAt + dcaAccount.intervalLength);
    // console.log(Date.now());

    // plus one to intervalLength to offset decimal precision loss
    await delay((intervalLength + 1) * 1000).then(async () => {
      await program.methods
        .triggerDcaPayment()
        .accounts({
          payer: currentAuthority.publicKey,
          crankAuthority: crankAuthority.publicKey,
          dcaMetadata: dcaMetadata.publicKey,
          fromMintCrankAuthorityTokenAccount:
            currentAuthorityFromMintTokenAccount,
          fromMintVaultTokenAccount: vaultFromMintTokenPublicKey,
          fromMint: fromPaymentMint,
          programAsSigner: programAsSigner,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([currentAuthority])
        .rpc();
    });

    await delay((intervalLength + 1) * 1000).then(async () => {
      await program.methods
        .triggerDcaPayment()
        .accounts({
          payer: currentAuthority.publicKey,
          crankAuthority: crankAuthority.publicKey,
          dcaMetadata: dcaMetadata.publicKey,
          fromMintCrankAuthorityTokenAccount:
            currentAuthorityFromMintTokenAccount,
          fromMintVaultTokenAccount: vaultFromMintTokenPublicKey,
          fromMint: fromPaymentMint,
          programAsSigner: programAsSigner,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([currentAuthority])
        .rpc();
    });

    await delay((intervalLength + 1) * 1000).then(async () => {
      await program.methods
        .triggerDcaPayment()
        .accounts({
          payer: currentAuthority.publicKey,
          crankAuthority: crankAuthority.publicKey,
          dcaMetadata: dcaMetadata.publicKey,
          fromMintCrankAuthorityTokenAccount:
            currentAuthorityFromMintTokenAccount,
          fromMintVaultTokenAccount: vaultFromMintTokenPublicKey,
          fromMint: fromPaymentMint,
          programAsSigner: programAsSigner,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([currentAuthority])
        .rpc();
    });
  });
});
