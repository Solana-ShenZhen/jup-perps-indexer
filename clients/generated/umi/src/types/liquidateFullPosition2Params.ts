/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Serializer, bool, struct } from '@metaplex-foundation/umi/serializers';

export type LiquidateFullPosition2Params = { usePriceUpdate: boolean };

export type LiquidateFullPosition2ParamsArgs = LiquidateFullPosition2Params;

export function getLiquidateFullPosition2ParamsSerializer(): Serializer<
  LiquidateFullPosition2ParamsArgs,
  LiquidateFullPosition2Params
> {
  return struct<LiquidateFullPosition2Params>([['usePriceUpdate', bool()]], {
    description: 'LiquidateFullPosition2Params',
  }) as Serializer<
    LiquidateFullPosition2ParamsArgs,
    LiquidateFullPosition2Params
  >;
}
