import { Injectable } from '@nestjs/common';
import { Connection, Keypair } from '@solana/web3.js';
import * as anchor from '@project-serum/anchor';
import {
  JupToken,
  executeSwap,
  getRoutes,
  AUTODCA_PROGRAM_ADDRESS,
} from '../utils/index';
import { Program } from '@project-serum/anchor';
import { Autodca, IDL } from '../utils/autodca';
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  Token,
} from '@solana/spl-token';
import { Jupiter, TOKEN_LIST_URL } from '@jup-ag/core';
import { ConfigService } from '@nestjs/config';
import { bs58 } from '@project-serum/anchor/dist/cjs/utils/bytes';

@Injectable()
export class SolanaService {
  constructor(private configService: ConfigService) {}

  async currentCrankAuthority() {
    const crankAuthorityWallet =
      this.configService.get<string>('WALLET_PRIVATE_KEY');
    const walletArray = bs58.decode(crankAuthorityWallet);
    const CRANK_AUTHORITY_KEYPAIR = Keypair.fromSecretKey(walletArray);

    return CRANK_AUTHORITY_KEYPAIR.publicKey.toBase58();
  }

  async withdrawTokenFromPantheonVault({
    network,
    vaultFromTokenMint,
    vaultToTokenMint,
    dcaMetadataAddress,
  }: {
    network: string;
    vaultFromTokenMint: string;
    vaultToTokenMint: string;
    dcaMetadataAddress: string;
  }) {
    const crankAuthorityWallet =
      this.configService.get<string>('WALLET_PRIVATE_KEY');
    const walletArray = bs58.decode(crankAuthorityWallet);
    const CRANK_AUTHORITY_KEYPAIR = Keypair.fromSecretKey(walletArray);

    // Network param used to determine RPC endpoint
    const connection =
      network == 'mainnet-beta'
        ? new Connection(process.env.SOLANA_RPC_HTTP_MAINNET_URL)
        : new Connection(process.env.SOLANA_RPC_HTTP_DEVNET_URL);

    const wallet = new anchor.Wallet(CRANK_AUTHORITY_KEYPAIR);
    const provider = new anchor.AnchorProvider(connection, wallet, {
      commitment: 'confirmed',
      maxRetries: 10,
    });

    // Build program instance using local copy of the AutoDCA program IDL
    const program = new anchor.Program(
      IDL,
      AUTODCA_PROGRAM_ADDRESS,
      provider,
    ) as Program<Autodca>;

    const crankAuthorityPubkey = wallet.publicKey;

    const autodcaProgramPubkey = new anchor.web3.PublicKey(
      AUTODCA_PROGRAM_ADDRESS,
    );

    const vaultFromTokenMintPubkey = new anchor.web3.PublicKey(
      vaultFromTokenMint,
    );
    const vaultToTokenMintPubkey = new anchor.web3.PublicKey(vaultToTokenMint);

    const dcaMetadataPubkey = new anchor.web3.PublicKey(dcaMetadataAddress);

    let dcaMetadataAccount: any;

    try {
      const account = await program.account.dcaMetadata.fetch(
        dcaMetadataPubkey,
      );

      dcaMetadataAccount = account;
    } catch (error) {
      throw new Error(`Could not fetch DCA metadata account. Error: ${error}`);
    }

    const accountAssociatedCrankAuthority = dcaMetadataAccount.crankAuthority;

    // Make sure that the crank authority is the same as the one that is associated with the DCA metadata account
    if (accountAssociatedCrankAuthority !== crankAuthorityPubkey) {
      throw new Error(
        `Account ${dcaMetadataPubkey} is not associated with crank authority ${crankAuthorityPubkey}`,
      );
    }

    const vaultFromTokenAccount = dcaMetadataAccount.vaultFromTokenAccount;

    // Compute the ATA for the crank authority, given the vault from token mint
    const [vaultFromTokenCrankAuthorityAta] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          crankAuthorityPubkey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          vaultFromTokenMintPubkey.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID,
      );

    const fromToken = new Token(
      connection,
      vaultFromTokenMintPubkey,
      TOKEN_PROGRAM_ID,
      CRANK_AUTHORITY_KEYPAIR,
    );

    const toToken = new Token(
      connection,
      vaultToTokenMintPubkey,
      TOKEN_PROGRAM_ID,
      CRANK_AUTHORITY_KEYPAIR,
    );

    try {
      const tryAccessingFromMintAta =
        await fromToken.getOrCreateAssociatedAccountInfo(crankAuthorityPubkey);

      console.log(
        `Created the following from mint ATA: ${tryAccessingFromMintAta.address.toString()}`,
      );
    } catch (error) {
      throw new Error(
        'Could not get or create associated account info for vault from mint. Error: ' +
          error,
      );
    }

    try {
      const tryAccessingToMintAta =
        await toToken.getOrCreateAssociatedAccountInfo(crankAuthorityPubkey);

      console.log(
        `Created the following to mint ATA: ${tryAccessingToMintAta.address.toString()}`,
      );
    } catch (error) {
      throw new Error(
        'Could not get or create associated account info for vault to mint. Error: ' +
          error,
      );
    }

    const [programAsSignerPubkey] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from('program'), Buffer.from('signer')],
        autodcaProgramPubkey,
      );

    // Call the AutoDCA program to withdraw the token from the vault
    const tx = await program.methods
      .triggerDcaPayment()
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
        payer: crankAuthorityPubkey,
        crankAuthority: accountAssociatedCrankAuthority,
        dcaMetadata: dcaMetadataPubkey,
        fromMint: vaultFromTokenMintPubkey,
        programAsSigner: programAsSignerPubkey,
        fromMintVaultTokenAccount: vaultFromTokenAccount,
        fromMintCrankAuthorityTokenAccount: vaultFromTokenCrankAuthorityAta,
      })
      .signers([CRANK_AUTHORITY_KEYPAIR])
      .rpc();

    console.log('Withdrawal successful');

    return tx.toString();
  }

  async performSwap({
    network,
    vaultFromTokenMint,
    vaultToTokenMint,
    inputAmount,
    slippage,
  }: {
    network: string;
    vaultFromTokenMint: string;
    vaultToTokenMint: string;
    inputAmount: number;
    slippage: number;
  }) {
    const crankAuthorityWallet =
      this.configService.get<string>('WALLET_PRIVATE_KEY');
    const walletArray = bs58.decode(crankAuthorityWallet);
    const CRANK_AUTHORITY_KEYPAIR = Keypair.fromSecretKey(walletArray);

    // Network param used to determine RPC endpoint
    const connection =
      network == 'mainnet-beta'
        ? new Connection(process.env.SOLANA_RPC_HTTP_MAINNET_URL)
        : new Connection(process.env.SOLANA_RPC_HTTP_DEVNET_URL);
    const cluster = network == 'mainnet-beta' ? 'mainnet-beta' : 'devnet';

    let tokens: JupToken[] = [];

    try {
      const tryFetchingTokens: JupToken[] = await (
        await fetch(TOKEN_LIST_URL[cluster])
      ).json(); // Fetch token list from Jupiter API

      tokens = tryFetchingTokens;
    } catch (error) {
      throw new Error("Couldn't fetch token list from Jupiter API");
    }

    const jupiter = await Jupiter.load({
      connection,
      cluster,
      user: CRANK_AUTHORITY_KEYPAIR,
      wrapUnwrapSOL: false,
    });

    const inputToken = tokens.find((t) => t.address === vaultFromTokenMint);
    const outputToken = tokens.find((t) => t.address === vaultToTokenMint);

    const routes = await getRoutes({
      jupiter,
      inputToken,
      outputToken,
      inputAmount, // 1 unit in UI
      slippage, // 1% slippage
    });

    try {
      const [txid, inputAmount, outputAmount] = await executeSwap({
        jupiter,
        routeInfo: routes.routesInfos[0],
      });

      return [txid, inputAmount, outputAmount];
    } catch (error) {
      throw new Error('Could not execute swap. Error: ' + error);
    }
  }

  async depositBackToPantheonVault({
    network,
    vaultToTokenMint,
    dcaMetadataAddress,
    outputAmount,
  }: {
    network: string;
    vaultToTokenMint: string;
    dcaMetadataAddress: string;
    outputAmount: number;
  }) {
    const crankAuthorityWallet =
      this.configService.get<string>('WALLET_PRIVATE_KEY');
    const walletArray = bs58.decode(crankAuthorityWallet);
    const CRANK_AUTHORITY_KEYPAIR = Keypair.fromSecretKey(walletArray);

    const connection =
      network == 'mainnet-beta'
        ? new Connection(process.env.SOLANA_RPC_HTTP_MAINNET_URL)
        : new Connection(process.env.SOLANA_RPC_HTTP_DEVNET_URL);

    const wallet = new anchor.Wallet(CRANK_AUTHORITY_KEYPAIR);
    const provider = new anchor.AnchorProvider(connection, wallet, {
      commitment: 'confirmed',
      maxRetries: 10,
    });

    const program = new anchor.Program(
      IDL,
      AUTODCA_PROGRAM_ADDRESS,
      provider,
    ) as Program<Autodca>;

    const crankAuthorityPubkey = wallet.publicKey;

    const vaultToTokenMintPubkey = new anchor.web3.PublicKey(vaultToTokenMint);

    const [vaultToTokenCrankAuthorityAta] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          crankAuthorityPubkey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          vaultToTokenMintPubkey.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID,
      );

    const dcaMetadataPubkey = new anchor.web3.PublicKey(dcaMetadataAddress);

    let dcaMetadataAccount: any;

    try {
      const account = await program.account.dcaMetadata.fetch(
        dcaMetadataPubkey,
      );

      dcaMetadataAccount = account;
    } catch (error) {
      throw new Error(`Could not fetch DCA metadata account. Error: ${error}`);
    }

    const dcaMetadataToTokenAccount = dcaMetadataAccount.vaultToTokenAccount;

    const toToken = new Token(
      connection,
      vaultToTokenMintPubkey,
      TOKEN_PROGRAM_ID,
      CRANK_AUTHORITY_KEYPAIR,
    );

    let transferTxid: string;

    try {
      const tryTransfer = await toToken.transfer(
        vaultToTokenCrankAuthorityAta,
        dcaMetadataToTokenAccount,
        CRANK_AUTHORITY_KEYPAIR,
        [CRANK_AUTHORITY_KEYPAIR],
        outputAmount,
      );

      transferTxid = tryTransfer.toString();
    } catch (error) {
      throw new Error("Couldn't transfer to vault. Error: " + error);
    }

    return transferTxid;
  }
}
