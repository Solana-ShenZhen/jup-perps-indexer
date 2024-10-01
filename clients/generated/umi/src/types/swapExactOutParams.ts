/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Serializer, struct, u64 } from '@metaplex-foundation/umi/serializers';

export type SwapExactOutParams = { amountOut: bigint; maxAmountIn: bigint };

export type SwapExactOutParamsArgs = {
  amountOut: number | bigint;
  maxAmountIn: number | bigint;
};

export function getSwapExactOutParamsSerializer(): Serializer<
  SwapExactOutParamsArgs,
  SwapExactOutParams
> {
  return struct<SwapExactOutParams>(
    [
      ['amountOut', u64()],
      ['maxAmountIn', u64()],
    ],
    { description: 'SwapExactOutParams' }
  ) as Serializer<SwapExactOutParamsArgs, SwapExactOutParams>;
}
