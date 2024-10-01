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
} from '@metaplex-foundation/umi/serializers';
import {
  PriceCalcMode,
  PriceCalcModeArgs,
  getPriceCalcModeSerializer,
} from '.';

export type GetAssetsUnderManagement2Params = { mode: Option<PriceCalcMode> };

export type GetAssetsUnderManagement2ParamsArgs = {
  mode: OptionOrNullable<PriceCalcModeArgs>;
};

export function getGetAssetsUnderManagement2ParamsSerializer(): Serializer<
  GetAssetsUnderManagement2ParamsArgs,
  GetAssetsUnderManagement2Params
> {
  return struct<GetAssetsUnderManagement2Params>(
    [['mode', option(getPriceCalcModeSerializer())]],
    { description: 'GetAssetsUnderManagement2Params' }
  ) as Serializer<
    GetAssetsUnderManagement2ParamsArgs,
    GetAssetsUnderManagement2Params
  >;
}
