/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  Serializer,
  i32,
  i64,
  struct,
  u64,
} from '@metaplex-foundation/umi/serializers';

export type SetTestOraclePriceParams = {
  price: bigint;
  expo: number;
  conf: bigint;
  publishTime: bigint;
};

export type SetTestOraclePriceParamsArgs = {
  price: number | bigint;
  expo: number;
  conf: number | bigint;
  publishTime: number | bigint;
};

export function getSetTestOraclePriceParamsSerializer(): Serializer<
  SetTestOraclePriceParamsArgs,
  SetTestOraclePriceParams
> {
  return struct<SetTestOraclePriceParams>(
    [
      ['price', u64()],
      ['expo', i32()],
      ['conf', u64()],
      ['publishTime', i64()],
    ],
    { description: 'SetTestOraclePriceParams' }
  ) as Serializer<SetTestOraclePriceParamsArgs, SetTestOraclePriceParams>;
}
