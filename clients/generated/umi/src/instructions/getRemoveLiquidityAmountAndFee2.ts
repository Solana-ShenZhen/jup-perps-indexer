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
  GetRemoveLiquidityAmountAndFee2Params,
  GetRemoveLiquidityAmountAndFee2ParamsArgs,
  getGetRemoveLiquidityAmountAndFee2ParamsSerializer,
} from '../types';

// Accounts.
export type GetRemoveLiquidityAmountAndFee2InstructionAccounts = {
  perpetuals: PublicKey | Pda;
  pool: PublicKey | Pda;
  custody: PublicKey | Pda;
  custodyDovesPriceAccount: PublicKey | Pda;
  custodyPythnetPriceAccount: PublicKey | Pda;
  lpTokenMint: PublicKey | Pda;
};

// Data.
export type GetRemoveLiquidityAmountAndFee2InstructionData = {
  discriminator: Uint8Array;
  params: GetRemoveLiquidityAmountAndFee2Params;
};

export type GetRemoveLiquidityAmountAndFee2InstructionDataArgs = {
  params: GetRemoveLiquidityAmountAndFee2ParamsArgs;
};

export function getGetRemoveLiquidityAmountAndFee2InstructionDataSerializer(): Serializer<
  GetRemoveLiquidityAmountAndFee2InstructionDataArgs,
  GetRemoveLiquidityAmountAndFee2InstructionData
> {
  return mapSerializer<
    GetRemoveLiquidityAmountAndFee2InstructionDataArgs,
    any,
    GetRemoveLiquidityAmountAndFee2InstructionData
  >(
    struct<GetRemoveLiquidityAmountAndFee2InstructionData>(
      [
        ['discriminator', bytes({ size: 8 })],
        ['params', getGetRemoveLiquidityAmountAndFee2ParamsSerializer()],
      ],
      { description: 'GetRemoveLiquidityAmountAndFee2InstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([183, 59, 72, 110, 223, 243, 150, 142]),
    })
  ) as Serializer<
    GetRemoveLiquidityAmountAndFee2InstructionDataArgs,
    GetRemoveLiquidityAmountAndFee2InstructionData
  >;
}

// Args.
export type GetRemoveLiquidityAmountAndFee2InstructionArgs =
  GetRemoveLiquidityAmountAndFee2InstructionDataArgs;

// Instruction.
export function getRemoveLiquidityAmountAndFee2(
  context: Pick<Context, 'programs'>,
  input: GetRemoveLiquidityAmountAndFee2InstructionAccounts &
    GetRemoveLiquidityAmountAndFee2InstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey('perpetuals', '');

  // Accounts.
  const resolvedAccounts = {
    perpetuals: {
      index: 0,
      isWritable: false as boolean,
      value: input.perpetuals ?? null,
    },
    pool: { index: 1, isWritable: false as boolean, value: input.pool ?? null },
    custody: {
      index: 2,
      isWritable: false as boolean,
      value: input.custody ?? null,
    },
    custodyDovesPriceAccount: {
      index: 3,
      isWritable: false as boolean,
      value: input.custodyDovesPriceAccount ?? null,
    },
    custodyPythnetPriceAccount: {
      index: 4,
      isWritable: false as boolean,
      value: input.custodyPythnetPriceAccount ?? null,
    },
    lpTokenMint: {
      index: 5,
      isWritable: false as boolean,
      value: input.lpTokenMint ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: GetRemoveLiquidityAmountAndFee2InstructionArgs = {
    ...input,
  };

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
  const data =
    getGetRemoveLiquidityAmountAndFee2InstructionDataSerializer().serialize(
      resolvedArgs as GetRemoveLiquidityAmountAndFee2InstructionDataArgs
    );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
