/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Serializer, struct } from '@metaplex-foundation/umi/serializers';

export type GetLiquidationStateParams = {};

export type GetLiquidationStateParamsArgs = GetLiquidationStateParams;

export function getGetLiquidationStateParamsSerializer(): Serializer<
  GetLiquidationStateParamsArgs,
  GetLiquidationStateParams
> {
  return struct<GetLiquidationStateParams>([], {
    description: 'GetLiquidationStateParams',
  }) as Serializer<GetLiquidationStateParamsArgs, GetLiquidationStateParams>;
}
