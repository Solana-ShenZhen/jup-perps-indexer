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
  AddLiquidityParams,
  AddLiquidityParamsArgs,
  getAddLiquidityParamsSerializer,
} from '../types';

// Accounts.
export type AddLiquidityInstructionAccounts = {
  owner: Signer;
  fundingAccount: PublicKey | Pda;
  lpTokenAccount: PublicKey | Pda;
  transferAuthority: PublicKey | Pda;
  perpetuals: PublicKey | Pda;
  pool: PublicKey | Pda;
  custody: PublicKey | Pda;
  custodyOracleAccount: PublicKey | Pda;
  custodyTokenAccount: PublicKey | Pda;
  lpTokenMint: PublicKey | Pda;
  tokenProgram: PublicKey | Pda;
  eventAuthority: PublicKey | Pda;
  program: PublicKey | Pda;
};

// Data.
export type AddLiquidityInstructionData = {
  discriminator: Uint8Array;
  params: AddLiquidityParams;
};

export type AddLiquidityInstructionDataArgs = {
  params: AddLiquidityParamsArgs;
};

export function getAddLiquidityInstructionDataSerializer(): Serializer<
  AddLiquidityInstructionDataArgs,
  AddLiquidityInstructionData
> {
  return mapSerializer<
    AddLiquidityInstructionDataArgs,
    any,
    AddLiquidityInstructionData
  >(
    struct<AddLiquidityInstructionData>(
      [
        ['discriminator', bytes({ size: 8 })],
        ['params', getAddLiquidityParamsSerializer()],
      ],
      { description: 'AddLiquidityInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([181, 157, 89, 67, 143, 182, 52, 72]),
    })
  ) as Serializer<AddLiquidityInstructionDataArgs, AddLiquidityInstructionData>;
}

// Args.
export type AddLiquidityInstructionArgs = AddLiquidityInstructionDataArgs;

// Instruction.
export function addLiquidity(
  context: Pick<Context, 'programs'>,
  input: AddLiquidityInstructionAccounts & AddLiquidityInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey('perpetuals', '');

  // Accounts.
  const resolvedAccounts = {
    owner: {
      index: 0,
      isWritable: false as boolean,
      value: input.owner ?? null,
    },
    fundingAccount: {
      index: 1,
      isWritable: true as boolean,
      value: input.fundingAccount ?? null,
    },
    lpTokenAccount: {
      index: 2,
      isWritable: true as boolean,
      value: input.lpTokenAccount ?? null,
    },
    transferAuthority: {
      index: 3,
      isWritable: false as boolean,
      value: input.transferAuthority ?? null,
    },
    perpetuals: {
      index: 4,
      isWritable: false as boolean,
      value: input.perpetuals ?? null,
    },
    pool: { index: 5, isWritable: true as boolean, value: input.pool ?? null },
    custody: {
      index: 6,
      isWritable: true as boolean,
      value: input.custody ?? null,
    },
    custodyOracleAccount: {
      index: 7,
      isWritable: false as boolean,
      value: input.custodyOracleAccount ?? null,
    },
    custodyTokenAccount: {
      index: 8,
      isWritable: true as boolean,
      value: input.custodyTokenAccount ?? null,
    },
    lpTokenMint: {
      index: 9,
      isWritable: true as boolean,
      value: input.lpTokenMint ?? null,
    },
    tokenProgram: {
      index: 10,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
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
  const resolvedArgs: AddLiquidityInstructionArgs = { ...input };

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
  const data = getAddLiquidityInstructionDataSerializer().serialize(
    resolvedArgs as AddLiquidityInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
