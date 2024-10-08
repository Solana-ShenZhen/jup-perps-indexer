//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::jup_perps::types::RemoveLiquidityParams;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use crate::jup_perps::programs::PERPETUALS_ID;

/// Accounts.
pub struct RemoveLiquidity {
    pub owner: solana_program::pubkey::Pubkey,

    pub receiving_account: solana_program::pubkey::Pubkey,

    pub lp_token_account: solana_program::pubkey::Pubkey,

    pub transfer_authority: solana_program::pubkey::Pubkey,

    pub perpetuals: solana_program::pubkey::Pubkey,

    pub pool: solana_program::pubkey::Pubkey,

    pub custody: solana_program::pubkey::Pubkey,

    pub custody_oracle_account: solana_program::pubkey::Pubkey,

    pub custody_token_account: solana_program::pubkey::Pubkey,

    pub lp_token_mint: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl RemoveLiquidity {
    pub fn instruction(
        &self,
        args: RemoveLiquidityInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: RemoveLiquidityInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.owner, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.receiving_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lp_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.transfer_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.perpetuals,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.pool, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.custody,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.custody_oracle_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.custody_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lp_token_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
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
        let mut data = RemoveLiquidityInstructionData::new().try_to_vec().unwrap();
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
pub struct RemoveLiquidityInstructionData {
    discriminator: [u8; 8],
}

impl RemoveLiquidityInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [80, 85, 209, 72, 24, 206, 177, 108],
        }
    }
}

impl Default for RemoveLiquidityInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquidityInstructionArgs {
    pub params: RemoveLiquidityParams,
}

/// Instruction builder for `RemoveLiquidity`.
///
/// ### Accounts:
///
///   0. `[signer]` owner
///   1. `[writable]` receiving_account
///   2. `[writable]` lp_token_account
///   3. `[]` transfer_authority
///   4. `[]` perpetuals
///   5. `[writable]` pool
///   6. `[writable]` custody
///   7. `[]` custody_oracle_account
///   8. `[writable]` custody_token_account
///   9. `[writable]` lp_token_mint
///   10. `[]` token_program
///   11. `[]` event_authority
///   12. `[]` program
#[derive(Clone, Debug, Default)]
pub struct RemoveLiquidityBuilder {
    owner: Option<solana_program::pubkey::Pubkey>,
    receiving_account: Option<solana_program::pubkey::Pubkey>,
    lp_token_account: Option<solana_program::pubkey::Pubkey>,
    transfer_authority: Option<solana_program::pubkey::Pubkey>,
    perpetuals: Option<solana_program::pubkey::Pubkey>,
    pool: Option<solana_program::pubkey::Pubkey>,
    custody: Option<solana_program::pubkey::Pubkey>,
    custody_oracle_account: Option<solana_program::pubkey::Pubkey>,
    custody_token_account: Option<solana_program::pubkey::Pubkey>,
    lp_token_mint: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    params: Option<RemoveLiquidityParams>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl RemoveLiquidityBuilder {
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
    pub fn lp_token_account(
        &mut self,
        lp_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.lp_token_account = Some(lp_token_account);
        self
    }
    #[inline(always)]
    pub fn transfer_authority(
        &mut self,
        transfer_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
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
    pub fn custody_token_account(
        &mut self,
        custody_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.custody_token_account = Some(custody_token_account);
        self
    }
    #[inline(always)]
    pub fn lp_token_mint(&mut self, lp_token_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lp_token_mint = Some(lp_token_mint);
        self
    }
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
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
    pub fn params(&mut self, params: RemoveLiquidityParams) -> &mut Self {
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
        let accounts = RemoveLiquidity {
            owner: self.owner.expect("owner is not set"),
            receiving_account: self
                .receiving_account
                .expect("receiving_account is not set"),
            lp_token_account: self.lp_token_account.expect("lp_token_account is not set"),
            transfer_authority: self
                .transfer_authority
                .expect("transfer_authority is not set"),
            perpetuals: self.perpetuals.expect("perpetuals is not set"),
            pool: self.pool.expect("pool is not set"),
            custody: self.custody.expect("custody is not set"),
            custody_oracle_account: self
                .custody_oracle_account
                .expect("custody_oracle_account is not set"),
            custody_token_account: self
                .custody_token_account
                .expect("custody_token_account is not set"),
            lp_token_mint: self.lp_token_mint.expect("lp_token_mint is not set"),
            token_program: self.token_program.expect("token_program is not set"),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = RemoveLiquidityInstructionArgs {
            params: self.params.clone().expect("params is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `remove_liquidity` CPI accounts.
pub struct RemoveLiquidityCpiAccounts<'a, 'b> {
    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub receiving_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub lp_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `remove_liquidity` CPI instruction.
pub struct RemoveLiquidityCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub receiving_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub lp_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub perpetuals: &'b solana_program::account_info::AccountInfo<'a>,

    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_oracle_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub custody_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: RemoveLiquidityInstructionArgs,
}

impl<'a, 'b> RemoveLiquidityCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: RemoveLiquidityCpiAccounts<'a, 'b>,
        args: RemoveLiquidityInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            owner: accounts.owner,
            receiving_account: accounts.receiving_account,
            lp_token_account: accounts.lp_token_account,
            transfer_authority: accounts.transfer_authority,
            perpetuals: accounts.perpetuals,
            pool: accounts.pool,
            custody: accounts.custody,
            custody_oracle_account: accounts.custody_oracle_account,
            custody_token_account: accounts.custody_token_account,
            lp_token_mint: accounts.lp_token_mint,
            token_program: accounts.token_program,
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
        let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.owner.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.receiving_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lp_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.transfer_authority.key,
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
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.custody.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.custody_oracle_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.custody_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lp_token_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
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
        let mut data = RemoveLiquidityInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: PERPETUALS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(13 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.receiving_account.clone());
        account_infos.push(self.lp_token_account.clone());
        account_infos.push(self.transfer_authority.clone());
        account_infos.push(self.perpetuals.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.custody.clone());
        account_infos.push(self.custody_oracle_account.clone());
        account_infos.push(self.custody_token_account.clone());
        account_infos.push(self.lp_token_mint.clone());
        account_infos.push(self.token_program.clone());
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

/// Instruction builder for `RemoveLiquidity` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` owner
///   1. `[writable]` receiving_account
///   2. `[writable]` lp_token_account
///   3. `[]` transfer_authority
///   4. `[]` perpetuals
///   5. `[writable]` pool
///   6. `[writable]` custody
///   7. `[]` custody_oracle_account
///   8. `[writable]` custody_token_account
///   9. `[writable]` lp_token_mint
///   10. `[]` token_program
///   11. `[]` event_authority
///   12. `[]` program
#[derive(Clone, Debug)]
pub struct RemoveLiquidityCpiBuilder<'a, 'b> {
    instruction: Box<RemoveLiquidityCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RemoveLiquidityCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(RemoveLiquidityCpiBuilderInstruction {
            __program: program,
            owner: None,
            receiving_account: None,
            lp_token_account: None,
            transfer_authority: None,
            perpetuals: None,
            pool: None,
            custody: None,
            custody_oracle_account: None,
            custody_token_account: None,
            lp_token_mint: None,
            token_program: None,
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
    pub fn lp_token_account(
        &mut self,
        lp_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lp_token_account = Some(lp_token_account);
        self
    }
    #[inline(always)]
    pub fn transfer_authority(
        &mut self,
        transfer_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.transfer_authority = Some(transfer_authority);
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
    pub fn custody_token_account(
        &mut self,
        custody_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.custody_token_account = Some(custody_token_account);
        self
    }
    #[inline(always)]
    pub fn lp_token_mint(
        &mut self,
        lp_token_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lp_token_mint = Some(lp_token_mint);
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
    pub fn params(&mut self, params: RemoveLiquidityParams) -> &mut Self {
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
        let args = RemoveLiquidityInstructionArgs {
            params: self.instruction.params.clone().expect("params is not set"),
        };
        let instruction = RemoveLiquidityCpi {
            __program: self.instruction.__program,

            owner: self.instruction.owner.expect("owner is not set"),

            receiving_account: self
                .instruction
                .receiving_account
                .expect("receiving_account is not set"),

            lp_token_account: self
                .instruction
                .lp_token_account
                .expect("lp_token_account is not set"),

            transfer_authority: self
                .instruction
                .transfer_authority
                .expect("transfer_authority is not set"),

            perpetuals: self.instruction.perpetuals.expect("perpetuals is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            custody: self.instruction.custody.expect("custody is not set"),

            custody_oracle_account: self
                .instruction
                .custody_oracle_account
                .expect("custody_oracle_account is not set"),

            custody_token_account: self
                .instruction
                .custody_token_account
                .expect("custody_token_account is not set"),

            lp_token_mint: self
                .instruction
                .lp_token_mint
                .expect("lp_token_mint is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

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
struct RemoveLiquidityCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    receiving_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lp_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    transfer_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    perpetuals: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    custody: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    custody_oracle_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    custody_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lp_token_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    params: Option<RemoveLiquidityParams>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
