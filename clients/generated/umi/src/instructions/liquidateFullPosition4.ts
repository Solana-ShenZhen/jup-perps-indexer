/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  bytes,
  mapSerializer,
  struct,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';
import {
  LiquidateFullPosition4Params,
  LiquidateFullPosition4ParamsArgs,
  getLiquidateFullPosition4ParamsSerializer,
} from '../types';

// Accounts.
export type LiquidateFullPosition4InstructionAccounts = {
  signer: Signer;
  perpetuals: PublicKey | Pda;
  pool: PublicKey | Pda;
  position: PublicKey | Pda;
  custody: PublicKey | Pda;
  custodyDovesPriceAccount: PublicKey | Pda;
  custodyPythnetPriceAccount: PublicKey | Pda;
  collateralCustody: PublicKey | Pda;
  collateralCustodyDovesPriceAccount: PublicKey | Pda;
  collateralCustodyPythnetPriceAccount: PublicKey | Pda;
  collateralCustodyTokenAccount: PublicKey | Pda;
  eventAuthority: PublicKey | Pda;
  program: PublicKey | Pda;
};

// Data.
export type LiquidateFullPosition4InstructionData = {
  discriminator: Uint8Array;
  params: LiquidateFullPosition4Params;
};

export type LiquidateFullPosition4InstructionDataArgs = {
  params: LiquidateFullPosition4ParamsArgs;
};

export function getLiquidateFullPosition4InstructionDataSerializer(): Serializer<
  LiquidateFullPosition4InstructionDataArgs,
  LiquidateFullPosition4InstructionData
> {
  return mapSerializer<
    LiquidateFullPosition4InstructionDataArgs,
    any,
    LiquidateFullPosition4InstructionData
  >(
    struct<LiquidateFullPosition4InstructionData>(
      [
        ['discriminator', bytes({ size: 8 })],
        ['params', getLiquidateFullPosition4ParamsSerializer()],
      ],
      { description: 'LiquidateFullPosition4InstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([64, 176, 88, 51, 168, 188, 156, 175]),
    })
  ) as Serializer<
    LiquidateFullPosition4InstructionDataArgs,
    LiquidateFullPosition4InstructionData
  >;
}

// Args.
export type LiquidateFullPosition4InstructionArgs =
  LiquidateFullPosition4InstructionDataArgs;

// Instruction.
export function liquidateFullPosition4(
  context: Pick<Context, 'programs'>,
  input: LiquidateFullPosition4InstructionAccounts &
    LiquidateFullPosition4InstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey('perpetuals', '');

  // Accounts.
  const resolvedAccounts = {
    signer: {
      index: 0,
      isWritable: false as boolean,
      value: input.signer ?? null,
    },
    perpetuals: {
      index: 1,
      isWritable: false as boolean,
      value: input.perpetuals ?? null,
    },
    pool: { index: 2, isWritable: true as boolean, value: input.pool ?? null },
    position: {
      index: 3,
      isWritable: true as boolean,
      value: input.position ?? null,
    },
    custody: {
      index: 4,
      isWritable: true as boolean,
      value: input.custody ?? null,
    },
    custodyDovesPriceAccount: {
      index: 5,
      isWritable: false as boolean,
      value: input.custodyDovesPriceAccount ?? null,
    },
    custodyPythnetPriceAccount: {
      index: 6,
      isWritable: false as boolean,
      value: input.custodyPythnetPriceAccount ?? null,
    },
    collateralCustody: {
      index: 7,
      isWritable: true as boolean,
      value: input.collateralCustody ?? null,
    },
    collateralCustodyDovesPriceAccount: {
      index: 8,
      isWritable: false as boolean,
      value: input.collateralCustodyDovesPriceAccount ?? null,
    },
    collateralCustodyPythnetPriceAccount: {
      index: 9,
      isWritable: false as boolean,
      value: input.collateralCustodyPythnetPriceAccount ?? null,
    },
    collateralCustodyTokenAccount: {
      index: 10,
      isWritable: true as boolean,
      value: input.collateralCustodyTokenAccount ?? null,
    },
    eventAuthority: {
      index: 11,
      isWritable: false as boolean,
      value: input.eventAuthority ?? null,
    },
    program: {
      index: 12,
      isWritable: false as boolean,
      value: input.program ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: LiquidateFullPosition4InstructionArgs = { ...input };

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getLiquidateFullPosition4InstructionDataSerializer().serialize(
    resolvedArgs as LiquidateFullPosition4InstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
