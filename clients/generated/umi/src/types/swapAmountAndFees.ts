/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Serializer, struct, u64 } from '@metaplex-foundation/umi/serializers';

export type SwapAmountAndFees = {
  amountIn: bigint;
  amountOut: bigint;
  feeBps: bigint;
  feeToken: bigint;
};

export type SwapAmountAndFeesArgs = {
  amountIn: number | bigint;
  amountOut: number | bigint;
  feeBps: number | bigint;
  feeToken: number | bigint;
};

export function getSwapAmountAndFeesSerializer(): Serializer<
  SwapAmountAndFeesArgs,
  SwapAmountAndFees
> {
  return struct<SwapAmountAndFees>(
    [
      ['amountIn', u64()],
      ['amountOut', u64()],
      ['feeBps', u64()],
      ['feeToken', u64()],
    ],
    { description: 'SwapAmountAndFees' }
  ) as Serializer<SwapAmountAndFeesArgs, SwapAmountAndFees>;
}
