/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Option, OptionOrNullable } from '@metaplex-foundation/umi';
import {
  Serializer,
  option,
  struct,
  u64,
} from '@metaplex-foundation/umi/serializers';
import { Side, SideArgs, getSideSerializer } from '.';

export type CreateIncreasePositionMarketRequestParams = {
  sizeUsdDelta: bigint;
  collateralTokenDelta: bigint;
  side: Side;
  priceSlippage: bigint;
  jupiterMinimumOut: Option<bigint>;
  counter: bigint;
};

export type CreateIncreasePositionMarketRequestParamsArgs = {
  sizeUsdDelta: number | bigint;
  collateralTokenDelta: number | bigint;
  side: SideArgs;
  priceSlippage: number | bigint;
  jupiterMinimumOut: OptionOrNullable<number | bigint>;
  counter: number | bigint;
};

export function getCreateIncreasePositionMarketRequestParamsSerializer(): Serializer<
  CreateIncreasePositionMarketRequestParamsArgs,
  CreateIncreasePositionMarketRequestParams
> {
  return struct<CreateIncreasePositionMarketRequestParams>(
    [
      ['sizeUsdDelta', u64()],
      ['collateralTokenDelta', u64()],
      ['side', getSideSerializer()],
      ['priceSlippage', u64()],
      ['jupiterMinimumOut', option(u64())],
      ['counter', u64()],
    ],
    { description: 'CreateIncreasePositionMarketRequestParams' }
  ) as Serializer<
    CreateIncreasePositionMarketRequestParamsArgs,
    CreateIncreasePositionMarketRequestParams
  >;
}
