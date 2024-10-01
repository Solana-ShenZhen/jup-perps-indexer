/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Serializer, struct, u64 } from '@metaplex-foundation/umi/serializers';

export type Swap2Params = { amountIn: bigint; minAmountOut: bigint };

export type Swap2ParamsArgs = {
  amountIn: number | bigint;
  minAmountOut: number | bigint;
};

export function getSwap2ParamsSerializer(): Serializer<
  Swap2ParamsArgs,
  Swap2Params
> {
  return struct<Swap2Params>(
    [
      ['amountIn', u64()],
      ['minAmountOut', u64()],
    ],
    { description: 'Swap2Params' }
  ) as Serializer<Swap2ParamsArgs, Swap2Params>;
}
