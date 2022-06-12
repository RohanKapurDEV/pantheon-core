import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Autodca } from "../target/types/autodca";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  mintToChecked,
  createAccount,
} from "@solana/spl-token";

describe("autodca", () => {
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

  // Initialize DCA Metadata accounts
  const userPayer = anchor.web3.Keypair.generate();
  let userToMintTokenAccount: anchor.web3.PublicKey;
  let userFromMintTokenAccount: anchor.web3.PublicKey;

  // Param objects
  let initializeDcaMetadataParams = {
    amountPerInterval: 100 * Math.pow(10, mintDecimals),
    intervalLength: intervalLength,
    maxIntervals: maxIntervals,
  };

  before(async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        authorityPayer.publicKey,
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

    // Create user token account - to
    let userToMintTokenAccountPubkey = await createAccount(
      provider.connection,
      userPayer,
      toPaymentMint,
      userPayer.publicKey
    );
    userFromMintTokenAccount = userToMintTokenAccountPubkey;

    // Create user token account - from
    let userFromMintTokenAccountPubkey = await createAccount(
      provider.connection,
      userPayer,
      fromPaymentMint,
      userPayer.publicKey
    );
    userFromMintTokenAccount = userFromMintTokenAccountPubkey;
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
    await program.methods
      .initializeDcaMetadata(
        initializeDcaMetadataParams.amountPerInterval,
        initializeDcaMetadataParams.intervalLength,
        initializeDcaMetadataParams.maxIntervals
      )
      .accounts({})
      .signers([])
      .rpc();
  });
});
