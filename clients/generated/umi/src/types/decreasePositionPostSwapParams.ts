/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Serializer, struct } from '@metaplex-foundation/umi/serializers';

export type DecreasePositionPostSwapParams = {};

export type DecreasePositionPostSwapParamsArgs = DecreasePositionPostSwapParams;

export function getDecreasePositionPostSwapParamsSerializer(): Serializer<
  DecreasePositionPostSwapParamsArgs,
  DecreasePositionPostSwapParams
> {
  return struct<DecreasePositionPostSwapParams>([], {
    description: 'DecreasePositionPostSwapParams',
  }) as Serializer<
    DecreasePositionPostSwapParamsArgs,
    DecreasePositionPostSwapParams
  >;
}
