//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::jup_perps::types::SetTestTimeParams;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use crate::jup_perps::programs::PERPETUALS_ID;

/// Accounts.
pub struct SetTestTime {
    pub admin: solana_program::pubkey::Pubkey,

    pub perpetuals: solana_program::pubkey::Pubkey,
}

impl SetTestTime {
    pub fn instruction(
        &self,
        args: SetTestTimeInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: SetTestTimeInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.admin, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.perpetuals,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = SetTestTimeInstructionData::new().try_to_vec().unwrap();
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
pub struct SetTestTimeInstructionData {
    discriminator: [u8; 8],
}

impl SetTestTimeInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [242, 231, 177, 251, 126, 145, 159, 104],
        }
    }
}

impl Default for SetTestTimeInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetTestTimeInstructionArgs {
    pub params: SetTestTimeParams,
}

/// Instruction builder for `SetTestTime`.
///
/// ### Accounts:
///
///   0. `[signer]` admin
///   1. `[writable]` perpetuals
#[derive(Clone, Debug, Default)]
pub struct SetTestTimeBuilder {
    admin: Option<solana_program::pubkey::Pubkey>,
    perpetuals: Option<solana_program::pubkey::Pubkey>,
    params: Option<SetTestTimeParams>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SetTestTimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn perpetuals(&mut self, perpetuals: solana_program::pubkey::Pubkey) -> &mut Self {
        self.perpetuals = Some(perpetuals);
        self
    }
    #[inline(always)]
    pub fn params(&mut self, params: SetTestTimeParams) -> &mut Self {
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
        let accounts = SetTestTime {
            admin: self.admin.expect("admin is not set"),
            perpetuals: self.perpetuals.expect("perpetuals is not set"),
        };
        let args = SetTestTimeInstructionArgs {
            params: self.params.clone().expect("params is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `set_test_time` CPI accounts.
pub struct SetTestTimeCpiAccounts<'a, 'b> {
    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `set_test_time` CPI instruction.
pub struct SetTestTimeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: SetTestTimeInstructionArgs,
}

impl<'a, 'b> SetTestTimeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SetTestTimeCpiAccounts<'a, 'b>,
        args: SetTestTimeInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            admin: accounts.admin,
            perpetuals: accounts.perpetuals,
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
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.perpetuals.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = SetTestTimeInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: PERPETUALS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.perpetuals.clone());
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

/// Instruction builder for `SetTestTime` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` admin
///   1. `[writable]` perpetuals
#[derive(Clone, Debug)]
pub struct SetTestTimeCpiBuilder<'a, 'b> {
    instruction: Box<SetTestTimeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SetTestTimeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SetTestTimeCpiBuilderInstruction {
            __program: program,
            admin: None,
            perpetuals: None,
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
    pub fn perpetuals(
        &mut self,
        perpetuals: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.perpetuals = Some(perpetuals);
        self
    }
    #[inline(always)]
    pub fn params(&mut self, params: SetTestTimeParams) -> &mut Self {
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
        let args = SetTestTimeInstructionArgs {
            params: self.instruction.params.clone().expect("params is not set"),
        };
        let instruction = SetTestTimeCpi {
            __program: self.instruction.__program,

            admin: self.instruction.admin.expect("admin is not set"),

            perpetuals: self.instruction.perpetuals.expect("perpetuals is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct SetTestTimeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    perpetuals: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    params: Option<SetTestTimeParams>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
