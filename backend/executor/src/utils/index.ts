export * from './jupiter';

// Wallets
// export const WALLET_PRIVATE_KEY = process.env.WALLET_PRIVATE_KEY;
// export const CRANK_AUTHORITY_PRIVATE_KEY = bs58.decode(WALLET_PRIVATE_KEY);
// export const CRANK_AUTHORITY_KEYPAIR = Keypair.fromSecretKey(
//   CRANK_AUTHORITY_PRIVATE_KEY,
// );

export const AUTODCA_PROGRAM_ADDRESS =
  'dca6xdPrxUTazoTEq7ue51nhWWSH2efXRBJhYrxHB4W';

export const selectCluster = (cluster: string) => {
  switch (cluster) {
    case 'mainnet':
      return 'mainnet-beta';
    case 'devnet':
      return 'devnet';
    default:
      return 'mainnet-beta';
  }
};
