export type Autodca = {
  version: '0.1.0';
  name: 'autodca';
  instructions: [
    {
      name: 'initializeCrankAuthority';
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'crankAuthority';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'currentAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'crankTreasury';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'feeBps';
          type: 'u16';
        },
      ];
    },
    {
      name: 'transferCrankAuthority';
      accounts: [
        {
          name: 'payer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'crankAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'pendingAuthority';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [];
    },
    {
      name: 'acceptCrankAuthority';
      accounts: [
        {
          name: 'payer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'crankAuthority';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [];
    },
    {
      name: 'setCrankTreasury';
      accounts: [
        {
          name: 'payer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'crankAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'crankTreasury';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [];
    },
    {
      name: 'setCrankFeeBps';
      accounts: [
        {
          name: 'payer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'crankAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'crankTreasury';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'feeBps';
          type: 'u16';
        },
      ];
    },
    {
      name: 'initializeDcaMetadata';
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'crankAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'dcaMetadata';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'fromMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'toMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'fromMintUserTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'toMintUserTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'fromMintVaultTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'toMintVaultTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'programAsSigner';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'associatedTokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'amountPerInterval';
          type: 'u64';
        },
        {
          name: 'intervalLength';
          type: 'u64';
        },
        {
          name: 'maxIntervals';
          type: 'u16';
        },
      ];
    },
    {
      name: 'triggerDcaPayment';
      accounts: [
        {
          name: 'payer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'crankAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'dcaMetadata';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'fromMintCrankAuthorityTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'fromMintVaultTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'fromMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'programAsSigner';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [];
    },
    {
      name: 'withdrawTokenFromMetadata';
      accounts: [
        {
          name: 'payer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'dcaMetadata';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'crankAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'fromMintUserTokenAccount';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'toMintUserTokenAccount';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'fromMintVaultTokenAccount';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'toMintVaultTokenAccount';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'fromMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'toMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'fromToken';
          type: 'bool';
        },
        {
          name: 'amount';
          type: 'u64';
        },
      ];
    },
    {
      name: 'closeDcaMetadata';
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'dcaMetadata';
          isMut: true;
          isSigner: false;
        },
      ];
      args: [];
    },
  ];
  accounts: [
    {
      name: 'crankAuthority';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'currentAuthority';
            type: 'publicKey';
          },
          {
            name: 'pendingAuthority';
            type: 'publicKey';
          },
          {
            name: 'crankTreasury';
            type: 'publicKey';
          },
          {
            name: 'crankFeeBps';
            type: 'u16';
          },
        ];
      };
    },
    {
      name: 'dcaMetadata';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'owner';
            type: 'publicKey';
          },
          {
            name: 'fromTokenMint';
            type: 'publicKey';
          },
          {
            name: 'toTokenMint';
            type: 'publicKey';
          },
          {
            name: 'ownerFromTokenAccount';
            type: 'publicKey';
          },
          {
            name: 'ownerToTokenAccount';
            type: 'publicKey';
          },
          {
            name: 'vaultFromTokenAccount';
            type: 'publicKey';
          },
          {
            name: 'vaultToTokenAccount';
            type: 'publicKey';
          },
          {
            name: 'amountPerInterval';
            type: 'u64';
          },
          {
            name: 'intervalLength';
            type: 'u64';
          },
          {
            name: 'intervalCounter';
            type: 'u16';
          },
          {
            name: 'maxIntervals';
            type: 'u16';
          },
          {
            name: 'crankAuthority';
            type: 'publicKey';
          },
          {
            name: 'createdAt';
            type: 'u64';
          },
        ];
      };
    },
  ];
  errors: [
    {
      code: 6000;
      name: 'InvalidFeeBpsParameter';
      msg: 'Fee bps must be equal to or below 10000';
    },
    {
      code: 6001;
      name: 'InvalidCrankAuthority';
      msg: 'Invalid crank authority';
    },
    {
      code: 6002;
      name: 'InvalidPendingAuthority';
      msg: 'Invalid pending authority';
    },
    {
      code: 6003;
      name: 'TokenMintsCannotBeTheSame';
      msg: 'From and To token mints cannot be the same';
    },
    {
      code: 6004;
      name: 'IncorrectMint';
      msg: 'Incorrect token mint supplied';
    },
    {
      code: 6005;
      name: 'CurrentCrankAuthorityNotSigner';
      msg: 'Instruction not signed by current crank authority';
    },
    {
      code: 6006;
      name: 'CurrentCrankDoesNotOwnTokenAccount';
      msg: 'The current crank authority does not own the token account';
    },
    {
      code: 6007;
      name: 'CurrentIntervalOutOfBounds';
      msg: 'The current interval is higher than the max set by the user';
    },
    {
      code: 6008;
      name: 'DcaScheduleInViolation';
      msg: 'The payment schedule initially set by the owner is being violated';
    },
    {
      code: 6009;
      name: 'IncorrectOwner';
      msg: 'Only the owner of the account can call the close instruction';
    },
    {
      code: 6010;
      name: 'IncorrectFromMintTokenAccount';
      msg: 'Incorrect from mint token account';
    },
    {
      code: 6011;
      name: 'IncorrectToMintTokenAccount';
      msg: 'Incorrect to mint token account';
    },
    {
      code: 6012;
      name: 'InsufficientBalanceInSelectedTokenAccount';
      msg: 'Insufficient balance in selected token account';
    },
    {
      code: 6013;
      name: 'InsufficientFundingBalanceInTokenAccount';
      msg: 'Insufficient funding balance in token account';
    },
  ];
};

export const IDL: Autodca = {
  version: '0.1.0',
  name: 'autodca',
  instructions: [
    {
      name: 'initializeCrankAuthority',
      accounts: [
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'crankAuthority',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'currentAuthority',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'crankTreasury',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: 'feeBps',
          type: 'u16',
        },
      ],
    },
    {
      name: 'transferCrankAuthority',
      accounts: [
        {
          name: 'payer',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'crankAuthority',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'pendingAuthority',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: 'acceptCrankAuthority',
      accounts: [
        {
          name: 'payer',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'crankAuthority',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: 'setCrankTreasury',
      accounts: [
        {
          name: 'payer',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'crankAuthority',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'crankTreasury',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: 'setCrankFeeBps',
      accounts: [
        {
          name: 'payer',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'crankAuthority',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'crankTreasury',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: 'feeBps',
          type: 'u16',
        },
      ],
    },
    {
      name: 'initializeDcaMetadata',
      accounts: [
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'crankAuthority',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'dcaMetadata',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'fromMint',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'toMint',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'fromMintUserTokenAccount',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'toMintUserTokenAccount',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'fromMintVaultTokenAccount',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'toMintVaultTokenAccount',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'programAsSigner',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'associatedTokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: 'amountPerInterval',
          type: 'u64',
        },
        {
          name: 'intervalLength',
          type: 'u64',
        },
        {
          name: 'maxIntervals',
          type: 'u16',
        },
      ],
    },
    {
      name: 'triggerDcaPayment',
      accounts: [
        {
          name: 'payer',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'crankAuthority',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'dcaMetadata',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'fromMintCrankAuthorityTokenAccount',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'fromMintVaultTokenAccount',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'fromMint',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'programAsSigner',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: 'withdrawTokenFromMetadata',
      accounts: [
        {
          name: 'payer',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'dcaMetadata',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'crankAuthority',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'fromMintUserTokenAccount',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'toMintUserTokenAccount',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'fromMintVaultTokenAccount',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'toMintVaultTokenAccount',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'fromMint',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'toMint',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: 'fromToken',
          type: 'bool',
        },
        {
          name: 'amount',
          type: 'u64',
        },
      ],
    },
    {
      name: 'closeDcaMetadata',
      accounts: [
        {
          name: 'payer',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'dcaMetadata',
          isMut: true,
          isSigner: false,
        },
      ],
      args: [],
    },
  ],
  accounts: [
    {
      name: 'crankAuthority',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'currentAuthority',
            type: 'publicKey',
          },
          {
            name: 'pendingAuthority',
            type: 'publicKey',
          },
          {
            name: 'crankTreasury',
            type: 'publicKey',
          },
          {
            name: 'crankFeeBps',
            type: 'u16',
          },
        ],
      },
    },
    {
      name: 'dcaMetadata',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'owner',
            type: 'publicKey',
          },
          {
            name: 'fromTokenMint',
            type: 'publicKey',
          },
          {
            name: 'toTokenMint',
            type: 'publicKey',
          },
          {
            name: 'ownerFromTokenAccount',
            type: 'publicKey',
          },
          {
            name: 'ownerToTokenAccount',
            type: 'publicKey',
          },
          {
            name: 'vaultFromTokenAccount',
            type: 'publicKey',
          },
          {
            name: 'vaultToTokenAccount',
            type: 'publicKey',
          },
          {
            name: 'amountPerInterval',
            type: 'u64',
          },
          {
            name: 'intervalLength',
            type: 'u64',
          },
          {
            name: 'intervalCounter',
            type: 'u16',
          },
          {
            name: 'maxIntervals',
            type: 'u16',
          },
          {
            name: 'crankAuthority',
            type: 'publicKey',
          },
          {
            name: 'createdAt',
            type: 'u64',
          },
        ],
      },
    },
  ],
  errors: [
    {
      code: 6000,
      name: 'InvalidFeeBpsParameter',
      msg: 'Fee bps must be equal to or below 10000',
    },
    {
      code: 6001,
      name: 'InvalidCrankAuthority',
      msg: 'Invalid crank authority',
    },
    {
      code: 6002,
      name: 'InvalidPendingAuthority',
      msg: 'Invalid pending authority',
    },
    {
      code: 6003,
      name: 'TokenMintsCannotBeTheSame',
      msg: 'From and To token mints cannot be the same',
    },
    {
      code: 6004,
      name: 'IncorrectMint',
      msg: 'Incorrect token mint supplied',
    },
    {
      code: 6005,
      name: 'CurrentCrankAuthorityNotSigner',
      msg: 'Instruction not signed by current crank authority',
    },
    {
      code: 6006,
      name: 'CurrentCrankDoesNotOwnTokenAccount',
      msg: 'The current crank authority does not own the token account',
    },
    {
      code: 6007,
      name: 'CurrentIntervalOutOfBounds',
      msg: 'The current interval is higher than the max set by the user',
    },
    {
      code: 6008,
      name: 'DcaScheduleInViolation',
      msg: 'The payment schedule initially set by the owner is being violated',
    },
    {
      code: 6009,
      name: 'IncorrectOwner',
      msg: 'Only the owner of the account can call the close instruction',
    },
    {
      code: 6010,
      name: 'IncorrectFromMintTokenAccount',
      msg: 'Incorrect from mint token account',
    },
    {
      code: 6011,
      name: 'IncorrectToMintTokenAccount',
      msg: 'Incorrect to mint token account',
    },
    {
      code: 6012,
      name: 'InsufficientBalanceInSelectedTokenAccount',
      msg: 'Insufficient balance in selected token account',
    },
    {
      code: 6013,
      name: 'InsufficientFundingBalanceInTokenAccount',
      msg: 'Insufficient funding balance in token account',
    },
  ],
};
