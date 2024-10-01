//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use crate::generated::types::LiquidateFullPosition2Params;

/// Accounts.
pub struct LiquidateFullPosition2 {
      
              
          pub signer: solana_program::pubkey::Pubkey,
          
              
          pub perpetuals: solana_program::pubkey::Pubkey,
          
              
          pub pool: solana_program::pubkey::Pubkey,
          
              
          pub position: solana_program::pubkey::Pubkey,
          
              
          pub custody: solana_program::pubkey::Pubkey,
          
              
          pub custody_oracle_account: solana_program::pubkey::Pubkey,
          
              
          pub collateral_custody: solana_program::pubkey::Pubkey,
          
              
          pub collateral_custody_oracle_account: solana_program::pubkey::Pubkey,
          
              
          pub collateral_custody_token_account: solana_program::pubkey::Pubkey,
          
              
          pub custody_price_update: Option<solana_program::pubkey::Pubkey>,
          
              
          pub collateral_custody_price_update: Option<solana_program::pubkey::Pubkey>,
          
              
          pub event_authority: solana_program::pubkey::Pubkey,
          
              
          pub program: solana_program::pubkey::Pubkey,
      }

impl LiquidateFullPosition2 {
  pub fn instruction(&self, args: LiquidateFullPosition2InstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: LiquidateFullPosition2InstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.perpetuals,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.position,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.custody,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.custody_oracle_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.collateral_custody,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collateral_custody_oracle_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.collateral_custody_token_account,
            false
          ));
                                                      if let Some(custody_price_update) = self.custody_price_update {
              accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                custody_price_update,
                false,
              ));
            } else {
              accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::PERPETUALS_ID,
                false,
              ));
            }
                                                                if let Some(collateral_custody_price_update) = self.collateral_custody_price_update {
              accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                collateral_custody_price_update,
                false,
              ));
            } else {
              accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::PERPETUALS_ID,
                false,
              ));
            }
                                                    accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = LiquidateFullPosition2InstructionData::new().try_to_vec().unwrap();
          let mut args = args.try_to_vec().unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::PERPETUALS_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct LiquidateFullPosition2InstructionData {
            discriminator: [u8; 8],
            }

impl LiquidateFullPosition2InstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [233, 160, 187, 98, 2, 234, 48, 249],
                                }
  }
}

impl Default for LiquidateFullPosition2InstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidateFullPosition2InstructionArgs {
                  pub params: LiquidateFullPosition2Params,
      }


/// Instruction builder for `LiquidateFullPosition2`.
///
/// ### Accounts:
///
                ///   0. `[signer]` signer
          ///   1. `[]` perpetuals
                ///   2. `[writable]` pool
                ///   3. `[writable]` position
                ///   4. `[writable]` custody
          ///   5. `[]` custody_oracle_account
                ///   6. `[writable]` collateral_custody
          ///   7. `[]` collateral_custody_oracle_account
                ///   8. `[writable]` collateral_custody_token_account
                ///   9. `[optional]` custody_price_update
                ///   10. `[optional]` collateral_custody_price_update
          ///   11. `[]` event_authority
          ///   12. `[]` program
#[derive(Clone, Debug, Default)]
pub struct LiquidateFullPosition2Builder {
            signer: Option<solana_program::pubkey::Pubkey>,
                perpetuals: Option<solana_program::pubkey::Pubkey>,
                pool: Option<solana_program::pubkey::Pubkey>,
                position: Option<solana_program::pubkey::Pubkey>,
                custody: Option<solana_program::pubkey::Pubkey>,
                custody_oracle_account: Option<solana_program::pubkey::Pubkey>,
                collateral_custody: Option<solana_program::pubkey::Pubkey>,
                collateral_custody_oracle_account: Option<solana_program::pubkey::Pubkey>,
                collateral_custody_token_account: Option<solana_program::pubkey::Pubkey>,
                custody_price_update: Option<solana_program::pubkey::Pubkey>,
                collateral_custody_price_update: Option<solana_program::pubkey::Pubkey>,
                event_authority: Option<solana_program::pubkey::Pubkey>,
                program: Option<solana_program::pubkey::Pubkey>,
                        params: Option<LiquidateFullPosition2Params>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl LiquidateFullPosition2Builder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.signer = Some(signer);
                    self
    }
            #[inline(always)]
    pub fn perpetuals(&mut self, perpetuals: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.perpetuals = Some(perpetuals);
                    self
    }
            #[inline(always)]
    pub fn pool(&mut self, pool: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.pool = Some(pool);
                    self
    }
            #[inline(always)]
    pub fn position(&mut self, position: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.position = Some(position);
                    self
    }
            #[inline(always)]
    pub fn custody(&mut self, custody: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.custody = Some(custody);
                    self
    }
            #[inline(always)]
    pub fn custody_oracle_account(&mut self, custody_oracle_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.custody_oracle_account = Some(custody_oracle_account);
                    self
    }
            #[inline(always)]
    pub fn collateral_custody(&mut self, collateral_custody: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.collateral_custody = Some(collateral_custody);
                    self
    }
            #[inline(always)]
    pub fn collateral_custody_oracle_account(&mut self, collateral_custody_oracle_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.collateral_custody_oracle_account = Some(collateral_custody_oracle_account);
                    self
    }
            #[inline(always)]
    pub fn collateral_custody_token_account(&mut self, collateral_custody_token_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.collateral_custody_token_account = Some(collateral_custody_token_account);
                    self
    }
            /// `[optional account]`
#[inline(always)]
    pub fn custody_price_update(&mut self, custody_price_update: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
                        self.custody_price_update = custody_price_update;
                    self
    }
            /// `[optional account]`
#[inline(always)]
    pub fn collateral_custody_price_update(&mut self, collateral_custody_price_update: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
                        self.collateral_custody_price_update = collateral_custody_price_update;
                    self
    }
            #[inline(always)]
    pub fn event_authority(&mut self, event_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.event_authority = Some(event_authority);
                    self
    }
            #[inline(always)]
    pub fn program(&mut self, program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.program = Some(program);
                    self
    }
                    #[inline(always)]
      pub fn params(&mut self, params: LiquidateFullPosition2Params) -> &mut Self {
        self.params = Some(params);
        self
      }
        /// Add an aditional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = LiquidateFullPosition2 {
                              signer: self.signer.expect("signer is not set"),
                                        perpetuals: self.perpetuals.expect("perpetuals is not set"),
                                        pool: self.pool.expect("pool is not set"),
                                        position: self.position.expect("position is not set"),
                                        custody: self.custody.expect("custody is not set"),
                                        custody_oracle_account: self.custody_oracle_account.expect("custody_oracle_account is not set"),
                                        collateral_custody: self.collateral_custody.expect("collateral_custody is not set"),
                                        collateral_custody_oracle_account: self.collateral_custody_oracle_account.expect("collateral_custody_oracle_account is not set"),
                                        collateral_custody_token_account: self.collateral_custody_token_account.expect("collateral_custody_token_account is not set"),
                                        custody_price_update: self.custody_price_update,
                                        collateral_custody_price_update: self.collateral_custody_price_update,
                                        event_authority: self.event_authority.expect("event_authority is not set"),
                                        program: self.program.expect("program is not set"),
                      };
          let args = LiquidateFullPosition2InstructionArgs {
                                                              params: self.params.clone().expect("params is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `liquidate_full_position2` CPI accounts.
  pub struct LiquidateFullPosition2CpiAccounts<'a, 'b> {
          
                    
              pub signer: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub pool: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub position: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub custody: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub collateral_custody: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub collateral_custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub collateral_custody_token_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub custody_price_update: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                
                    
              pub collateral_custody_price_update: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                
                    
              pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `liquidate_full_position2` CPI instruction.
pub struct LiquidateFullPosition2Cpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub signer: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub pool: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub position: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub custody: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub collateral_custody: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub collateral_custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub collateral_custody_token_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub custody_price_update: Option<&'b solana_program::account_info::AccountInfo<'a>>,
          
              
          pub collateral_custody_price_update: Option<&'b solana_program::account_info::AccountInfo<'a>>,
          
              
          pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub program: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: LiquidateFullPosition2InstructionArgs,
  }

impl<'a, 'b> LiquidateFullPosition2Cpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: LiquidateFullPosition2CpiAccounts<'a, 'b>,
              args: LiquidateFullPosition2InstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              signer: accounts.signer,
              perpetuals: accounts.perpetuals,
              pool: accounts.pool,
              position: accounts.position,
              custody: accounts.custody,
              custody_oracle_account: accounts.custody_oracle_account,
              collateral_custody: accounts.collateral_custody,
              collateral_custody_oracle_account: accounts.collateral_custody_oracle_account,
              collateral_custody_token_account: accounts.collateral_custody_token_account,
              custody_price_update: accounts.custody_price_update,
              collateral_custody_price_update: accounts.collateral_custody_price_update,
              event_authority: accounts.event_authority,
              program: accounts.program,
                    __args: args,
          }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.signer.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.perpetuals.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.custody.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.custody_oracle_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.collateral_custody.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collateral_custody_oracle_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.collateral_custody_token_account.key,
            false
          ));
                                          if let Some(custody_price_update) = self.custody_price_update {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
              *custody_price_update.key,
              false,
            ));
          } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
              crate::PERPETUALS_ID,
              false,
            ));
          }
                                          if let Some(collateral_custody_price_update) = self.collateral_custody_price_update {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
              *collateral_custody_price_update.key,
              false,
            ));
          } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
              crate::PERPETUALS_ID,
              false,
            ));
          }
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = LiquidateFullPosition2InstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::PERPETUALS_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(13 + 1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.signer.clone());
                        account_infos.push(self.perpetuals.clone());
                        account_infos.push(self.pool.clone());
                        account_infos.push(self.position.clone());
                        account_infos.push(self.custody.clone());
                        account_infos.push(self.custody_oracle_account.clone());
                        account_infos.push(self.collateral_custody.clone());
                        account_infos.push(self.collateral_custody_oracle_account.clone());
                        account_infos.push(self.collateral_custody_token_account.clone());
                        if let Some(custody_price_update) = self.custody_price_update {
          account_infos.push(custody_price_update.clone());
        }
                        if let Some(collateral_custody_price_update) = self.collateral_custody_price_update {
          account_infos.push(collateral_custody_price_update.clone());
        }
                        account_infos.push(self.event_authority.clone());
                        account_infos.push(self.program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `LiquidateFullPosition2` via CPI.
///
/// ### Accounts:
///
                ///   0. `[signer]` signer
          ///   1. `[]` perpetuals
                ///   2. `[writable]` pool
                ///   3. `[writable]` position
                ///   4. `[writable]` custody
          ///   5. `[]` custody_oracle_account
                ///   6. `[writable]` collateral_custody
          ///   7. `[]` collateral_custody_oracle_account
                ///   8. `[writable]` collateral_custody_token_account
                ///   9. `[optional]` custody_price_update
                ///   10. `[optional]` collateral_custody_price_update
          ///   11. `[]` event_authority
          ///   12. `[]` program
#[derive(Clone, Debug)]
pub struct LiquidateFullPosition2CpiBuilder<'a, 'b> {
  instruction: Box<LiquidateFullPosition2CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> LiquidateFullPosition2CpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(LiquidateFullPosition2CpiBuilderInstruction {
      __program: program,
              signer: None,
              perpetuals: None,
              pool: None,
              position: None,
              custody: None,
              custody_oracle_account: None,
              collateral_custody: None,
              collateral_custody_oracle_account: None,
              collateral_custody_token_account: None,
              custody_price_update: None,
              collateral_custody_price_update: None,
              event_authority: None,
              program: None,
                                            params: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn signer(&mut self, signer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.signer = Some(signer);
                    self
    }
      #[inline(always)]
    pub fn perpetuals(&mut self, perpetuals: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.perpetuals = Some(perpetuals);
                    self
    }
      #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.pool = Some(pool);
                    self
    }
      #[inline(always)]
    pub fn position(&mut self, position: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.position = Some(position);
                    self
    }
      #[inline(always)]
    pub fn custody(&mut self, custody: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.custody = Some(custody);
                    self
    }
      #[inline(always)]
    pub fn custody_oracle_account(&mut self, custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.custody_oracle_account = Some(custody_oracle_account);
                    self
    }
      #[inline(always)]
    pub fn collateral_custody(&mut self, collateral_custody: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.collateral_custody = Some(collateral_custody);
                    self
    }
      #[inline(always)]
    pub fn collateral_custody_oracle_account(&mut self, collateral_custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.collateral_custody_oracle_account = Some(collateral_custody_oracle_account);
                    self
    }
      #[inline(always)]
    pub fn collateral_custody_token_account(&mut self, collateral_custody_token_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.collateral_custody_token_account = Some(collateral_custody_token_account);
                    self
    }
      /// `[optional account]`
#[inline(always)]
    pub fn custody_price_update(&mut self, custody_price_update: Option<&'b solana_program::account_info::AccountInfo<'a>>) -> &mut Self {
                        self.instruction.custody_price_update = custody_price_update;
                    self
    }
      /// `[optional account]`
#[inline(always)]
    pub fn collateral_custody_price_update(&mut self, collateral_custody_price_update: Option<&'b solana_program::account_info::AccountInfo<'a>>) -> &mut Self {
                        self.instruction.collateral_custody_price_update = collateral_custody_price_update;
                    self
    }
      #[inline(always)]
    pub fn event_authority(&mut self, event_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.event_authority = Some(event_authority);
                    self
    }
      #[inline(always)]
    pub fn program(&mut self, program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.program = Some(program);
                    self
    }
                    #[inline(always)]
      pub fn params(&mut self, params: LiquidateFullPosition2Params) -> &mut Self {
        self.instruction.params = Some(params);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
          let args = LiquidateFullPosition2InstructionArgs {
                                                              params: self.instruction.params.clone().expect("params is not set"),
                                    };
        let instruction = LiquidateFullPosition2Cpi {
        __program: self.instruction.__program,
                  
          signer: self.instruction.signer.expect("signer is not set"),
                  
          perpetuals: self.instruction.perpetuals.expect("perpetuals is not set"),
                  
          pool: self.instruction.pool.expect("pool is not set"),
                  
          position: self.instruction.position.expect("position is not set"),
                  
          custody: self.instruction.custody.expect("custody is not set"),
                  
          custody_oracle_account: self.instruction.custody_oracle_account.expect("custody_oracle_account is not set"),
                  
          collateral_custody: self.instruction.collateral_custody.expect("collateral_custody is not set"),
                  
          collateral_custody_oracle_account: self.instruction.collateral_custody_oracle_account.expect("collateral_custody_oracle_account is not set"),
                  
          collateral_custody_token_account: self.instruction.collateral_custody_token_account.expect("collateral_custody_token_account is not set"),
                  
          custody_price_update: self.instruction.custody_price_update,
                  
          collateral_custody_price_update: self.instruction.collateral_custody_price_update,
                  
          event_authority: self.instruction.event_authority.expect("event_authority is not set"),
                  
          program: self.instruction.program.expect("program is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct LiquidateFullPosition2CpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                perpetuals: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                custody: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                custody_oracle_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                collateral_custody: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                collateral_custody_oracle_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                collateral_custody_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                custody_price_update: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                collateral_custody_price_update: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        params: Option<LiquidateFullPosition2Params>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

