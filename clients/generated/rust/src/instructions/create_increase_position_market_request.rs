//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use crate::generated::types::CreateIncreasePositionMarketRequestParams;

/// Accounts.
pub struct CreateIncreasePositionMarketRequest {
      
              
          pub owner: solana_program::pubkey::Pubkey,
          
              
          pub funding_account: solana_program::pubkey::Pubkey,
          
              
          pub perpetuals: solana_program::pubkey::Pubkey,
          
              
          pub pool: solana_program::pubkey::Pubkey,
          
              
          pub position: solana_program::pubkey::Pubkey,
          
              
          pub position_request: solana_program::pubkey::Pubkey,
          
              
          pub position_request_ata: solana_program::pubkey::Pubkey,
          
              
          pub custody: solana_program::pubkey::Pubkey,
          
              
          pub collateral_custody: solana_program::pubkey::Pubkey,
          
              
          pub input_mint: solana_program::pubkey::Pubkey,
          
              
          pub referral: Option<solana_program::pubkey::Pubkey>,
          
              
          pub token_program: solana_program::pubkey::Pubkey,
          
              
          pub associated_token_program: solana_program::pubkey::Pubkey,
          
              
          pub system_program: solana_program::pubkey::Pubkey,
          
              
          pub event_authority: solana_program::pubkey::Pubkey,
          
              
          pub program: solana_program::pubkey::Pubkey,
      }

impl CreateIncreasePositionMarketRequest {
  pub fn instruction(&self, args: CreateIncreasePositionMarketRequestInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: CreateIncreasePositionMarketRequestInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(16 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.owner,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.funding_account,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.perpetuals,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.pool,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.position,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_request,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_request_ata,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.custody,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collateral_custody,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.input_mint,
            false
          ));
                                                      if let Some(referral) = self.referral {
              accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                referral,
                false,
              ));
            } else {
              accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::PERPETUALS_ID,
                false,
              ));
            }
                                                    accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = CreateIncreasePositionMarketRequestInstructionData::new().try_to_vec().unwrap();
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
pub struct CreateIncreasePositionMarketRequestInstructionData {
            discriminator: [u8; 8],
            }

impl CreateIncreasePositionMarketRequestInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [184, 85, 199, 24, 105, 171, 156, 56],
                                }
  }
}

impl Default for CreateIncreasePositionMarketRequestInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateIncreasePositionMarketRequestInstructionArgs {
                  pub params: CreateIncreasePositionMarketRequestParams,
      }


/// Instruction builder for `CreateIncreasePositionMarketRequest`.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` owner
                ///   1. `[writable]` funding_account
          ///   2. `[]` perpetuals
          ///   3. `[]` pool
                ///   4. `[writable]` position
                ///   5. `[writable]` position_request
                ///   6. `[writable]` position_request_ata
          ///   7. `[]` custody
          ///   8. `[]` collateral_custody
          ///   9. `[]` input_mint
                ///   10. `[optional]` referral
          ///   11. `[]` token_program
          ///   12. `[]` associated_token_program
          ///   13. `[]` system_program
          ///   14. `[]` event_authority
          ///   15. `[]` program
#[derive(Clone, Debug, Default)]
pub struct CreateIncreasePositionMarketRequestBuilder {
            owner: Option<solana_program::pubkey::Pubkey>,
                funding_account: Option<solana_program::pubkey::Pubkey>,
                perpetuals: Option<solana_program::pubkey::Pubkey>,
                pool: Option<solana_program::pubkey::Pubkey>,
                position: Option<solana_program::pubkey::Pubkey>,
                position_request: Option<solana_program::pubkey::Pubkey>,
                position_request_ata: Option<solana_program::pubkey::Pubkey>,
                custody: Option<solana_program::pubkey::Pubkey>,
                collateral_custody: Option<solana_program::pubkey::Pubkey>,
                input_mint: Option<solana_program::pubkey::Pubkey>,
                referral: Option<solana_program::pubkey::Pubkey>,
                token_program: Option<solana_program::pubkey::Pubkey>,
                associated_token_program: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                event_authority: Option<solana_program::pubkey::Pubkey>,
                program: Option<solana_program::pubkey::Pubkey>,
                        params: Option<CreateIncreasePositionMarketRequestParams>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateIncreasePositionMarketRequestBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.owner = Some(owner);
                    self
    }
            #[inline(always)]
    pub fn funding_account(&mut self, funding_account: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.funding_account = Some(funding_account);
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
    pub fn position_request(&mut self, position_request: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.position_request = Some(position_request);
                    self
    }
            #[inline(always)]
    pub fn position_request_ata(&mut self, position_request_ata: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.position_request_ata = Some(position_request_ata);
                    self
    }
            #[inline(always)]
    pub fn custody(&mut self, custody: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.custody = Some(custody);
                    self
    }
            #[inline(always)]
    pub fn collateral_custody(&mut self, collateral_custody: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.collateral_custody = Some(collateral_custody);
                    self
    }
            #[inline(always)]
    pub fn input_mint(&mut self, input_mint: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.input_mint = Some(input_mint);
                    self
    }
            /// `[optional account]`
#[inline(always)]
    pub fn referral(&mut self, referral: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
                        self.referral = referral;
                    self
    }
            #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_program = Some(token_program);
                    self
    }
            #[inline(always)]
    pub fn associated_token_program(&mut self, associated_token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.associated_token_program = Some(associated_token_program);
                    self
    }
            #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
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
      pub fn params(&mut self, params: CreateIncreasePositionMarketRequestParams) -> &mut Self {
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
    let accounts = CreateIncreasePositionMarketRequest {
                              owner: self.owner.expect("owner is not set"),
                                        funding_account: self.funding_account.expect("funding_account is not set"),
                                        perpetuals: self.perpetuals.expect("perpetuals is not set"),
                                        pool: self.pool.expect("pool is not set"),
                                        position: self.position.expect("position is not set"),
                                        position_request: self.position_request.expect("position_request is not set"),
                                        position_request_ata: self.position_request_ata.expect("position_request_ata is not set"),
                                        custody: self.custody.expect("custody is not set"),
                                        collateral_custody: self.collateral_custody.expect("collateral_custody is not set"),
                                        input_mint: self.input_mint.expect("input_mint is not set"),
                                        referral: self.referral,
                                        token_program: self.token_program.expect("token_program is not set"),
                                        associated_token_program: self.associated_token_program.expect("associated_token_program is not set"),
                                        system_program: self.system_program.expect("system_program is not set"),
                                        event_authority: self.event_authority.expect("event_authority is not set"),
                                        program: self.program.expect("program is not set"),
                      };
          let args = CreateIncreasePositionMarketRequestInstructionArgs {
                                                              params: self.params.clone().expect("params is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `create_increase_position_market_request` CPI accounts.
  pub struct CreateIncreasePositionMarketRequestCpiAccounts<'a, 'b> {
          
                    
              pub owner: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub funding_account: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub pool: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub position: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub position_request: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub position_request_ata: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub custody: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub collateral_custody: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub input_mint: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub referral: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                
                    
              pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `create_increase_position_market_request` CPI instruction.
pub struct CreateIncreasePositionMarketRequestCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub owner: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub funding_account: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub pool: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub position: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub position_request: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub position_request_ata: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub custody: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub collateral_custody: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub input_mint: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub referral: Option<&'b solana_program::account_info::AccountInfo<'a>>,
          
              
          pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub program: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: CreateIncreasePositionMarketRequestInstructionArgs,
  }

impl<'a, 'b> CreateIncreasePositionMarketRequestCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: CreateIncreasePositionMarketRequestCpiAccounts<'a, 'b>,
              args: CreateIncreasePositionMarketRequestInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              owner: accounts.owner,
              funding_account: accounts.funding_account,
              perpetuals: accounts.perpetuals,
              pool: accounts.pool,
              position: accounts.position,
              position_request: accounts.position_request,
              position_request_ata: accounts.position_request_ata,
              custody: accounts.custody,
              collateral_custody: accounts.collateral_custody,
              input_mint: accounts.input_mint,
              referral: accounts.referral,
              token_program: accounts.token_program,
              associated_token_program: accounts.associated_token_program,
              system_program: accounts.system_program,
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
    let mut accounts = Vec::with_capacity(16 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.owner.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funding_account.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.perpetuals.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.pool.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_request.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_request_ata.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.custody.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collateral_custody.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.input_mint.key,
            false
          ));
                                          if let Some(referral) = self.referral {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
              *referral.key,
              false,
            ));
          } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
              crate::PERPETUALS_ID,
              false,
            ));
          }
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false
          ));
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
    let mut data = CreateIncreasePositionMarketRequestInstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::PERPETUALS_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(16 + 1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.owner.clone());
                        account_infos.push(self.funding_account.clone());
                        account_infos.push(self.perpetuals.clone());
                        account_infos.push(self.pool.clone());
                        account_infos.push(self.position.clone());
                        account_infos.push(self.position_request.clone());
                        account_infos.push(self.position_request_ata.clone());
                        account_infos.push(self.custody.clone());
                        account_infos.push(self.collateral_custody.clone());
                        account_infos.push(self.input_mint.clone());
                        if let Some(referral) = self.referral {
          account_infos.push(referral.clone());
        }
                        account_infos.push(self.token_program.clone());
                        account_infos.push(self.associated_token_program.clone());
                        account_infos.push(self.system_program.clone());
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

/// Instruction builder for `CreateIncreasePositionMarketRequest` via CPI.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` owner
                ///   1. `[writable]` funding_account
          ///   2. `[]` perpetuals
          ///   3. `[]` pool
                ///   4. `[writable]` position
                ///   5. `[writable]` position_request
                ///   6. `[writable]` position_request_ata
          ///   7. `[]` custody
          ///   8. `[]` collateral_custody
          ///   9. `[]` input_mint
                ///   10. `[optional]` referral
          ///   11. `[]` token_program
          ///   12. `[]` associated_token_program
          ///   13. `[]` system_program
          ///   14. `[]` event_authority
          ///   15. `[]` program
#[derive(Clone, Debug)]
pub struct CreateIncreasePositionMarketRequestCpiBuilder<'a, 'b> {
  instruction: Box<CreateIncreasePositionMarketRequestCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateIncreasePositionMarketRequestCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(CreateIncreasePositionMarketRequestCpiBuilderInstruction {
      __program: program,
              owner: None,
              funding_account: None,
              perpetuals: None,
              pool: None,
              position: None,
              position_request: None,
              position_request_ata: None,
              custody: None,
              collateral_custody: None,
              input_mint: None,
              referral: None,
              token_program: None,
              associated_token_program: None,
              system_program: None,
              event_authority: None,
              program: None,
                                            params: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.owner = Some(owner);
                    self
    }
      #[inline(always)]
    pub fn funding_account(&mut self, funding_account: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.funding_account = Some(funding_account);
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
    pub fn position_request(&mut self, position_request: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.position_request = Some(position_request);
                    self
    }
      #[inline(always)]
    pub fn position_request_ata(&mut self, position_request_ata: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.position_request_ata = Some(position_request_ata);
                    self
    }
      #[inline(always)]
    pub fn custody(&mut self, custody: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.custody = Some(custody);
                    self
    }
      #[inline(always)]
    pub fn collateral_custody(&mut self, collateral_custody: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.collateral_custody = Some(collateral_custody);
                    self
    }
      #[inline(always)]
    pub fn input_mint(&mut self, input_mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.input_mint = Some(input_mint);
                    self
    }
      /// `[optional account]`
#[inline(always)]
    pub fn referral(&mut self, referral: Option<&'b solana_program::account_info::AccountInfo<'a>>) -> &mut Self {
                        self.instruction.referral = referral;
                    self
    }
      #[inline(always)]
    pub fn token_program(&mut self, token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_program = Some(token_program);
                    self
    }
      #[inline(always)]
    pub fn associated_token_program(&mut self, associated_token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.associated_token_program = Some(associated_token_program);
                    self
    }
      #[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
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
      pub fn params(&mut self, params: CreateIncreasePositionMarketRequestParams) -> &mut Self {
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
          let args = CreateIncreasePositionMarketRequestInstructionArgs {
                                                              params: self.instruction.params.clone().expect("params is not set"),
                                    };
        let instruction = CreateIncreasePositionMarketRequestCpi {
        __program: self.instruction.__program,
                  
          owner: self.instruction.owner.expect("owner is not set"),
                  
          funding_account: self.instruction.funding_account.expect("funding_account is not set"),
                  
          perpetuals: self.instruction.perpetuals.expect("perpetuals is not set"),
                  
          pool: self.instruction.pool.expect("pool is not set"),
                  
          position: self.instruction.position.expect("position is not set"),
                  
          position_request: self.instruction.position_request.expect("position_request is not set"),
                  
          position_request_ata: self.instruction.position_request_ata.expect("position_request_ata is not set"),
                  
          custody: self.instruction.custody.expect("custody is not set"),
                  
          collateral_custody: self.instruction.collateral_custody.expect("collateral_custody is not set"),
                  
          input_mint: self.instruction.input_mint.expect("input_mint is not set"),
                  
          referral: self.instruction.referral,
                  
          token_program: self.instruction.token_program.expect("token_program is not set"),
                  
          associated_token_program: self.instruction.associated_token_program.expect("associated_token_program is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                  
          event_authority: self.instruction.event_authority.expect("event_authority is not set"),
                  
          program: self.instruction.program.expect("program is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct CreateIncreasePositionMarketRequestCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                funding_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                perpetuals: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                position_request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                position_request_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                custody: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                collateral_custody: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                input_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                referral: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        params: Option<CreateIncreasePositionMarketRequestParams>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

