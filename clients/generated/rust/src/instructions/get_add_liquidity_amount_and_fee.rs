//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use crate::generated::types::GetAddLiquidityAmountAndFeeParams;

/// Accounts.
pub struct GetAddLiquidityAmountAndFee {
      
              
          pub perpetuals: solana_program::pubkey::Pubkey,
          
              
          pub pool: solana_program::pubkey::Pubkey,
          
              
          pub custody: solana_program::pubkey::Pubkey,
          
              
          pub custody_oracle_account: solana_program::pubkey::Pubkey,
          
              
          pub lp_token_mint: solana_program::pubkey::Pubkey,
      }

impl GetAddLiquidityAmountAndFee {
  pub fn instruction(&self, args: GetAddLiquidityAmountAndFeeInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: GetAddLiquidityAmountAndFeeInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.perpetuals,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.pool,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.custody,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.custody_oracle_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.lp_token_mint,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = GetAddLiquidityAmountAndFeeInstructionData::new().try_to_vec().unwrap();
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
pub struct GetAddLiquidityAmountAndFeeInstructionData {
            discriminator: [u8; 8],
            }

impl GetAddLiquidityAmountAndFeeInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [172, 150, 249, 181, 233, 241, 78, 139],
                                }
  }
}

impl Default for GetAddLiquidityAmountAndFeeInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetAddLiquidityAmountAndFeeInstructionArgs {
                  pub params: GetAddLiquidityAmountAndFeeParams,
      }


/// Instruction builder for `GetAddLiquidityAmountAndFee`.
///
/// ### Accounts:
///
          ///   0. `[]` perpetuals
          ///   1. `[]` pool
          ///   2. `[]` custody
          ///   3. `[]` custody_oracle_account
          ///   4. `[]` lp_token_mint
#[derive(Clone, Debug, Default)]
pub struct GetAddLiquidityAmountAndFeeBuilder {
            perpetuals: Option<solana_program::pubkey::Pubkey>,
                pool: Option<solana_program::pubkey::Pubkey>,
                custody: Option<solana_program::pubkey::Pubkey>,
                custody_oracle_account: Option<solana_program::pubkey::Pubkey>,
                lp_token_mint: Option<solana_program::pubkey::Pubkey>,
                        params: Option<GetAddLiquidityAmountAndFeeParams>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl GetAddLiquidityAmountAndFeeBuilder {
  pub fn new() -> Self {
    Self::default()
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
    pub fn lp_token_mint(&mut self, lp_token_mint: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.lp_token_mint = Some(lp_token_mint);
                    self
    }
                    #[inline(always)]
      pub fn params(&mut self, params: GetAddLiquidityAmountAndFeeParams) -> &mut Self {
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
    let accounts = GetAddLiquidityAmountAndFee {
                              perpetuals: self.perpetuals.expect("perpetuals is not set"),
                                        pool: self.pool.expect("pool is not set"),
                                        custody: self.custody.expect("custody is not set"),
                                        custody_oracle_account: self.custody_oracle_account.expect("custody_oracle_account is not set"),
                                        lp_token_mint: self.lp_token_mint.expect("lp_token_mint is not set"),
                      };
          let args = GetAddLiquidityAmountAndFeeInstructionArgs {
                                                              params: self.params.clone().expect("params is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `get_add_liquidity_amount_and_fee` CPI accounts.
  pub struct GetAddLiquidityAmountAndFeeCpiAccounts<'a, 'b> {
          
                    
              pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub pool: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub custody: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `get_add_liquidity_amount_and_fee` CPI instruction.
pub struct GetAddLiquidityAmountAndFeeCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub pool: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub custody: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: GetAddLiquidityAmountAndFeeInstructionArgs,
  }

impl<'a, 'b> GetAddLiquidityAmountAndFeeCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: GetAddLiquidityAmountAndFeeCpiAccounts<'a, 'b>,
              args: GetAddLiquidityAmountAndFeeInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              perpetuals: accounts.perpetuals,
              pool: accounts.pool,
              custody: accounts.custody,
              custody_oracle_account: accounts.custody_oracle_account,
              lp_token_mint: accounts.lp_token_mint,
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
    let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.perpetuals.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.pool.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.custody.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.custody_oracle_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.lp_token_mint.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = GetAddLiquidityAmountAndFeeInstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::PERPETUALS_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(5 + 1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.perpetuals.clone());
                        account_infos.push(self.pool.clone());
                        account_infos.push(self.custody.clone());
                        account_infos.push(self.custody_oracle_account.clone());
                        account_infos.push(self.lp_token_mint.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `GetAddLiquidityAmountAndFee` via CPI.
///
/// ### Accounts:
///
          ///   0. `[]` perpetuals
          ///   1. `[]` pool
          ///   2. `[]` custody
          ///   3. `[]` custody_oracle_account
          ///   4. `[]` lp_token_mint
#[derive(Clone, Debug)]
pub struct GetAddLiquidityAmountAndFeeCpiBuilder<'a, 'b> {
  instruction: Box<GetAddLiquidityAmountAndFeeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> GetAddLiquidityAmountAndFeeCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(GetAddLiquidityAmountAndFeeCpiBuilderInstruction {
      __program: program,
              perpetuals: None,
              pool: None,
              custody: None,
              custody_oracle_account: None,
              lp_token_mint: None,
                                            params: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
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
    pub fn lp_token_mint(&mut self, lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.lp_token_mint = Some(lp_token_mint);
                    self
    }
                    #[inline(always)]
      pub fn params(&mut self, params: GetAddLiquidityAmountAndFeeParams) -> &mut Self {
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
          let args = GetAddLiquidityAmountAndFeeInstructionArgs {
                                                              params: self.instruction.params.clone().expect("params is not set"),
                                    };
        let instruction = GetAddLiquidityAmountAndFeeCpi {
        __program: self.instruction.__program,
                  
          perpetuals: self.instruction.perpetuals.expect("perpetuals is not set"),
                  
          pool: self.instruction.pool.expect("pool is not set"),
                  
          custody: self.instruction.custody.expect("custody is not set"),
                  
          custody_oracle_account: self.instruction.custody_oracle_account.expect("custody_oracle_account is not set"),
                  
          lp_token_mint: self.instruction.lp_token_mint.expect("lp_token_mint is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct GetAddLiquidityAmountAndFeeCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            perpetuals: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                custody: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                custody_oracle_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                lp_token_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        params: Option<GetAddLiquidityAmountAndFeeParams>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

