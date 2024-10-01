/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum PriceStaleTolerance {
  Strict,
  Loose,
}

export type PriceStaleToleranceArgs = PriceStaleTolerance;

export function getPriceStaleToleranceSerializer(): Serializer<
  PriceStaleToleranceArgs,
  PriceStaleTolerance
> {
  return scalarEnum<PriceStaleTolerance>(PriceStaleTolerance, {
    description: 'PriceStaleTolerance',
  }) as Serializer<PriceStaleToleranceArgs, PriceStaleTolerance>;
}
