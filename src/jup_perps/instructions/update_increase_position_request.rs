//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::jup_perps::types::UpdateIncreasePositionRequestParams;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use crate::jup_perps::programs::PERPETUALS_ID;

/// Accounts.
pub struct UpdateIncreasePositionRequest {
    pub owner: solana_program::pubkey::Pubkey,

    pub perpetuals: solana_program::pubkey::Pubkey,

    pub pool: solana_program::pubkey::Pubkey,

    pub position: solana_program::pubkey::Pubkey,

    pub position_request: solana_program::pubkey::Pubkey,

    pub custody: solana_program::pubkey::Pubkey,

    pub custody_oracle_account: solana_program::pubkey::Pubkey,
}

impl UpdateIncreasePositionRequest {
    pub fn instruction(
        &self,
        args: UpdateIncreasePositionRequestInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UpdateIncreasePositionRequestInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.owner, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.perpetuals,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.custody,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.custody_oracle_account,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = UpdateIncreasePositionRequestInstructionData::new()
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
pub struct UpdateIncreasePositionRequestInstructionData {
    discriminator: [u8; 8],
}

impl UpdateIncreasePositionRequestInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [100, 110, 83, 102, 86, 7, 105, 157],
        }
    }
}

impl Default for UpdateIncreasePositionRequestInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateIncreasePositionRequestInstructionArgs {
    pub params: UpdateIncreasePositionRequestParams,
}

/// Instruction builder for `UpdateIncreasePositionRequest`.
///
/// ### Accounts:
///
///   0. `[signer]` owner
///   1. `[]` perpetuals
///   2. `[]` pool
///   3. `[]` position
///   4. `[writable]` position_request
///   5. `[]` custody
///   6. `[]` custody_oracle_account
#[derive(Clone, Debug, Default)]
pub struct UpdateIncreasePositionRequestBuilder {
    owner: Option<solana_program::pubkey::Pubkey>,
    perpetuals: Option<solana_program::pubkey::Pubkey>,
    pool: Option<solana_program::pubkey::Pubkey>,
    position: Option<solana_program::pubkey::Pubkey>,
    position_request: Option<solana_program::pubkey::Pubkey>,
    custody: Option<solana_program::pubkey::Pubkey>,
    custody_oracle_account: Option<solana_program::pubkey::Pubkey>,
    params: Option<UpdateIncreasePositionRequestParams>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdateIncreasePositionRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
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
    pub fn params(&mut self, params: UpdateIncreasePositionRequestParams) -> &mut Self {
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
        let accounts = UpdateIncreasePositionRequest {
            owner: self.owner.expect("owner is not set"),
            perpetuals: self.perpetuals.expect("perpetuals is not set"),
            pool: self.pool.expect("pool is not set"),
            position: self.position.expect("position is not set"),
            position_request: self.position_request.expect("position_request is not set"),
            custody: self.custody.expect("custody is not set"),
            custody_oracle_account: self
                .custody_oracle_account
                .expect("custody_oracle_account is not set"),
        };
        let args = UpdateIncreasePositionRequestInstructionArgs {
            params: self.params.clone().expect("params is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `update_increase_position_request` CPI accounts.
pub struct UpdateIncreasePositionRequestCpiAccounts<'a, 'b> {
    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_request: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `update_increase_position_request` CPI instruction.
pub struct UpdateIncreasePositionRequestCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_request: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: UpdateIncreasePositionRequestInstructionArgs,
}

impl<'a, 'b> UpdateIncreasePositionRequestCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdateIncreasePositionRequestCpiAccounts<'a, 'b>,
        args: UpdateIncreasePositionRequestInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            owner: accounts.owner,
            perpetuals: accounts.perpetuals,
            pool: accounts.pool,
            position: accounts.position,
            position_request: accounts.position_request,
            custody: accounts.custody,
            custody_oracle_account: accounts.custody_oracle_account,
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
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.owner.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.perpetuals.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.custody.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.custody_oracle_account.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = UpdateIncreasePositionRequestInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: PERPETUALS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.perpetuals.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.position.clone());
        account_infos.push(self.position_request.clone());
        account_infos.push(self.custody.clone());
        account_infos.push(self.custody_oracle_account.clone());
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

/// Instruction builder for `UpdateIncreasePositionRequest` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` owner
///   1. `[]` perpetuals
///   2. `[]` pool
///   3. `[]` position
///   4. `[writable]` position_request
///   5. `[]` custody
///   6. `[]` custody_oracle_account
#[derive(Clone, Debug)]
pub struct UpdateIncreasePositionRequestCpiBuilder<'a, 'b> {
    instruction: Box<UpdateIncreasePositionRequestCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateIncreasePositionRequestCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdateIncreasePositionRequestCpiBuilderInstruction {
            __program: program,
            owner: None,
            perpetuals: None,
            pool: None,
            position: None,
            position_request: None,
            custody: None,
            custody_oracle_account: None,
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
    pub fn params(&mut self, params: UpdateIncreasePositionRequestParams) -> &mut Self {
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
        let args = UpdateIncreasePositionRequestInstructionArgs {
            params: self.instruction.params.clone().expect("params is not set"),
        };
        let instruction = UpdateIncreasePositionRequestCpi {
            __program: self.instruction.__program,

            owner: self.instruction.owner.expect("owner is not set"),

            perpetuals: self.instruction.perpetuals.expect("perpetuals is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            position: self.instruction.position.expect("position is not set"),

            position_request: self
                .instruction
                .position_request
                .expect("position_request is not set"),

            custody: self.instruction.custody.expect("custody is not set"),

            custody_oracle_account: self
                .instruction
                .custody_oracle_account
                .expect("custody_oracle_account is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UpdateIncreasePositionRequestCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    perpetuals: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    custody: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    custody_oracle_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    params: Option<UpdateIncreasePositionRequestParams>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
