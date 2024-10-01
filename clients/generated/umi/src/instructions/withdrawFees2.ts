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
  WithdrawFees2Params,
  WithdrawFees2ParamsArgs,
  getWithdrawFees2ParamsSerializer,
} from '../types';

// Accounts.
export type WithdrawFees2InstructionAccounts = {
  keeper: Signer;
  transferAuthority: PublicKey | Pda;
  perpetuals: PublicKey | Pda;
  pool: PublicKey | Pda;
  custody: PublicKey | Pda;
  custodyTokenAccount: PublicKey | Pda;
  custodyDovesPriceAccount: PublicKey | Pda;
  custodyPythnetPriceAccount: PublicKey | Pda;
  receivingTokenAccount: PublicKey | Pda;
  tokenProgram: PublicKey | Pda;
};

// Data.
export type WithdrawFees2InstructionData = {
  discriminator: Uint8Array;
  params: WithdrawFees2Params;
};

export type WithdrawFees2InstructionDataArgs = {
  params: WithdrawFees2ParamsArgs;
};

export function getWithdrawFees2InstructionDataSerializer(): Serializer<
  WithdrawFees2InstructionDataArgs,
  WithdrawFees2InstructionData
> {
  return mapSerializer<
    WithdrawFees2InstructionDataArgs,
    any,
    WithdrawFees2InstructionData
  >(
    struct<WithdrawFees2InstructionData>(
      [
        ['discriminator', bytes({ size: 8 })],
        ['params', getWithdrawFees2ParamsSerializer()],
      ],
      { description: 'WithdrawFees2InstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([252, 128, 143, 145, 225, 221, 159, 207]),
    })
  ) as Serializer<
    WithdrawFees2InstructionDataArgs,
    WithdrawFees2InstructionData
  >;
}

// Args.
export type WithdrawFees2InstructionArgs = WithdrawFees2InstructionDataArgs;

// Instruction.
export function withdrawFees2(
  context: Pick<Context, 'programs'>,
  input: WithdrawFees2InstructionAccounts & WithdrawFees2InstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey('perpetuals', '');

  // Accounts.
  const resolvedAccounts = {
    keeper: {
      index: 0,
      isWritable: false as boolean,
      value: input.keeper ?? null,
    },
    transferAuthority: {
      index: 1,
      isWritable: false as boolean,
      value: input.transferAuthority ?? null,
    },
    perpetuals: {
      index: 2,
      isWritable: false as boolean,
      value: input.perpetuals ?? null,
    },
    pool: { index: 3, isWritable: true as boolean, value: input.pool ?? null },
    custody: {
      index: 4,
      isWritable: true as boolean,
      value: input.custody ?? null,
    },
    custodyTokenAccount: {
      index: 5,
      isWritable: true as boolean,
      value: input.custodyTokenAccount ?? null,
    },
    custodyDovesPriceAccount: {
      index: 6,
      isWritable: false as boolean,
      value: input.custodyDovesPriceAccount ?? null,
    },
    custodyPythnetPriceAccount: {
      index: 7,
      isWritable: false as boolean,
      value: input.custodyPythnetPriceAccount ?? null,
    },
    receivingTokenAccount: {
      index: 8,
      isWritable: true as boolean,
      value: input.receivingTokenAccount ?? null,
    },
    tokenProgram: {
      index: 9,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: WithdrawFees2InstructionArgs = { ...input };

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
  const data = getWithdrawFees2InstructionDataSerializer().serialize(
    resolvedArgs as WithdrawFees2InstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
