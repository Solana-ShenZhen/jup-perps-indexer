//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use crate::generated::types::AddPoolParams;

/// Accounts.
pub struct AddPool {
      
              
          pub admin: solana_program::pubkey::Pubkey,
          
              
          pub transfer_authority: solana_program::pubkey::Pubkey,
          
              
          pub perpetuals: solana_program::pubkey::Pubkey,
          
              
          pub pool: solana_program::pubkey::Pubkey,
          
              
          pub lp_token_mint: solana_program::pubkey::Pubkey,
          
              
          pub system_program: solana_program::pubkey::Pubkey,
          
              
          pub token_program: solana_program::pubkey::Pubkey,
          
              
          pub rent: solana_program::pubkey::Pubkey,
      }

impl AddPool {
  pub fn instruction(&self, args: AddPoolInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: AddPoolInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.transfer_authority,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.perpetuals,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.lp_token_mint,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = AddPoolInstructionData::new().try_to_vec().unwrap();
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
pub struct AddPoolInstructionData {
            discriminator: [u8; 8],
            }

impl AddPoolInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [115, 230, 212, 211, 175, 49, 39, 169],
                                }
  }
}

impl Default for AddPoolInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddPoolInstructionArgs {
                  pub params: AddPoolParams,
      }


/// Instruction builder for `AddPool`.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` admin
          ///   1. `[]` transfer_authority
                ///   2. `[writable]` perpetuals
                ///   3. `[writable]` pool
                ///   4. `[writable]` lp_token_mint
          ///   5. `[]` system_program
          ///   6. `[]` token_program
          ///   7. `[]` rent
#[derive(Clone, Debug, Default)]
pub struct AddPoolBuilder {
            admin: Option<solana_program::pubkey::Pubkey>,
                transfer_authority: Option<solana_program::pubkey::Pubkey>,
                perpetuals: Option<solana_program::pubkey::Pubkey>,
                pool: Option<solana_program::pubkey::Pubkey>,
                lp_token_mint: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                token_program: Option<solana_program::pubkey::Pubkey>,
                rent: Option<solana_program::pubkey::Pubkey>,
                        params: Option<AddPoolParams>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl AddPoolBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.admin = Some(admin);
                    self
    }
            #[inline(always)]
    pub fn transfer_authority(&mut self, transfer_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.transfer_authority = Some(transfer_authority);
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
    pub fn lp_token_mint(&mut self, lp_token_mint: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.lp_token_mint = Some(lp_token_mint);
                    self
    }
            #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
                    self
    }
            #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_program = Some(token_program);
                    self
    }
            #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.rent = Some(rent);
                    self
    }
                    #[inline(always)]
      pub fn params(&mut self, params: AddPoolParams) -> &mut Self {
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
    let accounts = AddPool {
                              admin: self.admin.expect("admin is not set"),
                                        transfer_authority: self.transfer_authority.expect("transfer_authority is not set"),
                                        perpetuals: self.perpetuals.expect("perpetuals is not set"),
                                        pool: self.pool.expect("pool is not set"),
                                        lp_token_mint: self.lp_token_mint.expect("lp_token_mint is not set"),
                                        system_program: self.system_program.expect("system_program is not set"),
                                        token_program: self.token_program.expect("token_program is not set"),
                                        rent: self.rent.expect("rent is not set"),
                      };
          let args = AddPoolInstructionArgs {
                                                              params: self.params.clone().expect("params is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `add_pool` CPI accounts.
  pub struct AddPoolCpiAccounts<'a, 'b> {
          
                    
              pub admin: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub pool: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub rent: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `add_pool` CPI instruction.
pub struct AddPoolCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub admin: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub pool: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub rent: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: AddPoolInstructionArgs,
  }

impl<'a, 'b> AddPoolCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: AddPoolCpiAccounts<'a, 'b>,
              args: AddPoolInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              admin: accounts.admin,
              transfer_authority: accounts.transfer_authority,
              perpetuals: accounts.perpetuals,
              pool: accounts.pool,
              lp_token_mint: accounts.lp_token_mint,
              system_program: accounts.system_program,
              token_program: accounts.token_program,
              rent: accounts.rent,
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
    let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.transfer_authority.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.perpetuals.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lp_token_mint.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = AddPoolInstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::PERPETUALS_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(8 + 1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.admin.clone());
                        account_infos.push(self.transfer_authority.clone());
                        account_infos.push(self.perpetuals.clone());
                        account_infos.push(self.pool.clone());
                        account_infos.push(self.lp_token_mint.clone());
                        account_infos.push(self.system_program.clone());
                        account_infos.push(self.token_program.clone());
                        account_infos.push(self.rent.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `AddPool` via CPI.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` admin
          ///   1. `[]` transfer_authority
                ///   2. `[writable]` perpetuals
                ///   3. `[writable]` pool
                ///   4. `[writable]` lp_token_mint
          ///   5. `[]` system_program
          ///   6. `[]` token_program
          ///   7. `[]` rent
#[derive(Clone, Debug)]
pub struct AddPoolCpiBuilder<'a, 'b> {
  instruction: Box<AddPoolCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> AddPoolCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(AddPoolCpiBuilderInstruction {
      __program: program,
              admin: None,
              transfer_authority: None,
              perpetuals: None,
              pool: None,
              lp_token_mint: None,
              system_program: None,
              token_program: None,
              rent: None,
                                            params: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn admin(&mut self, admin: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.admin = Some(admin);
                    self
    }
      #[inline(always)]
    pub fn transfer_authority(&mut self, transfer_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.transfer_authority = Some(transfer_authority);
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
    pub fn lp_token_mint(&mut self, lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.lp_token_mint = Some(lp_token_mint);
                    self
    }
      #[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
                    self
    }
      #[inline(always)]
    pub fn token_program(&mut self, token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_program = Some(token_program);
                    self
    }
      #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.rent = Some(rent);
                    self
    }
                    #[inline(always)]
      pub fn params(&mut self, params: AddPoolParams) -> &mut Self {
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
          let args = AddPoolInstructionArgs {
                                                              params: self.instruction.params.clone().expect("params is not set"),
                                    };
        let instruction = AddPoolCpi {
        __program: self.instruction.__program,
                  
          admin: self.instruction.admin.expect("admin is not set"),
                  
          transfer_authority: self.instruction.transfer_authority.expect("transfer_authority is not set"),
                  
          perpetuals: self.instruction.perpetuals.expect("perpetuals is not set"),
                  
          pool: self.instruction.pool.expect("pool is not set"),
                  
          lp_token_mint: self.instruction.lp_token_mint.expect("lp_token_mint is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                  
          token_program: self.instruction.token_program.expect("token_program is not set"),
                  
          rent: self.instruction.rent.expect("rent is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct AddPoolCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                transfer_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                perpetuals: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                lp_token_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        params: Option<AddPoolParams>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

