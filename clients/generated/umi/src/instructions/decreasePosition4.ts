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
  DecreasePosition4Params,
  DecreasePosition4ParamsArgs,
  getDecreasePosition4ParamsSerializer,
} from '../types';

// Accounts.
export type DecreasePosition4InstructionAccounts = {
  keeper: Signer;
  owner: PublicKey | Pda;
  transferAuthority: PublicKey | Pda;
  perpetuals: PublicKey | Pda;
  pool: PublicKey | Pda;
  positionRequest: PublicKey | Pda;
  positionRequestAta: PublicKey | Pda;
  position: PublicKey | Pda;
  custody: PublicKey | Pda;
  custodyDovesPriceAccount: PublicKey | Pda;
  custodyPythnetPriceAccount: PublicKey | Pda;
  collateralCustody: PublicKey | Pda;
  collateralCustodyDovesPriceAccount: PublicKey | Pda;
  collateralCustodyPythnetPriceAccount: PublicKey | Pda;
  collateralCustodyTokenAccount: PublicKey | Pda;
  tokenProgram: PublicKey | Pda;
  eventAuthority: PublicKey | Pda;
  program: PublicKey | Pda;
};

// Data.
export type DecreasePosition4InstructionData = {
  discriminator: Uint8Array;
  params: DecreasePosition4Params;
};

export type DecreasePosition4InstructionDataArgs = {
  params: DecreasePosition4ParamsArgs;
};

export function getDecreasePosition4InstructionDataSerializer(): Serializer<
  DecreasePosition4InstructionDataArgs,
  DecreasePosition4InstructionData
> {
  return mapSerializer<
    DecreasePosition4InstructionDataArgs,
    any,
    DecreasePosition4InstructionData
  >(
    struct<DecreasePosition4InstructionData>(
      [
        ['discriminator', bytes({ size: 8 })],
        ['params', getDecreasePosition4ParamsSerializer()],
      ],
      { description: 'DecreasePosition4InstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([185, 161, 114, 175, 96, 148, 3, 170]),
    })
  ) as Serializer<
    DecreasePosition4InstructionDataArgs,
    DecreasePosition4InstructionData
  >;
}

// Args.
export type DecreasePosition4InstructionArgs =
  DecreasePosition4InstructionDataArgs;

// Instruction.
export function decreasePosition4(
  context: Pick<Context, 'programs'>,
  input: DecreasePosition4InstructionAccounts & DecreasePosition4InstructionArgs
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
    owner: {
      index: 1,
      isWritable: true as boolean,
      value: input.owner ?? null,
    },
    transferAuthority: {
      index: 2,
      isWritable: false as boolean,
      value: input.transferAuthority ?? null,
    },
    perpetuals: {
      index: 3,
      isWritable: false as boolean,
      value: input.perpetuals ?? null,
    },
    pool: { index: 4, isWritable: true as boolean, value: input.pool ?? null },
    positionRequest: {
      index: 5,
      isWritable: true as boolean,
      value: input.positionRequest ?? null,
    },
    positionRequestAta: {
      index: 6,
      isWritable: true as boolean,
      value: input.positionRequestAta ?? null,
    },
    position: {
      index: 7,
      isWritable: true as boolean,
      value: input.position ?? null,
    },
    custody: {
      index: 8,
      isWritable: true as boolean,
      value: input.custody ?? null,
    },
    custodyDovesPriceAccount: {
      index: 9,
      isWritable: false as boolean,
      value: input.custodyDovesPriceAccount ?? null,
    },
    custodyPythnetPriceAccount: {
      index: 10,
      isWritable: false as boolean,
      value: input.custodyPythnetPriceAccount ?? null,
    },
    collateralCustody: {
      index: 11,
      isWritable: true as boolean,
      value: input.collateralCustody ?? null,
    },
    collateralCustodyDovesPriceAccount: {
      index: 12,
      isWritable: false as boolean,
      value: input.collateralCustodyDovesPriceAccount ?? null,
    },
    collateralCustodyPythnetPriceAccount: {
      index: 13,
      isWritable: false as boolean,
      value: input.collateralCustodyPythnetPriceAccount ?? null,
    },
    collateralCustodyTokenAccount: {
      index: 14,
      isWritable: true as boolean,
      value: input.collateralCustodyTokenAccount ?? null,
    },
    tokenProgram: {
      index: 15,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    eventAuthority: {
      index: 16,
      isWritable: false as boolean,
      value: input.eventAuthority ?? null,
    },
    program: {
      index: 17,
      isWritable: false as boolean,
      value: input.program ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: DecreasePosition4InstructionArgs = { ...input };

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
  const data = getDecreasePosition4InstructionDataSerializer().serialize(
    resolvedArgs as DecreasePosition4InstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
