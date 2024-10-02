//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::jup_perps::types::Permissions;
use solana_program::pubkey::Pubkey;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Perpetuals {
pub discriminator: [u8; 8],
pub permissions: Permissions,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<Vec<serde_with::DisplayFromStr>>"))]
pub pools: Vec<Pubkey>,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub admin: Pubkey,
pub transfer_authority_bump: u8,
pub perpetuals_bump: u8,
pub inception_time: i64,
}


impl Perpetuals {
  #[inline(always)]
  pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
    let mut data = data;
    Self::deserialize(&mut data)
  }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for Perpetuals {
  type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for Perpetuals {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for Perpetuals {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for Perpetuals {
    fn owner() -> Pubkey {
        crate::PERPETUALS_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for Perpetuals {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for Perpetuals {
  const DISCRIMINATOR: [u8; 8] = [0; 8];
}
