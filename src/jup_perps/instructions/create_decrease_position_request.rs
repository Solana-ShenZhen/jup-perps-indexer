//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::jup_perps::types::CreateDecreasePositionRequestParams;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use crate::jup_perps::programs::PERPETUALS_ID;

/// Accounts.
pub struct CreateDecreasePositionRequest {
    pub owner: solana_program::pubkey::Pubkey,

    pub receiving_account: solana_program::pubkey::Pubkey,

    pub perpetuals: solana_program::pubkey::Pubkey,

    pub pool: solana_program::pubkey::Pubkey,

    pub position: solana_program::pubkey::Pubkey,

    pub position_request: solana_program::pubkey::Pubkey,

    pub position_request_ata: solana_program::pubkey::Pubkey,

    pub custody: solana_program::pubkey::Pubkey,

    pub custody_oracle_account: solana_program::pubkey::Pubkey,

    pub collateral_custody: solana_program::pubkey::Pubkey,

    pub desired_mint: solana_program::pubkey::Pubkey,

    pub referral: Option<solana_program::pubkey::Pubkey>,

    pub token_program: solana_program::pubkey::Pubkey,

    pub associated_token_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl CreateDecreasePositionRequest {
    pub fn instruction(
        &self,
        args: CreateDecreasePositionRequestInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateDecreasePositionRequestInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(17 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.owner, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.receiving_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.perpetuals,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.position,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_request,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_request_ata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.custody,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.custody_oracle_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collateral_custody,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.desired_mint,
            false,
        ));
        if let Some(referral) = self.referral {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                referral, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                PERPETUALS_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateDecreasePositionRequestInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: PERPETUALS_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateDecreasePositionRequestInstructionData {
    discriminator: [u8; 8],
}

impl CreateDecreasePositionRequestInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [146, 21, 51, 121, 187, 208, 7, 69],
        }
    }
}

impl Default for CreateDecreasePositionRequestInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateDecreasePositionRequestInstructionArgs {
    pub params: CreateDecreasePositionRequestParams,
}

/// Instruction builder for `CreateDecreasePositionRequest`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` owner
///   1. `[writable]` receiving_account
///   2. `[]` perpetuals
///   3. `[writable]` pool
///   4. `[]` position
///   5. `[writable]` position_request
///   6. `[writable]` position_request_ata
///   7. `[]` custody
///   8. `[]` custody_oracle_account
///   9. `[]` collateral_custody
///   10. `[]` desired_mint
///   11. `[optional]` referral
///   12. `[]` token_program
///   13. `[]` associated_token_program
///   14. `[]` system_program
///   15. `[]` event_authority
///   16. `[]` program
#[derive(Clone, Debug, Default)]
pub struct CreateDecreasePositionRequestBuilder {
    owner: Option<solana_program::pubkey::Pubkey>,
    receiving_account: Option<solana_program::pubkey::Pubkey>,
    perpetuals: Option<solana_program::pubkey::Pubkey>,
    pool: Option<solana_program::pubkey::Pubkey>,
    position: Option<solana_program::pubkey::Pubkey>,
    position_request: Option<solana_program::pubkey::Pubkey>,
    position_request_ata: Option<solana_program::pubkey::Pubkey>,
    custody: Option<solana_program::pubkey::Pubkey>,
    custody_oracle_account: Option<solana_program::pubkey::Pubkey>,
    collateral_custody: Option<solana_program::pubkey::Pubkey>,
    desired_mint: Option<solana_program::pubkey::Pubkey>,
    referral: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    params: Option<CreateDecreasePositionRequestParams>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateDecreasePositionRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }
    #[inline(always)]
    pub fn receiving_account(
        &mut self,
        receiving_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.receiving_account = Some(receiving_account);
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
    pub fn position_request(
        &mut self,
        position_request: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.position_request = Some(position_request);
        self
    }
    #[inline(always)]
    pub fn position_request_ata(
        &mut self,
        position_request_ata: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.position_request_ata = Some(position_request_ata);
        self
    }
    #[inline(always)]
    pub fn custody(&mut self, custody: solana_program::pubkey::Pubkey) -> &mut Self {
        self.custody = Some(custody);
        self
    }
    #[inline(always)]
    pub fn custody_oracle_account(
        &mut self,
        custody_oracle_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.custody_oracle_account = Some(custody_oracle_account);
        self
    }
    #[inline(always)]
    pub fn collateral_custody(
        &mut self,
        collateral_custody: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collateral_custody = Some(collateral_custody);
        self
    }
    #[inline(always)]
    pub fn desired_mint(&mut self, desired_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.desired_mint = Some(desired_mint);
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
    pub fn associated_token_program(
        &mut self,
        associated_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_token_program = Some(associated_token_program);
        self
    }
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(&mut self, program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
        self
    }
    #[inline(always)]
    pub fn params(&mut self, params: CreateDecreasePositionRequestParams) -> &mut Self {
        self.params = Some(params);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = CreateDecreasePositionRequest {
            owner: self.owner.expect("owner is not set"),
            receiving_account: self
                .receiving_account
                .expect("receiving_account is not set"),
            perpetuals: self.perpetuals.expect("perpetuals is not set"),
            pool: self.pool.expect("pool is not set"),
            position: self.position.expect("position is not set"),
            position_request: self.position_request.expect("position_request is not set"),
            position_request_ata: self
                .position_request_ata
                .expect("position_request_ata is not set"),
            custody: self.custody.expect("custody is not set"),
            custody_oracle_account: self
                .custody_oracle_account
                .expect("custody_oracle_account is not set"),
            collateral_custody: self
                .collateral_custody
                .expect("collateral_custody is not set"),
            desired_mint: self.desired_mint.expect("desired_mint is not set"),
            referral: self.referral,
            token_program: self.token_program.expect("token_program is not set"),
            associated_token_program: self
                .associated_token_program
                .expect("associated_token_program is not set"),
            system_program: self.system_program.expect("system_program is not set"),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = CreateDecreasePositionRequestInstructionArgs {
            params: self.params.clone().expect("params is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_decrease_position_request` CPI accounts.
pub struct CreateDecreasePositionRequestCpiAccounts<'a, 'b> {
    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub receiving_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_request: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_request_ata: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub collateral_custody: &'b solana_program::account_info::AccountInfo<'a>,

    pub desired_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_decrease_position_request` CPI instruction.
pub struct CreateDecreasePositionRequestCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub receiving_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_request: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_request_ata: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub collateral_custody: &'b solana_program::account_info::AccountInfo<'a>,

    pub desired_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub referral: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateDecreasePositionRequestInstructionArgs,
}

impl<'a, 'b> CreateDecreasePositionRequestCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateDecreasePositionRequestCpiAccounts<'a, 'b>,
        args: CreateDecreasePositionRequestInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            owner: accounts.owner,
            receiving_account: accounts.receiving_account,
            perpetuals: accounts.perpetuals,
            pool: accounts.pool,
            position: accounts.position,
            position_request: accounts.position_request,
            position_request_ata: accounts.position_request_ata,
            custody: accounts.custody,
            custody_oracle_account: accounts.custody_oracle_account,
            collateral_custody: accounts.collateral_custody,
            desired_mint: accounts.desired_mint,
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
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(17 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.owner.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.receiving_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.perpetuals.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.position.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_request.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_request_ata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.custody.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.custody_oracle_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collateral_custody.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.desired_mint.key,
            false,
        ));
        if let Some(referral) = self.referral {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *referral.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                PERPETUALS_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateDecreasePositionRequestInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: PERPETUALS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(17 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.receiving_account.clone());
        account_infos.push(self.perpetuals.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.position.clone());
        account_infos.push(self.position_request.clone());
        account_infos.push(self.position_request_ata.clone());
        account_infos.push(self.custody.clone());
        account_infos.push(self.custody_oracle_account.clone());
        account_infos.push(self.collateral_custody.clone());
        account_infos.push(self.desired_mint.clone());
        if let Some(referral) = self.referral {
            account_infos.push(referral.clone());
        }
        account_infos.push(self.token_program.clone());
        account_infos.push(self.associated_token_program.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.event_authority.clone());
        account_infos.push(self.program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `CreateDecreasePositionRequest` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` owner
///   1. `[writable]` receiving_account
///   2. `[]` perpetuals
///   3. `[writable]` pool
///   4. `[]` position
///   5. `[writable]` position_request
///   6. `[writable]` position_request_ata
///   7. `[]` custody
///   8. `[]` custody_oracle_account
///   9. `[]` collateral_custody
///   10. `[]` desired_mint
///   11. `[optional]` referral
///   12. `[]` token_program
///   13. `[]` associated_token_program
///   14. `[]` system_program
///   15. `[]` event_authority
///   16. `[]` program
#[derive(Clone, Debug)]
pub struct CreateDecreasePositionRequestCpiBuilder<'a, 'b> {
    instruction: Box<CreateDecreasePositionRequestCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateDecreasePositionRequestCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateDecreasePositionRequestCpiBuilderInstruction {
            __program: program,
            owner: None,
            receiving_account: None,
            perpetuals: None,
            pool: None,
            position: None,
            position_request: None,
            position_request_ata: None,
            custody: None,
            custody_oracle_account: None,
            collateral_custody: None,
            desired_mint: None,
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
    pub fn receiving_account(
        &mut self,
        receiving_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.receiving_account = Some(receiving_account);
        self
    }
    #[inline(always)]
    pub fn perpetuals(
        &mut self,
        perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.perpetuals = Some(perpetuals);
        self
    }
    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
        self
    }
    #[inline(always)]
    pub fn position(
        &mut self,
        position: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position = Some(position);
        self
    }
    #[inline(always)]
    pub fn position_request(
        &mut self,
        position_request: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_request = Some(position_request);
        self
    }
    #[inline(always)]
    pub fn position_request_ata(
        &mut self,
        position_request_ata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_request_ata = Some(position_request_ata);
        self
    }
    #[inline(always)]
    pub fn custody(
        &mut self,
        custody: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.custody = Some(custody);
        self
    }
    #[inline(always)]
    pub fn custody_oracle_account(
        &mut self,
        custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.custody_oracle_account = Some(custody_oracle_account);
        self
    }
    #[inline(always)]
    pub fn collateral_custody(
        &mut self,
        collateral_custody: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collateral_custody = Some(collateral_custody);
        self
    }
    #[inline(always)]
    pub fn desired_mint(
        &mut self,
        desired_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.desired_mint = Some(desired_mint);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn referral(
        &mut self,
        referral: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.referral = referral;
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.associated_token_program = Some(associated_token_program);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(
        &mut self,
        program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program = Some(program);
        self
    }
    #[inline(always)]
    pub fn params(&mut self, params: CreateDecreasePositionRequestParams) -> &mut Self {
        self.instruction.params = Some(params);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = CreateDecreasePositionRequestInstructionArgs {
            params: self.instruction.params.clone().expect("params is not set"),
        };
        let instruction = CreateDecreasePositionRequestCpi {
            __program: self.instruction.__program,

            owner: self.instruction.owner.expect("owner is not set"),

            receiving_account: self
                .instruction
                .receiving_account
                .expect("receiving_account is not set"),

            perpetuals: self.instruction.perpetuals.expect("perpetuals is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            position: self.instruction.position.expect("position is not set"),

            position_request: self
                .instruction
                .position_request
                .expect("position_request is not set"),

            position_request_ata: self
                .instruction
                .position_request_ata
                .expect("position_request_ata is not set"),

            custody: self.instruction.custody.expect("custody is not set"),

            custody_oracle_account: self
                .instruction
                .custody_oracle_account
                .expect("custody_oracle_account is not set"),

            collateral_custody: self
                .instruction
                .collateral_custody
                .expect("collateral_custody is not set"),

            desired_mint: self
                .instruction
                .desired_mint
                .expect("desired_mint is not set"),

            referral: self.instruction.referral,

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            associated_token_program: self
                .instruction
                .associated_token_program
                .expect("associated_token_program is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            event_authority: self
                .instruction
                .event_authority
                .expect("event_authority is not set"),

            program: self.instruction.program.expect("program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateDecreasePositionRequestCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    receiving_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    perpetuals: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_request_ata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    custody: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    custody_oracle_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collateral_custody: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    desired_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    referral: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    params: Option<CreateDecreasePositionRequestParams>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
