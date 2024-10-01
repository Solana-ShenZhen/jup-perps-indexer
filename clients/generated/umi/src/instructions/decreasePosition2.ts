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
  DecreasePosition2Params,
  DecreasePosition2ParamsArgs,
  getDecreasePosition2ParamsSerializer,
} from '../types';

// Accounts.
export type DecreasePosition2InstructionAccounts = {
  keeper: Signer;
  keeperAta?: PublicKey | Pda;
  owner: PublicKey | Pda;
  transferAuthority: PublicKey | Pda;
  perpetuals: PublicKey | Pda;
  pool: PublicKey | Pda;
  positionRequest: PublicKey | Pda;
  positionRequestAta: PublicKey | Pda;
  position: PublicKey | Pda;
  custody: PublicKey | Pda;
  custodyOracleAccount: PublicKey | Pda;
  collateralCustody: PublicKey | Pda;
  collateralCustodyOracleAccount: PublicKey | Pda;
  collateralCustodyTokenAccount: PublicKey | Pda;
  instruction: PublicKey | Pda;
  tokenProgram: PublicKey | Pda;
  custodyPriceUpdate?: PublicKey | Pda;
  collateralCustodyPriceUpdate?: PublicKey | Pda;
  eventAuthority: PublicKey | Pda;
  program: PublicKey | Pda;
};

// Data.
export type DecreasePosition2InstructionData = {
  discriminator: Uint8Array;
  params: DecreasePosition2Params;
};

export type DecreasePosition2InstructionDataArgs = {
  params: DecreasePosition2ParamsArgs;
};

export function getDecreasePosition2InstructionDataSerializer(): Serializer<
  DecreasePosition2InstructionDataArgs,
  DecreasePosition2InstructionData
> {
  return mapSerializer<
    DecreasePosition2InstructionDataArgs,
    any,
    DecreasePosition2InstructionData
  >(
    struct<DecreasePosition2InstructionData>(
      [
        ['discriminator', bytes({ size: 8 })],
        ['params', getDecreasePosition2ParamsSerializer()],
      ],
      { description: 'DecreasePosition2InstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([180, 193, 163, 222, 169, 231, 66, 253]),
    })
  ) as Serializer<
    DecreasePosition2InstructionDataArgs,
    DecreasePosition2InstructionData
  >;
}

// Args.
export type DecreasePosition2InstructionArgs =
  DecreasePosition2InstructionDataArgs;

// Instruction.
export function decreasePosition2(
  context: Pick<Context, 'programs'>,
  input: DecreasePosition2InstructionAccounts & DecreasePosition2InstructionArgs
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
    keeperAta: {
      index: 1,
      isWritable: true as boolean,
      value: input.keeperAta ?? null,
    },
    owner: {
      index: 2,
      isWritable: true as boolean,
      value: input.owner ?? null,
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
    positionRequest: {
      index: 6,
      isWritable: true as boolean,
      value: input.positionRequest ?? null,
    },
    positionRequestAta: {
      index: 7,
      isWritable: true as boolean,
      value: input.positionRequestAta ?? null,
    },
    position: {
      index: 8,
      isWritable: true as boolean,
      value: input.position ?? null,
    },
    custody: {
      index: 9,
      isWritable: true as boolean,
      value: input.custody ?? null,
    },
    custodyOracleAccount: {
      index: 10,
      isWritable: false as boolean,
      value: input.custodyOracleAccount ?? null,
    },
    collateralCustody: {
      index: 11,
      isWritable: true as boolean,
      value: input.collateralCustody ?? null,
    },
    collateralCustodyOracleAccount: {
      index: 12,
      isWritable: false as boolean,
      value: input.collateralCustodyOracleAccount ?? null,
    },
    collateralCustodyTokenAccount: {
      index: 13,
      isWritable: true as boolean,
      value: input.collateralCustodyTokenAccount ?? null,
    },
    instruction: {
      index: 14,
      isWritable: false as boolean,
      value: input.instruction ?? null,
    },
    tokenProgram: {
      index: 15,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    custodyPriceUpdate: {
      index: 16,
      isWritable: false as boolean,
      value: input.custodyPriceUpdate ?? null,
    },
    collateralCustodyPriceUpdate: {
      index: 17,
      isWritable: false as boolean,
      value: input.collateralCustodyPriceUpdate ?? null,
    },
    eventAuthority: {
      index: 18,
      isWritable: false as boolean,
      value: input.eventAuthority ?? null,
    },
    program: {
      index: 19,
      isWritable: false as boolean,
      value: input.program ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: DecreasePosition2InstructionArgs = { ...input };

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
  const data = getDecreasePosition2InstructionDataSerializer().serialize(
    resolvedArgs as DecreasePosition2InstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
